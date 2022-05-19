use getopts :: Options;

pub fn usage (app_name: & str, options: & Options) -> ()
{
    println ! ("Usage: {} <FILE> [OPTIONS]", app_name);
    println ! ("{}", options.usage ("Parses the output of `git log`."));
    return;
}
