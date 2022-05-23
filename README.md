# Commit Analyzer

## Description

This is a simple tool which takes the output of `git log` and analyzes how much
time somebody worked on the project.

For instance, simply call the following lines to see all commits that fit the
criteria and how much time was spend creating them.

```
git log --output=outputfile.txt
commit-analyzer outputfile.txt --author-equals "wert007"
```

You can also use the output of `git log --numstat` and give `commit-analyzer`
an output file such that it will generate a CSV with the date, the number of
commits made on that day and the loc changes that have been made.

```
git log --numstat --output=outputfile.txt
commit-analyzer outputfile.txt --output "commits-and-loc-by-date.csv"
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
| `--file-extension`        | `-f`  | (**OR**) Filter loc for certain file extension, e.g.`-f cpp`.             |
| `--help`                  | `-h`  | Show this help and exit.                                                  |
| `--message-contains`      | `-m`  | (**OR**) Filter for certain commit messages.                              |
| `--message-equals`        |       | (**OR**) Filter for certain commit messages.                              |
| `--message-starts-with`   | `-l`  | (**OR**) Filter for certain commit messages.                              |
| `--output`                | `-o`  | An output file for the commits per day in CSV format.                     |
| `--verbose`               | `-v`  | Always show the entire output.                                            |

Options marked with **OR** in their explanation can be specified multiple times
and will be evaluated by joining them using the logical OR (`||`).
