//! The `Author` struct and related utilities.
//!
//! This module defines the `Author` data structure together with its utility
//! enum `AuthorParseError`.
//!
//! A valid author information consists of
//!
//! * the specified name, and
//! * the specified email address, wrapped in sharp brackets (`<>`).
//!
//! In case that some of the assumptions should fail, an according error from
//! this module will occur.

/// The author information.
#[derive(Debug)]
pub struct Author {
    /// The electronic contact information of the author.
    email: String,

    /// The author's name.
    name: String,
}

/// The methods and associated functions on and for an `Author`, respectively.
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

/// The set of errors which may occur.
#[derive(Debug)]
pub enum AuthorParseError {
    /// Parsing the email was not possible.
    EmailFailed,

    /// Parsing the name was not possible.
    NameFailed,
}
