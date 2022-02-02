fn main() {
    let mut opts = getopts::Options::new();
    let opts = opts
        .optflag("q", "quiet", "Hides the output of commit messages")
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
        );
    let matches = match opts.parse(std::env::args()) {
        Ok(it) => it,
        Err(_) => {
            usage("commit-analzer", opts);
            return;
        }
    };
    if matches.free.len() < 2 {
        usage(&matches.free[0], opts);
        return;
    }
    let is_quiet = matches.opt_present("q");
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
    let mut duration = chrono::Duration::zero();
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
    };
    for commit in parsed_commits.into_iter().rev() {
        if matches_filter(&commit, &filter) {
            if let Some(last_time) = last_time {
                let diff: chrono::Duration = commit.date - last_time;
                if diff.num_hours() <= 3 {
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
}

fn usage(program_name: &str, opts: &getopts::Options) {
    println!("Usage: {} <FILE> [OPTIONS]\n", program_name);
    println!("{}", opts.usage("Parses the output of `git log`."));
}

fn matches_filter(commit: &Commit, filter: &Filter) -> bool {
    filter.check_author_name(&commit.author.name)
        && filter.check_author_email(&commit.author.email)
        && filter.check_commit(&commit.commit)
        && filter.check_message(&commit.message)
}

#[derive(Debug, Default)]
struct Filter {
    author_equals: Vec<String>,
    author_contains: Vec<String>,
    email_equals: Vec<String>,
    email_contains: Vec<String>,
    commit_equals: Vec<String>,
    commit_contains: Vec<String>,
    message_equals: Vec<String>,
    message_contains: Vec<String>,
    message_starts_with: Vec<String>,
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
}

#[derive(Debug)]
enum CommitParseError {
    CommitMissing,
    AuthorMissing,
    DateMissing,
    AuthorFailed(AuthorParseError),
    DateFailed(chrono::ParseError),
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
    let (date, remainder) = remainder
        .strip_prefix("Date:   ")
        .ok_or(CommitParseError::DateMissing)?
        .split_once('\n')
        .ok_or(CommitParseError::DateMissing)?;

    let mut message = String::new();
    let mut remainder_result = "";
    let mut space_count = 0;
    let mut increase_space_count = false;
    for (index, char) in remainder.char_indices() {
        if increase_space_count {
            if char == ' ' {
                space_count += 1;
            } else {
                increase_space_count = false;
            }
        } else if char == '\n' {
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
    let commit = Commit {
        commit: commit.into(),
        merge,
        author: Author::parse(author).map_err(|err| CommitParseError::AuthorFailed(err))?,
        date: chrono::DateTime::parse_from_str(date, "%a %b %e %T %Y %z")
            .map_err(|err| CommitParseError::DateFailed(err))?,
        message: message.into(),
    };
    Ok((commit, remainder_result))
}

#[derive(Debug)]
pub struct Commit {
    commit: String,
    merge: Option<String>,
    author: Author,
    date: chrono::DateTime<chrono::FixedOffset>,
    message: String,
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
