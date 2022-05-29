//! The utility functions and data structures of this project.

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

    /// Extract the author information from the given line.
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

    /// Construct a new instance from the raw input data.
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

/// The revealed filter creteria.
///
/// This data structure allows to filter the input commits by certain creteria.
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

impl Filter {
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

    /// Create a new instance from a given set of filter creteria.
    pub fn new(matches: &getopts::Matches) -> Self {
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

/// A brief in-app documentation.
///
/// This function will write a brief usage information, including a short
/// introduction to the meaning of the configured `options`, to `stdout`.
pub fn usage(options: &getopts::Options) {
    let description = "Parses the Git history.";
    let name = "commit-analyzer";
    let synopsis = "[OPTIONS]";

    println!(
        "{name}.\n{description}\n\n{}",
        options.usage(&format!("Usage: {name} {synopsis}"))
    );
}
