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
            super::version::MAJOR,
            super::version::MINOR,
            super::version::FIX_LEVEL,
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

/// Version related settings analogously to `Cargo.toml`.
pub mod version {
    /// This application's fix level.
    pub const FIX_LEVEL: u8 = 0u8;

    /// This application's major version.
    pub const MAJOR: u8 = 0u8;

    /// This application's minor version.
    pub const MINOR: u8 = 1u8;
}
