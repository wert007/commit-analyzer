use filter::Filter;
use loc::LocDiff;
use std::{collections::HashMap, error::Error, io::Write, ops::AddAssign};

mod commit;
mod filter;
mod loc;

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
            commit_analyzer::application::usage(opts);
            return Err(err.into());
        }
    };
    if matches.opt_present("help") || (!matches.opt_present("git") && !matches.opt_present("input"))
    {
        commit_analyzer::application::usage(opts);
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
        let result = parse_commit(commits);
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
    let filter = Filter::new(&matches);
    let mut commit_count = 0;
    let mut commits_per_day = HashMap::new();
    let mut loc_per_day = HashMap::new();
    for commit in parsed_commits.into_iter().rev() {
        if filter.matches(&commit) {
            commit_count += 1;
            commits_per_day
                .entry(commit.date.date())
                .or_insert(0)
                .add_assign(1);
            loc_per_day
                .entry(commit.date.date())
                .or_insert(0)
                .add_assign(commit.loc(&filter));
            if let Some(last_time) = last_time {
                let diff: chrono::Duration = commit.date - last_time;
                if diff.num_hours() <= max_diff_hours as i64 {
                    duration = duration + diff;
                }
            }
            last_time = Some(commit.date);
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

#[derive(Debug)]
enum CommitParseError {
    CommitMissing,
    AuthorMissing,
    DateMissing,
    AuthorFailed(commit_analyzer::author::AuthorParseError),
    DateFailed(chrono::ParseError),
    LocSyntaxError,
    LocFailed(loc::LocParseError),
    Unknown,
}

fn parse_commit(commit: &str) -> Result<(Commit, &str), CommitParseError> {
    let (commit, remainder) = commit
        .strip_prefix("commit")
        .ok_or(CommitParseError::CommitMissing)?
        .split_once('\n')
        .ok_or(CommitParseError::CommitMissing)?;
    let commit = commit.trim();
    let (merge, remainder) = if let Some(it) = remainder
        .strip_prefix("Merge: ")
        .map(|s| s.split_once('\n').ok_or(CommitParseError::Unknown))
    {
        let (merge, remainder) = it?;
        (Some(merge.trim().to_owned()), remainder)
    } else {
        (None, remainder)
    };
    let (author, remainder) = remainder
        .strip_prefix("Author: ")
        .ok_or(CommitParseError::AuthorMissing)?
        .split_once('\n')
        .ok_or(CommitParseError::AuthorMissing)?;
    let (date, mut remainder) = remainder
        .strip_prefix("Date:   ")
        .ok_or(CommitParseError::DateMissing)?
        .split_once('\n')
        .ok_or(CommitParseError::DateMissing)?;

    let mut message = String::new();
    let mut remainder_result = remainder;
    if let Some(it) = remainder.strip_prefix('\n') {
        remainder = it;
    }
    let mut space_count = 0;
    let mut increase_space_count = true;
    for (index, char) in remainder.char_indices() {
        if increase_space_count {
            if char == ' ' {
                space_count += 1;
            } else {
                increase_space_count = false;
            }
        } else if space_count == 4 && char == '\n' {
            increase_space_count = true;
            space_count = 0;
        } else if space_count < 4 {
            break;
        }
        // This removes the char from remainder, before adding it to the
        // message.
        remainder_result = &remainder[index + char.len_utf8()..];
        if !increase_space_count {
            message.push(char);
        }
    }
    let message = message.trim();

    let mut locs = vec![];
    loop {
        if remainder_result.is_empty() {
            break;
        }
        let (loc, remainder) = remainder_result
            .split_once('\n')
            .ok_or(CommitParseError::LocSyntaxError)?;
        if loc.is_empty() {
            // We still need to consume the last line feed, otherwise the parser
            // will fail on the last commit.
            remainder_result = remainder;
            break;
        } else if loc.starts_with("commit") {
            break;
        }
        locs.push(LocDiff::parse(loc).map_err(CommitParseError::LocFailed)?);
        remainder_result = remainder;
        if let Some(remainder) = remainder_result.strip_prefix('\n') {
            remainder_result = remainder;
            break;
        }
    }

    let commit = Commit {
        commit: commit.into(),
        merge,
        author: commit_analyzer::author::Author::parse(author).map_err(CommitParseError::AuthorFailed)?,
        date: chrono::DateTime::parse_from_str(date, "%a %b %e %T %Y %z")
            .map_err(CommitParseError::DateFailed)?,
        message: message.into(),
        locs,
    };
    Ok((commit, remainder_result))
}

#[derive(Debug)]
pub struct Commit {
    commit: String,
    /// This field is currently unused, if this changes, you can delete this
    /// comment and the allow(dead_code) tag.
    #[allow(dead_code)]
    merge: Option<String>,
    author: commit_analyzer::author::Author,
    date: chrono::DateTime<chrono::FixedOffset>,
    message: String,
    locs: Vec<LocDiff>,
}

impl Commit {
    pub fn loc(&self, filter: &Filter) -> i64 {
        self.locs
            .iter()
            .filter(|l| filter.check_loc(l))
            .map(|l| l.loc())
            .sum()
    }
}
