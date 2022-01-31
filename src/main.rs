fn main() {
    let opts = getopts::Options::new();
    let matches = opts.parse(std::env::args()).unwrap();
    if matches.free.len() < 2 {
        usage(&matches.free[0], opts);
        return;
    }
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
    let filter = Filter::none();
    for commit in parsed_commits.into_iter().rev() {
        if matches_filter(&commit, &filter) {
            if let Some(last_time) = last_time {
                let diff : chrono::Duration = commit.date - last_time;
                if diff.num_hours() <= 3 {
                    duration = duration + diff;
                }
            }
            last_time = Some(commit.date);
            println!("{:#?}", commit);
        }
    }

    println!("Estimated time was {}h", duration.num_hours());
}

fn usage(program_name: &str, opts: getopts::Options) {
    println!("Usage: {} <FILE> [OPTIONS]\n", program_name);
    println!("{}", opts.usage("Parses the output of `git log`."));
}

fn matches_filter(commit: &Commit, filter: &Filter) -> bool {
    true
}

#[derive(Debug)]
struct Filter {

}

impl Filter {
    pub fn none() -> Self {
        Self {}
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
