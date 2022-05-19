use getopts :: Options;

pub fn usage (options: & Options) -> ()
{
    println ! ( "Usage: commit-analyzer <FILE> [OPTIONS]\n{}"
              , options.usage ("Parses the output of `git log`.")
              );
    return;
}
