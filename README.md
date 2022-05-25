# Commit Analyzer

## Description

This is a simple tool which grabs the output of `git log --numstat` and analyzes
how much time somebody worked on the project.

For instance, simply call the one of the following lines to see all commits that
fit the criteria and how much time was spend creating them.

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

## License

This project's license is **MIT**.  The license can be found `LICENSE` in the
main directory of this repository.

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
| `--verbose`               | `-v`  | Always show the entire output.                                            |

Options marked with **OR** in their explanation can be specified multiple times
and will be evaluated by joining them using the logical OR (`||`).
