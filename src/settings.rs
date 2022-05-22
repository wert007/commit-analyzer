//! The global variables of this project.
//!
//! This module specifies the magic numbers of this project once and centrally
//! in order to avoid redundancy.

/// This application's name.
#[allow(dead_code)]
pub const APP_NAME: &str = "commit-analyzer";

/// This application's tool tip.
///
/// In order to show a brief usage synopsis to the user, this global variable
/// sets the information about the required and optional command line arguments.
#[allow(dead_code)]
pub const APP_TOOL_TIP: &str = "<FILE> [OPTIONS]";

/// This application's version information.
///
/// This constant represents this application's fix level.
#[allow(dead_code)]
pub const VERSIO_FIX_LEVEL: u8 = 0u8;

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
