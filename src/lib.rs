//! The utility functions and data structures of this project.

/// Application related settings and utility functions.
pub mod application {
    /// This application's purpose.
    ///
    /// A brief summary what this application is supposed to do.
    pub const DESCRIPTION: &str = "Parses the Git history.";

    /// This application's name.
    pub const NAME: &str = "commit-analyzer";

    /// This application's usage synopsis.
    ///
    /// In order to show a brief usage synopsis to the user, this global
    /// variable sets the information about the required and optional command
    /// line arguments.
    pub const SYNOPSIS: &str = "[OPTIONS]";

    /// A brief in-app documentation.
    ///
    /// This function will write a brief usage information, including a short
    /// introduction to the meaning of the configured `options`, to `stdout`.
    pub fn usage(options: &getopts::Options) {
        println!(
            "{NAME}, version {}.{}.{}. {DESCRIPTION}\n\n{}",
            crate::version::MAJOR,
            crate::version::MINOR,
            crate::version::FIX_LEVEL,
            options.usage(&format!("Usage: {NAME} {SYNOPSIS}"))
        );
    }
}

/// The `Author` struct and related utilities.
///
/// This module defines the `Author` data structure together with its utility
/// enum `AuthorParseError`.
///
/// A valid author information consists of
///
/// * the specified name, and
/// * the specified email address, wrapped in sharp brackets (`<>`).
///
/// In case that some of the assumptions should fail, an according error from
/// this module will occur.
pub mod author {
    /// The author information.
    #[derive(Debug)]
    pub struct Author {
        /// The electronic contact information of the author.
        email: String,

        /// The author's name.
        name: String,
    }

    /// The methods and associated functions.
    impl Author {
        /// The getter method for the field `email` of the corresponding struct.
        pub fn email(&self) -> &str {
            &self.email
        }

        /// The getter method for the field `name` of the corresponding struct.
        pub fn name(&self) -> &str {
            &self.name
        }

        /// Extract the author information from the given line.
        pub fn new(author: &str) -> Result<Author, AuthorParseError> {
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
        /// Parsing the email was not possible.
        EmailFailed,

        /// Parsing the name was not possible.
        NameFailed,
    }
}

/// The `Commit` struct and related utilities.
///
/// This module defines the `Commit` data structure together with its utility
/// enum `CommitParseError`.
pub mod commit {
    /// The set of errors which may occur.
    #[derive(Debug)]
    pub enum CommitParseError {
        /// Parsing the author information was not possible.
        AuthorFailed(crate::author::AuthorParseError),

        /// There were no author information.
        AuthorMissing,

        /// There was no commit.
        CommitMissing,

        /// Parsing the date was not possible.
        DateFailed(chrono::ParseError),

        /// There was no date.
        DateMissing,

        /// Parsing the LOC diff was not possible.
        LocFailed(crate::loc::LocParseError),

        /// The LOC diff was malformatted.
        LocSyntaxError,

        /// Another reason.
        Unknown,
    }

    #[derive(Debug)]
    pub struct Commit {
        commit: String,
        /// This field is currently unused, if this changes, you can delete this
        /// comment and the allow(dead_code) tag.
        #[allow(dead_code)]
        merge: Option<String>,
        author: crate::author::Author,
        date: chrono::DateTime<chrono::FixedOffset>,
        message: String,
        locs: Vec<crate::loc::LocDiff>,
    }

    impl Commit {
        pub fn author(&self) -> &crate::author::Author {
            &self.author
        }

        pub fn commit(&self) -> &str {
            &self.commit
        }

        pub fn date(&self) -> &chrono::DateTime<chrono::FixedOffset> {
            &self.date
        }

        pub fn loc(&self, filter: &crate::filter::Filter) -> i64 {
            self.locs
                .iter()
                .filter(|l| filter.check_loc(l))
                .map(|l| l.loc())
                .sum()
        }

        pub fn message(&self) -> &str {
            &self.message
        }

        pub fn new(commit: &str) -> Result<(Commit, &str), CommitParseError> {
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
                locs.push(crate::loc::LocDiff::parse(loc).map_err(CommitParseError::LocFailed)?);
                remainder_result = remainder;
                if let Some(remainder) = remainder_result.strip_prefix('\n') {
                    remainder_result = remainder;
                    break;
                }
            }

            let commit = Self {
                commit: commit.into(),
                merge,
                author: crate::author::Author::new(author)
                    .map_err(CommitParseError::AuthorFailed)?,
                date: chrono::DateTime::parse_from_str(date, "%a %b %e %T %Y %z")
                    .map_err(CommitParseError::DateFailed)?,
                message: message.into(),
                locs,
            };
            Ok((commit, remainder_result))
        }
    }
}

/// The `Filter` struct.
///
/// The data structure defined by this module allows to filter the input commits
/// by certain creteria.
pub mod filter {
    /// The revealed filter creteria.
    #[derive(Debug, Default)]
    pub struct Filter {
        /// A set of substrings to be contained by some authors' names.
        author_contains: Vec<String>,

