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

| Option    | Brief | Meaning                                                  |
|:---------:|:-----:|:---------------------------------------------------------|
| `--help`  | `-h`  | Show this help and exit.                                 |

