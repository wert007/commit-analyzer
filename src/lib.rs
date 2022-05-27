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

/// Version related settings analogously to `Cargo.toml`.
pub mod version {
    /// This application's fix level.
    pub const FIX_LEVEL: u8 = 0u8;

    /// This application's major version.
    pub const MAJOR: u8 = 0u8;

    /// This application's minor version.
    pub const MINOR: u8 = 1u8;
}
