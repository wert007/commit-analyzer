//! <!------------------------------------------------------------------------->
//!
//! [license]: https://img.shields.io/github/license/wert007/commit-analyzer
//! [repository]: https://github.com/wert007/commit-analyzer
//!
//! <!------------------------------------------------------------------------->
//!
//! # Commit Analyzer
//!
//! ## Summary
//!
//! [![][license]][repository]
//!
//! Gets the time somebody worked on something from `git log`.
//!
//! 1. [Description](#description)
//! 1. [Configured Options and Subcommands](#configured-options-and-subcommands)
//!    1. [Options](#options)
//!       1. [`--author-contains`, `-a`](#--author-contains--a)
//!       1. [`--author-equals`](#--author-equals)
//!       1. [`--commit-contains`, `-c`](#--commit-contains--c)
//!       1. [`--commit-equals`](#--commit-equals)
//!       1. [`--duration`, `-d`](#--duration--d)
//!       1. [`--email-contains`, `-e`](#--email-contains--e)
//!       1. [`--email-equals`](#--email-equals)
//!       1. [`--file-extension`, `-f`](#--file-extension--f)
//!       1. [`--help`, `-h`](#--help--h)
//!       1. [`--message-starts-with`, `-l`](#--message-starts-with--l)
//!       1. [`--message-contains`, `-m`](#--message-contains--m)
//!       1. [`--message-equals`](#--message-equals)
//!       1. [`--output`, `-o`](#--output--o)
//!       1. [`--verbose`, `-v`](#--verbose--v)
//!       1. [`--version`, `-V`](#--version--v)
//!    1. [Subcommands](#subcommands)
//!       1. [`git-history`](#git-history)
//!       1. [`help`](#help-1)
//!       1. [`log-file`](#log-file)
//!       1. [`stdin`](#stdin)
//! 1. [Installation and Updation](#installation-and-updating)
//!
//! ## Description
//!
//! This is a simple tool which grabs the output of `git log --numstat` and
//! analyzes how much time somebody worked on the project.
//!
//! For instance, simply call one of the following lines to see all commits that
//! fit the criteria and how much time was spend creating them.
//!
//! ```bash
//! commit-analyzer --git --author-equals wert007
//! commit-analyzer --input git.log --author-equals wert007
//! ```
//!
//! You can also give `commit-analyzer` an output file such that it will
//! generate a CSV with the date, the number of commits made on the respective
//! days and the LOC changes due to them.
//!
//! ```bash
//! commit-analyzer --git --output commits-and-loc-by-date.csv
//! commit-analyzer --input git.log --output commits-and-loc-by-date.csv
//! ```
//!
//! ## Configured Options and Subcommands
//!
//! ### Options
//!
//! #### `--author-contains`, `-a`
//!
//! Filters for certain author names. ORs if specified multiple times.
//!
//! #### `--author-equals`
//!
//! Filters for certain author names. ORs if specified multiple times.
//!
//! #### `--commit-contains`, `-c`
//!
//! Filters for certain commit hashes. ORs if specified multiple times.
//!
//! #### `--commit-equals`
//!
//! Filters for certain commit hashes. ORs if specified multiple times.
//!
//! #### `--duration`, `-d`
//!
//! The time which may pass between two commits that still counts as working
//! [default: 3].
//!
//! #### `--email-contains`, `-e`
//!
//! Filters for certain author emails. ORs if specified multiple times.
//!
//! #### `--email-equals`
//!
//! Filters for certain author emails. ORs if specified multiple times.
//!
//! #### `--file-extension`, `-f`
//!
//! Filters the LOC diff for a certain file extension (e.g. `--file-extension
//! cpp`). ORs if specified multiple times.
//!
//! #### `--help`, `-h`
//!
//! Print help information.
//!
//! #### `--message-starts-with`, `-l`
//!
//! Filters for certain commit messages. ORs if specified multiple times.
//!
//! #### `--message-contains`, `-m`
//!
//! Filters for certain commit messages. ORs if specified multiple times.
//!
//! #### `--message-equals`
//!
//! Filters for certain commit messages. ORs if specified multiple times.
//!
//! #### `--output`, `-o`
//!
//! An output file for the commits per day in CSV format.
//!
//! #### `--verbose`, `-v`
//!
//! Always shows the entire output.
//!
//! #### `--version`, `-V`
//!
//! Print version information.
//!
//! ### Subcommands
//!
//! #### `git-history`
//!
//! Reads the input from the local Git history.
//!
//! #### `help`
//!
//! Print a general help message or the help of the given subcommand(s).
//!
//! #### `log-file`
//!
//! Reads the specified input file.
//!
//! #### `stdin`
//!
//! Reads from `stdin`.
//!
//! ## Installation and Updating
//!
//! Installation is very easy with Cargo. Just execute the following line in a
//! terminal:
//!
//! ```bash
//! cargo install --git https://github.com/wert007/commit-analyzer
//! ```
//!
//! This command will install a release compilation of the latest version
//! locally to your user account. The same command also works for regular
//! updates, this is, new commits have had been introduced since the last
//! installation.
//!
//! <!------------------------------------------------------------------------->

