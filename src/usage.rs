use getopts :: Options;

/// A small in-app documentation.
///
/// * `options` - The options the render the tool tips for.
///
/// This function will write a brief usage information, including a short
/// introduction to the meaning of the configured options, to `stdout`.
pub fn usage (options: & Options) -> ()
{
    println ! ( "Usage:  commit-analyzer <FILE> [OPTIONS]\n{}"
              , options.usage ("Parses the output of `git log`.")
              );
    return;
}
