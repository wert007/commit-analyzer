#[derive(Debug)]
pub struct Author {
    name: String,
    email: String,
}

impl Author {
    pub fn email(&self) -> String {
        String::from(&self.email)
    }

    pub fn name(&self) -> String {
        String::from(&self.name)
    }

    pub fn parse(author: &str) -> Result<Author, AuthorParseError> {
        let (name, remainder) = author.split_once('<').ok_or(AuthorParseError::NameFailed)?;

        let email = remainder
            .strip_suffix('>')
            .ok_or(AuthorParseError::EmailFailed)?
            .trim()
            .into();

        Ok(Self {
            name: name.trim().into(),
            email,
        })
    }
}

#[derive(Debug)]
pub enum AuthorParseError {
    NameFailed,
    EmailFailed,
}
