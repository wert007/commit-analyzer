fn main() {
    let mut commit = include_str!("..\\commits.txt");
    let mut parsed_commits = vec![];
    while !commit.is_empty() {
        let result = parse_commit(commit);
        match result {
            Ok((result, remainder)) => {
                commit = remainder;
                parsed_commits.push(result);
            }
            Err(err) => {
                dbg!(err);
                // println!("{:100}", commit);
                break;
            }
        }
    }
    println!("{:#?}", parsed_commits);
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
    let (merge, remainder) = if let Some(it) = remainder
        .strip_prefix("Merge: ")
        .map(|s| s.split_once('\n').ok_or(CommitParseError::Unknown))
    {
        let (merge, remainder) = it?;
        (Some(merge.to_owned()), remainder)
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
    let commit = Commit {
        commit: commit.into(),
        merge,
        author: Author::parse(author).map_err(|err| CommitParseError::AuthorFailed(err))?,
        date: chrono::DateTime::parse_from_str(date, "%a %b %e %T %Y %z")
            .map_err(|err| CommitParseError::DateFailed(err))?,
        message,
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
            name: name.into(),
            email: email.into(),
        })
    }
}
