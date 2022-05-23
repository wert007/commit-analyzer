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

/// This application's version information.
///
/// This constant represents this application's fix level.
#[allow(dead_code)]
pub const VERSION_FIX_LEVEL: u8 = 0u8;

/// This application's version information.
///
/// This constant represents this application's major version.
#[allow(dead_code)]
pub const VERSION_MAJOR: u8 = 0u8;

/// This application's version information.
///
/// This constant represents this application's minor version.
#[allow(dead_code)]
pub const VERSION_MINOR: u8 = 1u8;
