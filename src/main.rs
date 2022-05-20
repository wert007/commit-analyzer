use chrono::{DateTime, Duration, FixedOffset, ParseError};
use getopts::Options;
use std::{collections::HashMap, error::Error, io::Write, num::ParseIntError, ops::AddAssign};

fn main() -> Result<(), Box<dyn Error>> {
    let mut opts = Options::new();
    let opts = opts
        .optflag("q", "quiet", "Hides the output of commit messages.")
        .optflag("h", "help", "Show this help and exit.")
        .optmulti(
            "a",
            "author-contains",
            "Filters after certain author names. ORs if multiple specified.",
            "NAME",
        )
        .optmulti(
            "",
            "author-equals",
            "Filters after certain author names. ORs if multiple specified.",
            "NAME",
        )
        .optmulti(
            "e",
            "email-contains",
            "Filters after certain author emails. ORs if multiple specified.",
            "EMAIL",
        )
        .optmulti(
            "",
            "email-equals",
            "Filters after certain author emails. ORs if multiple specified.",
            "EMAIL",
        )
        .optmulti(
            "c",
            "commit-contains",
            "Filters after certain commit hashes. ORs if multiple specified.",
            "HASH",
        )
        .optmulti(
            "",
            "commit-equals",
            "Filters after certain commit hashes. ORs if multiple specified.",
            "HASH",
        )
        .optmulti(
            "m",
            "message-contains",
            "Filters after certain commit messages. ORs if multiple specified.",
            "MESSAGE",
        )
        .optmulti(
            "",
            "message-equals",
            "Filters after certain commit messages. ORs if multiple specified.",
            "MESSAGE",
        )
        .optmulti(
            "l",
            "message-starts-with",
            "Filters after certain commit messages. ORs if multiple specified.",
            "MESSAGE",
        )
        .optmulti(
            "f",
            "file-extension",
            "Filters loc after certain file extension (e.g. `--file-extension cpp`). ORs if multiple specified.",
            "EXTENSION",
        )
        .optopt(
            "d",
            "duration",
            "The time, which may pass between two commits, that still counts as working.",
            "HOURS",
        )
        .optopt(
            "o",
            "output",
            "An output file for the commits per day in csv format.",
            "FILE",
        );
    let matches = match opts.parse(std::env::args()) {
        Ok(it) => it,
        Err(err) => {
            usage(opts);
            return Err(err.into());
        }
    };
    if matches.opt_present("h") || matches.free.len() < 2 {
        usage(opts);
        return Ok(());
    }
    let is_quiet = matches.opt_present("q");
    let max_diff_hours: u32 = match matches.opt_str("duration").map(|str| str.parse()) {
        None => 3,
        Some(Ok(it)) => it,
        Some(Err(err)) => {
            eprintln!("duration must be an integer value!");
            return Err(err.into());
        }
    };
    let path = matches.opt_str("output");
    let commits_path = &matches.free[1];
    let commits = std::fs::read_to_string(commits_path).unwrap();
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
    let mut duration = Duration::zero();
    let filter = Filter {
        author_equals: matches.opt_strs("author-equals"),
        author_contains: matches.opt_strs("author-contains"),
        email_equals: matches.opt_strs("email-equals"),
        email_contains: matches.opt_strs("email-contains"),
        commit_equals: matches.opt_strs("commit-equals"),
        commit_contains: matches.opt_strs("commit-contains"),
        message_equals: matches.opt_strs("message-equals"),
        message_contains: matches.opt_strs("message-contains"),
        message_starts_with: matches.opt_strs("message-starts-with"),
        file_extension: matches.opt_strs("file-extension"),
    };
    let mut commit_count = 0;
    let mut commits_per_day = HashMap::new();
    let mut loc_per_day = HashMap::new();
    for commit in parsed_commits.into_iter().rev() {
        if matches_filter(&commit, &filter) {
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
                let diff: Duration = commit.date - last_time;
                if diff.num_hours() <= max_diff_hours as i64 {
                    duration = duration + diff;
                }
            }
            last_time = Some(commit.date);
            if !is_quiet {
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

/// A small in-app documentation.
///
/// This function will write a brief usage information, including a short
/// introduction to the meaning of the configured `options`, to `stdout`.
fn usage(options: &Options) {
    println!(
        "Usage:  commit-analyzer <FILE> [OPTIONS]\n\n{}",
        options.usage("Parses the output of `git log`.")
    );
}

fn matches_filter(commit: &Commit, filter: &Filter) -> bool {
    filter.check_author_name(&commit.author.name)
        && filter.check_author_email(&commit.author.email)
        && filter.check_commit(&commit.commit)
        && filter.check_message(&commit.message)
}

#[derive(Debug, Default)]
pub struct Filter {
    author_equals: Vec<String>,
    author_contains: Vec<String>,
    email_equals: Vec<String>,
    email_contains: Vec<String>,
    commit_equals: Vec<String>,
    commit_contains: Vec<String>,
    message_equals: Vec<String>,
    message_contains: Vec<String>,
    message_starts_with: Vec<String>,
    file_extension: Vec<String>,
}

impl Filter {
    fn check_author_name(&self, name: &str) -> bool {
        let equals = self.author_equals.is_empty() || self.author_equals.iter().any(|n| n == name);
        let contains = self.author_contains.is_empty()
            || self.author_contains.iter().any(|n| name.contains(n));
        equals && contains
    }

    fn check_author_email(&self, email: &str) -> bool {
        let equals = self.email_equals.is_empty() || self.email_equals.iter().any(|e| e == email);
        let contains =
            self.email_contains.is_empty() || self.email_contains.iter().any(|e| email.contains(e));
        equals && contains
    }

    fn check_commit(&self, commit: &str) -> bool {
        let equals =
            self.commit_equals.is_empty() || self.commit_equals.iter().any(|c| c == commit);
        let contains = self.commit_contains.is_empty()
            || self.commit_contains.iter().any(|c| commit.contains(c));
        equals && contains
    }

    fn check_message(&self, message: &str) -> bool {
        let equals =
            self.message_equals.is_empty() || self.message_equals.iter().any(|m| m == message);
        let contains = self.message_contains.is_empty()
            || self.message_contains.iter().any(|m| message.contains(m));
        let starts_with = self.message_starts_with.is_empty()
            || self
                .message_starts_with
                .iter()
                .any(|m| message.starts_with(m));
        equals && contains && starts_with
    }

    fn check_loc(&self, loc: &&Loc) -> bool {
        self.file_extension.is_empty()
            || self
                .file_extension
                .iter()
                .any(|ext| loc.file.ends_with(&format!(".{}", ext)))
    }
}

#[derive(Debug)]
enum CommitParseError {
    CommitMissing,
    AuthorMissing,
    DateMissing,
    AuthorFailed(AuthorParseError),
    DateFailed(ParseError),
    LocSyntaxError,
    LocFailed(LocParseError),
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
            remainder_result = &remainder[index..];
            break;
        }
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
        if loc.is_empty() || loc.starts_with("commit") {
            break;
        }
        locs.push(Loc::parse(loc).map_err(CommitParseError::LocFailed)?);
        remainder_result = remainder;
        if let Some(remainder) = remainder_result.strip_prefix('\n') {
            remainder_result = remainder;
            break;
        }
    }

    let commit = Commit {
        commit: commit.into(),
        merge,
        author: Author::parse(author).map_err(CommitParseError::AuthorFailed)?,
        date: DateTime::parse_from_str(date, "%a %b %e %T %Y %z")
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
    author: Author,
    date: DateTime<FixedOffset>,
    message: String,
    locs: Vec<Loc>,
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

#[derive(Debug)]
pub struct Author {
    name: String,
    email: String,
}

#[derive(Debug)]
enum AuthorParseError {
    NameFailed,
    EmailFailed,
}

impl Author {
    fn parse(author: &str) -> Result<Author, AuthorParseError> {
        let (name, remainder) = author.split_once('<').ok_or(AuthorParseError::NameFailed)?;
        let email = remainder
            .strip_suffix('>')
            .ok_or(AuthorParseError::EmailFailed)?;
        Ok(Self {
            name: name.trim().into(),
            email: email.trim().into(),
        })
    }
}

#[derive(Debug)]
enum LocParseError {
    FirstTabulatorMissing,
    SecondTabulatorMissing,
    AddedParseError(ParseIntError),
    RemovedParseError(ParseIntError),
}

#[derive(Debug)]
struct Loc {
    added: Option<u32>,
    removed: Option<u32>,
    file: String,
}

impl Loc {
    pub fn parse(loc: &str) -> Result<Self, LocParseError> {
        let (added, remainder) = loc
            .split_once('\t')
            .ok_or(LocParseError::FirstTabulatorMissing)?;
        let (removed, file) = remainder
            .split_once('\t')
            .ok_or(LocParseError::SecondTabulatorMissing)?;
        let added = if added == "-" {
            None
        } else {
            Some(added.parse().map_err(LocParseError::AddedParseError)?)
        };
        let removed = if removed == "-" {
            None
        } else {
            Some(removed.parse().map_err(LocParseError::RemovedParseError)?)
        };
        let file = file.into();
        Ok(Self {
            added,
            removed,
            file,
        })
    }

    fn loc(&self) -> i64 {
        if self.added.is_none() && self.removed.is_none() {
            0
        } else {
            self.added.unwrap() as i64 - self.removed.unwrap() as i64
        }
    }
}
