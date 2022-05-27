//! The `Filter` struct.
//!
//! The data structure defined by this module allows to filter the input commits
//! by certain creteria.

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

    /// A set of substrings to be contained by some authors' email addresses.
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

/// The methods and associated functions on and for a `Filter`, respectively.
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
    /// This function checks whether the given `commit` matches the expectations
    /// defined in this `filter`.
    pub fn matches(&self, commit: &crate::Commit) -> bool {
        self.check_author_name(commit.author.name())
            && self.check_author_email(commit.author.email())
            && self.check_commit(&commit.commit)
            && self.check_message(&commit.message)
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