use std::path::PathBuf;

use clap::{Parser, Subcommand};

/// Parses the Git history.
#[derive(Parser, Debug)]
#[clap(version, about, long_about = None)]
pub struct Args {
    /// Specifies how the program will read the Git history.
    #[clap(subcommand)]
    input_method: InputMethod,

    /// Always shows the entire output.
    #[clap(short = 'v', long = "verbose")]
    is_verbose: bool,

    /// Filters the LOC diff for a certain file extension (e.g.
    /// `--file-extension cpp`). ORs if specified multiple times.
    #[clap(short, long)]
    file_extension: Vec<String>,

    /// The time which may pass between two commits that still counts as working.
    #[clap(short, long, default_value_t = 3)]
    duration: u32,

    /// An output file for the commits per day in CSV format.
    #[clap(short, long)]
    output: Option<PathBuf>,

    /// Filters for certain author names. ORs if specified multiple times.
    #[clap(short, long)]
    author_contains: Vec<String>,

    /// Filters for certain author names. ORs if specified multiple times.
    #[clap(long)]
    author_equals: Vec<String>,

    /// Filters for certain author emails. ORs if specified multiple times.
    #[clap(short, long)]
    email_contains: Vec<String>,

    /// Filters for certain author emails. ORs if specified multiple times.
    #[clap(long)]
    email_equals: Vec<String>,

    /// Filters for certain commit hashes. ORs if specified multiple times.
    #[clap(short, long)]
    commit_contains: Vec<String>,

    /// Filters for certain commit hashes. ORs if specified multiple times.
    #[clap(long)]
    commit_equals: Vec<String>,

    /// Filters for certain commit messages. ORs if specified multiple times.
    #[clap(short, long)]
    message_contains: Vec<String>,

    /// Filters for certain commit messages. ORs if specified multiple times.
    #[clap(long)]
    message_equals: Vec<String>,

    /// Filters for certain commit messages. ORs if specified multiple times.
    #[clap(short = 'l', long)]
    message_starts_with: Vec<String>,
}

impl Args {
    /// Gets the input method specified by the user.
    #[must_use]
    pub fn input_method(&self) -> &InputMethod {
        &self.input_method
    }

    /// Gets the configured verbosity level of the program.
    #[must_use]
    pub fn is_verbose(&self) -> bool {
        self.is_verbose
    }

    /// Gets the maximum duration between two commits considered spent working.
    #[must_use]
    pub fn duration(&self) -> u32 {
        self.duration
    }

    /// Moves the output path specified by the user out of `Args`
    ///
    /// This method moves the specified path to the intended output file out of
    /// this struct by calling `Option::take`. This should, hence, be called
    /// just once but prevents an obsolete clone.
    #[must_use]
    pub fn take_output(&mut self) -> Option<PathBuf> {
        self.output.take()
    }

    /// Creates a new Filter as specified by the user.
    #[must_use]
    pub fn filter(&self) -> Filter {
        Filter {
            author_contains: &self.author_contains,
            author_equals: &self.author_equals,
            commit_contains: &self.commit_contains,
            commit_equals: &self.commit_equals,
            email_contains: &self.email_contains,
            email_equals: &self.email_equals,
            file_extension: &self.file_extension,
            message_contains: &self.message_contains,
            message_equals: &self.message_equals,
            message_starts_with: &self.message_starts_with,
        }
    }
}

/// The possible input methods.
#[derive(Subcommand, Debug)]
pub enum InputMethod {
    /// Reads the input from the local Git history.
    GitHistory,

    /// Reads the specified input file.
    LogFile {
        /// The log file to read from.
        log_file: PathBuf,
    },

    /// Reads from `stdin`.
    Stdin,
}

impl InputMethod {
    /// Processes the configured input method.
    pub fn read(&self) -> Result<String, Box<dyn std::error::Error>> {
        match self {
            Self::GitHistory => {
                let process = std::process::Command::new("git")
                    .arg("log")
                    .arg("--numstat")
                    .output()?;

                Ok(String::from_utf8(process.stdout)?)
            }
            Self::LogFile { log_file } => Ok(std::fs::read_to_string(log_file)?),
            Self::Stdin => {
                let mut input = String::new();

                loop {
                    let mut buffer = String::new();

                    if std::io::stdin().read_line(&mut buffer)? == 0 {
                        break;
                    }

                    input.push_str(&buffer);
                }

                Ok(input)
            }
        }
    }
}