        /// A set of strings to match some authors's names.
        author_equals: Vec<String>,

        /// A set of substrings to be contained by some commits' hashes.
        commit_contains: Vec<String>,

        /// A set of strings to match some commits' hashes.
        commit_equals: Vec<String>,

        /// A set of substrings to be contained by some authors' email
        /// addresses.
        email_contains: Vec<String>,

        /// A set of strings to match some authors' email addresses.
        email_equals: Vec<String>,

        /// A set of file extensions to filter by.
        file_extension: Vec<String>,

        /// A set of substrings to be contained by some commits' messages.
        message_contains: Vec<String>,

        /// A set of strings to match some commits' messages.
        message_equals: Vec<String>,

        /// A set of strings to introduce some commits' messages.
        message_starts_with: Vec<String>,
    }

    /// The methods and associated functions.
    impl Filter {
        /// Whether the author's email address matches the expectations.
        fn check_author_email(&self, email: &str) -> bool {
            let contains = self.email_contains.is_empty()
                || self.email_contains.iter().any(|e| email.contains(e));
            let equals =
                self.email_equals.is_empty() || self.email_equals.iter().any(|e| e == email);

            equals && contains
        }

        /// Whether the author's name matches the expectations.
        fn check_author_name(&self, name: &str) -> bool {
            let contains = self.author_contains.is_empty()
                || self.author_contains.iter().any(|n| name.contains(n));
            let equals =
                self.author_equals.is_empty() || self.author_equals.iter().any(|n| n == name);

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
        pub fn check_loc(&self, loc: &&crate::loc::LocDiff) -> bool {
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
        pub fn matches(&self, commit: &crate::commit::Commit) -> bool {
            self.check_author_name(commit.author().name())
                && self.check_author_email(commit.author().email())
                && self.check_commit(commit.commit())
                && self.check_message(commit.message())
        }

        /// Create a new instance from a given set of filter creteria.
        pub fn new(matches: &getopts::Matches) -> Filter {
            Self {
                author_equals: matches.opt_strs("author-equals"),
                author_contains: matches.opt_strs("author-contains"),
                commit_equals: matches.opt_strs("commit-equals"),
                commit_contains: matches.opt_strs("commit-contains"),
                email_equals: matches.opt_strs("email-equals"),
                email_contains: matches.opt_strs("email-contains"),
                file_extension: matches.opt_strs("file-extension"),
                message_equals: matches.opt_strs("message-equals"),
                message_contains: matches.opt_strs("message-contains"),
                message_starts_with: matches.opt_strs("message-starts-with"),
            }
        }
    }
}

/// The `LocDiff` struct and related utilities.
///
/// This module defines the `LocDiff` data structure together with its utility
/// enum `LocParseError`.
///
/// LOC is the abbreviation for the number of **l**ines **o**f **c**ode a
/// project has and a possible measure to tell how this project has grown or
/// shrunk due to a set of changes applied to it.
///
/// A valid LOC diff consists of
///
/// * the integral number of insertions,
/// * the integral number of deletions, and
/// * the affected file
///
/// with each of these pieces of information being separated by a tab character.
/// In case that some of these assumptions should fail, an according error from
/// this module will occur.
pub mod loc {
    /// The LOC diff a certain commit introduces.
    #[derive(Debug)]
    pub struct LocDiff {
        /// The count of insertions.
        added: Option<u32>,

        /// The count of deletions.
        removed: Option<u32>,

        /// The affected file.
        file: String,
    }

    /// The methods and associated functions.
    impl LocDiff {
        /// The getter method for the field `file` of the corresponding struct.
        pub fn file(&self) -> &str {
            &self.file
        }

        /// Calculate the LOC diff.
        pub fn loc(&self) -> i64 {
            if self.added.is_none() && self.removed.is_none() {
                0
            } else {
                self.added.unwrap() as i64 - self.removed.unwrap() as i64
            }
        }

        /// Extract the LOC diff information from the given line.
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

            Ok(Self {
                added,
                removed,
                file: String::from(file),
            })
        }
    }

    /// The set of errors which may occur.
    #[derive(Debug)]
    pub enum LocParseError {
        /// The tab character between the insertions and deletions is missing.
        FirstTabulatorMissing,

        /// The tab character between the deletions and file name is missing.
        SecondTabulatorMissing,

        /// The number of insertions could not be parsed correctly.
        AddedParseError(std::num::ParseIntError),

        /// The number of deletions could not be parsed correctly.
        RemovedParseError(std::num::ParseIntError),
    }
}

/// Version related settings analogously to `Cargo.toml`.
pub mod version {
    /// This application's fix level.
    pub const FIX_LEVEL: u8 = 0u8;

    /// This application's major version.
    pub const MAJOR: u8 = 0u8;

    /// This application's minor version.
    pub const MINOR: u8 = 1u8;
}
