//! The global variables of this project.
//!
//! This module specifies the magic numbers of this project once and centrally
//! in order to avoid redundancy and to enhance the maintainability.
//!
//! The constants are grouped by the following categories:
//! * `application`: application related information, and
//! * `version`: version information

/// Application related settings.
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
}

/// Version related settings.
///
/// This module defines the current version of this applciation analogeously to
/// `Cargo.toml`.
pub mod version {
    /// This application's fix level.
    pub const FIX_LEVEL: u8 = 0u8;

    /// This application's major version.
    pub const MAJOR: u8 = 0u8;

    /// This application's minor version.
    pub const MINOR: u8 = 1u8;
}
