//! The global variables of this project.
//!
//! This module specifies the magic numbers of this project once and centrally
//! in order to avoid redundancy and to enhance the maintainability.
//!
//! The constants are grouped by the following categories:
//!
//! * `application`: application related information, and
//! * `version`: version information

/// Application related settings.
pub mod application {
    /// This application's name.
    pub const NAME: &str = "commit-analyzer";

    /// This application's tool tip.
    ///
    /// In order to show a brief usage synopsis to the user, this global
    /// variable sets the information about the required and optional command
    /// line arguments.
    pub const TOOL_TIP: &str = "<FILE> [OPTIONS]";
}

/// Version related settings.
///
/// This module defines the current version of this applciation analogeously to
/// `Cargo.toml`.
pub mod version {
    /// This application's fix level.
    #[allow(dead_code)]
    pub const FIX_LEVEL: u8 = 0u8;

    /// This application's major version.
    #[allow(dead_code)]
    pub const MAJOR: u8 = 0u8;

    /// This application's minor version.
    #[allow(dead_code)]
    pub const MINOR: u8 = 1u8;
}
