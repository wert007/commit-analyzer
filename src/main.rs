use std::{collections::HashMap, io::Write, ops::AddAssign};

fn main() -> sysexits::ExitCode {
    let mut opts = getopts::Options::new();
    let opts = opts
        .optflag("", "git", "Grab the input data from the local Git history.")
        .optflag("h", "help", "Show this help and exit.")
        .optflag("", "stdin", "Read input data from `stdin`.")
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
        Err(getopts::Fail::ArgumentMissing(string)) => {
            eprintln!(
                "{}",
                opts.usage(&format!(
                    "There was an argument expected for option '{string}'."
                ))
            );
            return sysexits::ExitCode::Usage;
        }
        Err(getopts::Fail::UnrecognizedOption(string)) => {
            eprintln!("{}", opts.usage(&format!("Unknown option '{string}'.")));
            return sysexits::ExitCode::Usage;
        }
        Err(err) => {
            eprintln!("{}", opts.usage(&format!("{err}")));
            return sysexits::ExitCode::Config;
        }
    };
    let input = commit_analyzer::InputMethod::new(&matches);
    if matches.opt_present("help") {
        commit_analyzer::usage(opts);
        return sysexits::ExitCode::Ok;
    }
    if !input.specified() {
        eprintln!("{}", opts.usage("Please specify the input method."));
        return sysexits::ExitCode::Usage;
    }
    let is_verbose = matches.opt_present("verbose");
    let max_diff_hours: u32 = match matches.opt_str("duration").map(|str| str.parse()) {
        None => 3,
        Some(Ok(it)) => it,
        Some(Err(_)) => {
            eprintln!("duration must be an integer value!");
            return sysexits::ExitCode::Usage;
        }
    };
    let path = matches.opt_str("output");
    let commits = match input.read() {
        Some(string) => string,
        None => match input {
            commit_analyzer::InputMethod::GitHistory => {
                eprintln!("Reading from the Git history was not possible.");
                return sysexits::ExitCode::Unavailable;
            }
            commit_analyzer::InputMethod::LogFile(string) => {
                eprintln!("The input file '{string}' could not be read.");
                return sysexits::ExitCode::NoInput;
            }
            commit_analyzer::InputMethod::None => {
                eprintln!("The input method is invalid.");
                return sysexits::ExitCode::Usage;
            }
            commit_analyzer::InputMethod::Stdin => {
                eprintln!("Reading from `stdin` failed.");
                return sysexits::ExitCode::IoErr;
            }
        },
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
    let mut last_time = None;
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
        let mut file = match std::fs::File::create(path) {
            Ok(file) => file,
            Err(_) => return sysexits::ExitCode::CantCreat,
        };
        let mut sorted_per_day_data = vec![];
        for key in commits_per_day.keys() {
            let commit_count = commits_per_day[key];
            let loc = loc_per_day[key];
            sorted_per_day_data.push((*key, commit_count, loc));
        }
        sorted_per_day_data.sort_by_cached_key(|(k, _, _)| *k);
        match writeln!(file, "Date, Commits, Loc") {
            Ok(something) => something,
            Err(_) => return sysexits::ExitCode::IoErr,
        };
        for (date, commits, loc) in sorted_per_day_data {
            match writeln!(file, "{}, {}, {}", date, commits, loc) {
                Ok(something) => something,
                Err(_) => return sysexits::ExitCode::IoErr,
            };
        }
    }

    sysexits::ExitCode::Ok
}