/// The revealed filter criteria.
///
/// This data structure allows to filter the input commits by certain criteria.
#[derive(Debug, Default)]
pub struct Filter<'a> {
    /// A set of substrings to be contained by some authors' names.
    author_contains: &'a [String],

    /// A set of strings to match some authors's names.
    author_equals: &'a [String],

    /// A set of substrings to be contained by some commits' hashes.
    commit_contains: &'a [String],

    /// A set of strings to match some commits' hashes.
    commit_equals: &'a [String],

    /// A set of substrings to be contained by some authors' email addresses.
    email_contains: &'a [String],

    /// A set of strings to match some authors' email addresses.
    email_equals: &'a [String],

    /// A set of file extensions to filter by.
    file_extension: &'a [String],

    /// A set of substrings to be contained by some commits' messages.
    message_contains: &'a [String],

    /// A set of strings to match some commits' messages.
    message_equals: &'a [String],

    /// A set of strings to introduce some commits' messages.
    message_starts_with: &'a [String],
}

impl Filter<'_> {
    /// Whether the author's email address matches the expectations.
    fn check_author_email(&self, email: &str) -> bool {
        let contains =
            self.email_contains.is_empty() || self.email_contains.iter().any(|e| email.contains(e));
        let equals = self.email_equals.is_empty() || self.email_equals.iter().any(|e| e == email);

        equals && contains
    }

    /// Whether the author's name matches the expectations.
    fn check_author_name(&self, name: &str) -> bool {
        let contains = self.author_contains.is_empty()
            || self.author_contains.iter().any(|n| name.contains(n));
        let equals = self.author_equals.is_empty() || self.author_equals.iter().any(|n| n == name);

        equals && contains
    }

    /// Whether the commit meta data matches the expectations.
    fn check_commit(&self, commit: &str) -> bool {
        let contains = self.commit_contains.is_empty()
            || self.commit_contains.iter().any(|c| commit.contains(c));
        let equals =
            self.commit_equals.is_empty() || self.commit_equals.iter().any(|c| c == commit);

        equals && contains
    }

    /// Whether the LOC diff matches the expectations.
    pub fn check_loc(&self, loc: &&crate::LocDiff) -> bool {
        self.file_extension.is_empty()
            || self
                .file_extension
                .iter()
                .any(|ext| loc.file().ends_with(&format!(".{}", ext)))
    }

    /// Whether the message matches the expectations.
    fn check_message(&self, message: &str) -> bool {
        let contains = self.message_contains.is_empty()
            || self.message_contains.iter().any(|m| message.contains(m));
        let equals =
            self.message_equals.is_empty() || self.message_equals.iter().any(|m| m == message);
        let starts_with = self.message_starts_with.is_empty()
            || self
                .message_starts_with
                .iter()
                .any(|m| message.starts_with(m));

        equals && contains && starts_with
    }

    /// An abbreviation for the filter checks.
    ///
    /// This function checks whether the given `commit` matches the
    /// expectations defined in this `filter`.
    pub fn matches(&self, commit: &crate::Commit) -> bool {
        self.check_author_name(commit.author().name())
            && self.check_author_email(commit.author().email())
            && self.check_commit(commit.commit())
            && self.check_message(commit.message())
    }
}

/// The author meta data.
///
/// A valid author serialisation consists of
///
/// * the specified name, and
/// * the specified email address, wrapped in sharp brackets (`<>`).
///
/// In case that some of the assumptions should fail, an according error from
/// the utility enum `AuthorParseError` module will occur.
#[derive(Debug)]
pub struct Author {
    /// The email address.
    email: String,

    /// The author's name.
    name: String,
}

impl Author {
    /// The getter method for the field `email` of the corresponding struct.
    pub fn email(&self) -> &str {
        &self.email
    }

    /// The getter method for the field `name` of the corresponding struct.
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Extracts the author information from the given line.
    pub fn parse(author: &str) -> Result<Self, AuthorParseError> {
        let (name, remainder) = author.split_once('<').ok_or(AuthorParseError::NameFailed)?;

        Ok(Self {
            name: name.trim().into(),
            email: remainder
                .strip_suffix('>')
                .ok_or(AuthorParseError::EmailFailed)?
                .trim()
                .into(),
        })
    }
}

/// The set of errors which may occur.
#[derive(Debug)]
pub enum AuthorParseError {
    /// Parsing the email address was not possible.
    EmailFailed,

    /// Parsing the name was not possible.
    NameFailed,
}

/// The commit information.
#[derive(Debug)]
pub struct Commit {
    /// The author information.
    author: crate::Author,

    /// The commit's hash.
    commit: String,

    /// The commit's date.
    date: chrono::DateTime<chrono::FixedOffset>,

    /// The LOC diff information.
    locs: Vec<crate::LocDiff>,

