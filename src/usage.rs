//! The module for the usage advice.
//!
//! The usage advice defined in this module is specialised regarding the needs
//! of this application.  This is,
//!
//! * the application name is hard coded,
//! * the tool tip is hard coded, and
//! * the expected order of arguments in the usage template is hard coded.

use getopts::Options;

/// A small in-app documentation.
///
/// This function will write a brief usage information, including a short
/// introduction to the meaning of the configured `options`, to `stdout`.
pub fn usage(options: &Options) {
    println!(
        "Usage:  commit-analyzer <FILE> [OPTIONS]\n\n{}",
        options.usage("Parses the output of `git log`.")
    );
}
