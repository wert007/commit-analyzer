/// An abbreviation for the filter checks.
///
/// This function checks whether the given `commit` matches the expectations
/// defined in the given `filter`.
pub fn matches_filter(commit: &crate::Commit, filter: &Filter) -> bool {
    filter.check_author_name(commit.author.name())
        && filter.check_author_email(commit.author.email())
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

    pub fn check_loc(&self, loc: &&crate::LocDiff) -> bool {
        self.file_extension.is_empty()
            || self
                .file_extension
                .iter()
                .any(|ext| loc.file().ends_with(&format!(".{}", ext)))
    }

    pub fn new (matches: &getopts::Matches) -> Filter {
        Self {
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
        }
    }
}
