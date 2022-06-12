# Commit Analyzer

## Summary

Gets the time somebody worked on something from `git log`.

## License

This project's license is **MIT** and can be found in `LICENSE` in the main
directory of this repository.

## Description

This is a simple tool which grabs the output of `git log --numstat` and analyzes
how much time somebody worked on the project.

For instance, simply call one of the following lines to see all commits that fit
the criteria and how much time was spend creating them.

```
commit-analyzer --git --author-equals wert007
commit-analyzer --input git.log --author-equals wert007
```

You can also give `commit-analyzer` an output file such that it will generate a
CSV with the date, the number of commits made on the respective days and the LOC
changes due to them.

```
commit-analyzer --git --output commits-and-loc-by-date.csv
commit-analyzer --input git.log --output commits-and-loc-by-date.csv
```

## Configured Options

| Option                    | Brief | Meaning                                                                   |
|:-------------------------:|:-----:|:--------------------------------------------------------------------------|
| `--author-contains`       | `-a`  | (**OR**) Filter for certain author names.                                 |
| `--author-equals`         |       | (**OR**) Filter for certain author names.                                 |
| `--commit-contains`       | `-c`  | (**OR**) Filter for certain commit hashes.                                |
| `--commit-equals`         |       | (**OR**) Filter for certain commit hashes.                                |
| `--email-contains`        | `-e`  | (**OR**) Filter for certain author emails.                                |
| `--email-equals`          |       | (**OR**) Filter for certain author emails.                                |
| `--duration`              | `-d`  | The time which may pass between two commits that still counts as working. |
| `--file-extension`        | `-f`  | (**OR**) Filter loc for certain file extension, e.g. `-f cpp`.            |
| `--git`                   |       | Grab the input data from the local Git history.                           |
| `--help`                  | `-h`  | Show this help and exit.                                                  |
| `--input`                 | `-i`  | The log file to read from.                                                |
| `--message-contains`      | `-m`  | (**OR**) Filter for certain commit messages.                              |
| `--message-equals`        |       | (**OR**) Filter for certain commit messages.                              |
| `--message-starts-with`   | `-l`  | (**OR**) Filter for certain commit messages.                              |
| `--output`                | `-o`  | An output file for the commits per day in CSV format.                     |
| `--stdin`                 |       | Read input data from `stdin`.                                             |
| `--verbose`               | `-v`  | Always show the entire output.                                            |

Options marked with **OR** in their explanation can be specified multiple times
and will be evaluated by joining them using the logical OR (`||`).

## Installation and Updating

Installation is very easy with Cargo. Just execute the following line in a
terminal:

```
cargo install --git https://github.com/wert007/commit-analyzer
```

This command will install a release compilation of the latest version locally to
your user account. The same command also works for regular updates, this is, new
commits have had been introduced since the last installation.

Cargo will refuse to replace the currently installed executable if no new
commits were found. To override the current binary with the latest repository 
state anyway, rerun with `--force`:

```
cargo install --force --git https://github.com/wert007/commit-analyzer
```

Cargo will then replace the currently installed release binary with a newly
compiled one from the latest repository state using the configured release
profile.