    /// The merge information.
    ///
    /// This field is currently unused, if this changes, you can delete this
    /// `#[allow(dead_code)]` tag.
    #[allow(dead_code)]
    merge: Option<String>,

    /// The commit's description.
    message: String,
}

impl Commit {
    /// The getter method for the field `author` of the corresponding struct.
    pub fn author(&self) -> &crate::Author {
        &self.author
    }

    /// The getter method for the field `commit` of the corresponding struct.
    pub fn commit(&self) -> &str {
        &self.commit
    }

    /// The getter method for the field `date` of the corresponding struct.
    pub fn date(&self) -> &chrono::DateTime<chrono::FixedOffset> {
        &self.date
    }

    /// The getter method for the field `loc` of the corresponding struct.
    pub fn loc(&self, filter: &crate::Filter) -> i64 {
        self.locs
            .iter()
            .filter(|l| filter.check_loc(l))
            .map(|l| l.loc())
            .sum()
    }

    /// The getter method for the field `message` of the corresponding struct.
    pub fn message(&self) -> &str {
        &self.message
    }

    /// Constructs a new instance from the raw input data.
    pub fn parse(commit: &str) -> Result<(Self, &str), CommitParseError> {
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
            locs.push(crate::LocDiff::parse(loc).map_err(CommitParseError::LocFailed)?);
            remainder_result = remainder;
            if let Some(remainder) = remainder_result.strip_prefix('\n') {
                remainder_result = remainder;
                break;
            }
        }

        Ok((
            Self {
                commit: commit.into(),
                merge,
                author: crate::Author::parse(author).map_err(CommitParseError::AuthorFailed)?,
                date: chrono::DateTime::parse_from_str(date, "%a %b %e %T %Y %z")
                    .map_err(CommitParseError::DateFailed)?,
                message: message.into(),
                locs,
            },
            remainder_result,
        ))
    }
}

/// The set of errors which may occur.
#[derive(Debug)]
pub enum CommitParseError {
    /// Parsing the author information was not possible.
    AuthorFailed(crate::AuthorParseError),

    /// There were no author information.
    AuthorMissing,

    /// There was no commit hash.
    CommitMissing,

    /// Parsing the date was not possible.
    DateFailed(chrono::ParseError),

    /// There was no date.
    DateMissing,

    /// Parsing the LOC diff was not possible.
    LocFailed(crate::LocParseError),

    /// The LOC diff was malformatted.
    LocSyntaxError,

    /// Another reason.
    Unknown,
}

/// The LOC diff a certain commit introduces.
///
/// LOC is the abbreviation for the number of **l**ines **o**f **c**ode a
/// project has and a possible measure to tell how this project has grown or
/// shrunk due to a set of changes applied to it.
///
/// A valid LOC diff consists of
///
/// * the integral number of insertions or `-` in newer versions of Git,
/// * the integral number of deletions or `-` in newer versions of Git, and
/// * the affected file
///
/// with each of these pieces of information being separated by a tab character.
/// In case that some of these assumptions should fail, an according error from
/// the utility enum `LocParseError` will occur.
#[derive(Debug)]
pub struct LocDiff {
    /// The number of insertions.
    added: Option<u32>,

    /// The number of deletions.
    removed: Option<u32>,

    /// The affected file.
    file: String,
}

impl LocDiff {
    /// The getter method for the field `file` of the corresponding struct.
    pub fn file(&self) -> &str {
        &self.file
    }

    /// Calculates the LOC diff.
    pub fn loc(&self) -> i64 {
        if self.added.is_none() && self.removed.is_none() {
            0
        } else {
            self.added.unwrap() as i64 - self.removed.unwrap() as i64
        }
    }

    /// Extracts the LOC diff information from the given line.
    pub fn parse(loc: &str) -> Result<Self, LocParseError> {
        let (added, remainder) = loc
            .split_once('\t')
            .ok_or(LocParseError::FirstTabulatorMissing)?;
        let (removed, file) = remainder
            .split_once('\t')
            .ok_or(LocParseError::SecondTabulatorMissing)?;

        Ok(Self {
            added: if added == "-" {
                None
            } else {
                Some(added.parse().map_err(LocParseError::AddedParseError)?)
            },
            removed: if removed == "-" {
                None
            } else {
                Some(removed.parse().map_err(LocParseError::RemovedParseError)?)
            },
            file: String::from(file),
        })
    }
}

/// The set of errors which may occur.
#[derive(Debug)]
pub enum LocParseError {
    /// The number of insertions could not be parsed correctly.
    AddedParseError(std::num::ParseIntError),

    /// The tab character between the insertions and deletions is missing.
    FirstTabulatorMissing,

    /// The number of deletions could not be parsed correctly.
    RemovedParseError(std::num::ParseIntError),

    /// The tab character between the deletions and file name is missing.
    SecondTabulatorMissing,
}
