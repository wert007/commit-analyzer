use std::{collections::HashMap, error::Error, io::Write, ops::AddAssign};

fn main() -> Result<(), Box<dyn Error>> {
    let mut opts = getopts::Options::new();
    let opts = opts
        .optflag("", "git", "Grab the input data from the local Git history.")
        .optflag("h", "help", "Show this help and exit.")
        .optflag("v", "verbose", "Always show the entire output.")
        .optmulti(
            "a",
            "author-contains",
            "Filter for certain author names. ORs if specified multiple times.",
            "NAME",
        )
        .optmulti(
            "",
            "author-equals",
            "Filter for certain author names. ORs if specified multiple times.",
            "NAME",
        )
        .optmulti(
            "e",
            "email-contains",
            "Filter for certain author emails. ORs if specified multiple times.",
            "EMAIL",
        )
        .optmulti(
            "",
            "email-equals",
            "Filter for certain author emails. ORs if specified multiple times.",
            "EMAIL",
        )
        .optmulti(
            "c",
            "commit-contains",
            "Filter for certain commit hashes. ORs if specified multiple times.",
            "HASH",
        )
        .optmulti(
            "",
            "commit-equals",
            "Filter for certain commit hashes. ORs if specified multiple times.",
            "HASH",
        )
        .optmulti(
            "f",
            "file-extension",
            "Filter loc for certain file extension (e.g. `--file-extension cpp`). ORs if specified multiple times.",
            "EXTENSION",
        )
        .optmulti(
            "m",
            "message-contains",
            "Filter for certain commit messages. ORs if specified multiple times.",
            "MESSAGE",
        )
        .optmulti(
            "",
            "message-equals",
            "Filter for certain commit messages. ORs if specified multiple times.",
            "MESSAGE",
        )
        .optmulti(
            "l",
            "message-starts-with",
            "Filter for certain commit messages. ORs if specified multiple times.",
            "MESSAGE",
        )
        .optopt(
            "d",
            "duration",
            "The time which may pass between two commits that still counts as working.",
            "HOURS",
        )
        .optopt("i", "input", "The log file to read from.", "FILE")
        .optopt(
            "o",
            "output",
            "An output file for the commits per day in CSV format.",
            "FILE",
        );
    let matches = match opts.parse(std::env::args()) {
        Ok(it) => it,
        Err(err) => {
            commit_analyzer::usage(opts);
            return Err(err.into());
        }
    };
    if matches.opt_present("help") || (!matches.opt_present("git") && !matches.opt_present("input"))
    {
        commit_analyzer::usage(opts);
        return Ok(());
    }
    let is_verbose = matches.opt_present("verbose");
    let max_diff_hours: u32 = match matches.opt_str("duration").map(|str| str.parse()) {
        None => 3,
        Some(Ok(it)) => it,
        Some(Err(err)) => {
            eprintln!("duration must be an integer value!");
            return Err(err.into());
        }
    };
    let path = matches.opt_str("output");
    let commits = if matches.opt_present("git") {
        let process = std::process::Command::new("git")
            .arg("log")
            .arg("--numstat")
            .output()?;
        String::from_utf8(process.stdout)?
    } else if matches.opt_present("input") {
        let path = matches.opt_str("input").unwrap();
        std::fs::read_to_string(path)?
    } else {
        todo!("Read Git history from `stdin`.");
    };
    let mut commits = commits.as_str();
    let mut parsed_commits = vec![];
    while !commits.is_empty() {
        let result = commit_analyzer::Commit::parse(commits);
        match result {
            Ok((commit, remainder)) => {
                commits = remainder;
                parsed_commits.push(commit);
            }
            Err(err) => {
                dbg!(err);
                break;
            }
        }
    }
    let mut last_time: std::option::Option<chrono::DateTime<chrono::FixedOffset>> = None;
    let mut duration = chrono::Duration::zero();
    let filter = commit_analyzer::Filter::new(&matches);
    let mut commit_count = 0;
    let mut commits_per_day = HashMap::new();
    let mut loc_per_day = HashMap::new();
    for commit in parsed_commits.into_iter().rev() {
        if filter.matches(&commit) {
            commit_count += 1;
            commits_per_day
                .entry(commit.date().date())
                .or_insert(0)
                .add_assign(1);
            loc_per_day
                .entry(commit.date().date())
                .or_insert(0)
                .add_assign(commit.loc(&filter));
            if let Some(last_time) = last_time {
                let diff: chrono::Duration = *commit.date() - last_time;
                if diff.num_hours() <= max_diff_hours as i64 {
                    duration = duration + diff;
                }
            }
            last_time = Some(*commit.date());
            if is_verbose {
                println!("{:#?}", commit);
            }
        }
    }

    println!("Estimated time was {}h", duration.num_hours());
    println!("Found {} commits overall", commit_count);

    if let Some(path) = path {
        let mut file = std::fs::File::create(path)?;
        let mut sorted_per_day_data = vec![];
        for key in commits_per_day.keys() {
            let commit_count = commits_per_day[key];
            let loc = loc_per_day[key];
            sorted_per_day_data.push((*key, commit_count, loc));
        }
        sorted_per_day_data.sort_by_cached_key(|(k, _, _)| *k);
        writeln!(file, "Date, Commits, Loc")?;
        for (date, commits, loc) in sorted_per_day_data {
            writeln!(file, "{}, {}, {}", date, commits, loc)?;
        }
    }

    Ok(())
}
