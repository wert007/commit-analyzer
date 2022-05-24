//! The `Author` struct and related utilities.
//!
//! This module defines the `Author` data structure together with its utility
//! enum `AuthorParseError`.

/// The author information.
#[derive(Debug)]
pub struct Author {
    /// The electronic contact information of the author.
    email: String,

    /// The author's name.
    name: String,
}

/// The methods and associated functions on and for an `Author` instance,
/// respectively.
impl Author {
    /// The getter method for the field `email` of the corresponding struct.
    ///
    /// Since the struct `Author` defines its field `email` to be private, this
    /// getter is used in order to operate on this field.  It returns a
    /// read-only deep copy of the content.
    pub fn email(&self) -> String {
        String::from(&self.email)
    }

    /// The getter method for the field `name` of the corresponding struct.
    ///
    /// Since the struct `Author` defines its field `name` to be private, this
    /// getter is used in order to operate on this field.  It returns a
    /// read-only deep copy of the content.
    pub fn name(&self) -> String {
        String::from(&self.name)
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
///
/// This enum describes the possible errors when parsing the author information.
/// A valid author information consists of
/// * the specified name, and
/// * the specified email address, wrapped in sharp brackest (`<>`).
///
/// In case that some of the assumptions should fail, an according error from
/// this enum will occur.
#[derive(Debug)]
pub enum AuthorParseError {
    /// Parsing the email was not possible.
    EmailFailed,

    /// Parsing the name was not possible.
    NameFailed,
}
