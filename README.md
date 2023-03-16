<!------------------------------------------------------------------------->

[license]: https://img.shields.io/github/license/wert007/commit-analyzer
[repository]: https://github.com/wert007/commit-analyzer

<!------------------------------------------------------------------------->

# Commit Analyzer

## Summary

[![][license]][repository]

Gets the time somebody worked on something from `git log`.

1. [Description](#description)
1. [Configured Options and Subcommands](#configured-options-and-subcommands)
   1. [Options](#options)
      1. [`--author-contains`, `-a`](#--author-contains--a)
      1. [`--author-equals`](#--author-equals)
      1. [`--commit-contains`, `-c`](#--commit-contains--c)
      1. [`--commit-equals`](#--commit-equals)
      1. [`--duration`, `-d`](#--duration--d)
      1. [`--email-contains`, `-e`](#--email-contains--e)
      1. [`--email-equals`](#--email-equals)
      1. [`--file-extension`, `-f`](#--file-extension--f)
      1. [`--help`, `-h`](#--help--h)
      1. [`--message-starts-with`, `-l`](#--message-starts-with--l)
      1. [`--message-contains`, `-m`](#--message-contains--m)
      1. [`--message-equals`](#--message-equals)
      1. [`--output`, `-o`](#--output--o)
      1. [`--verbose`, `-v`](#--verbose--v)
      1. [`--version`, `-V`](#--version--v)
   1. [Subcommands](#subcommands)
      1. [`git-history`](#git-history)
      1. [`help`](#help-1)
      1. [`log-file`](#log-file)
      1. [`stdin`](#stdin)
1. [Installation and Updation](#installation-and-updating)

## Description

This is a simple tool which grabs the output of `git log --numstat` and
analyzes how much time somebody worked on the project.

For instance, simply call one of the following lines to see all commits that
fit the criteria and how much time was spend creating them.

```bash
commit-analyzer --git --author-equals wert007
commit-analyzer --input git.log --author-equals wert007
```

You can also give `commit-analyzer` an output file such that it will
generate a CSV with the date, the number of commits made on the respective
days and the LOC changes due to them.

```bash
commit-analyzer --git --output commits-and-loc-by-date.csv
commit-analyzer --input git.log --output commits-and-loc-by-date.csv
```

## Configured Options and Subcommands

### Options

#### `--author-contains`, `-a`

Filters for certain author names. ORs if specified multiple times.

#### `--author-equals`

Filters for certain author names. ORs if specified multiple times.

#### `--commit-contains`, `-c`

Filters for certain commit hashes. ORs if specified multiple times.

#### `--commit-equals`

Filters for certain commit hashes. ORs if specified multiple times.

#### `--duration`, `-d`

The time which may pass between two commits that still counts as working
[default: 3].

#### `--email-contains`, `-e`

Filters for certain author emails. ORs if specified multiple times.

#### `--email-equals`

Filters for certain author emails. ORs if specified multiple times.

#### `--file-extension`, `-f`

Filters the LOC diff for a certain file extension (e.g. `--file-extension
cpp`). ORs if specified multiple times.

#### `--help`, `-h`

Print help information.

#### `--message-starts-with`, `-l`

Filters for certain commit messages. ORs if specified multiple times.

#### `--message-contains`, `-m`

Filters for certain commit messages. ORs if specified multiple times.

#### `--message-equals`

Filters for certain commit messages. ORs if specified multiple times.

#### `--output`, `-o`

An output file for the commits per day in CSV format.

#### `--verbose`, `-v`

Always shows the entire output.

#### `--version`, `-V`

Print version information.

### Subcommands

#### `git-history`

Reads the input from the local Git history.

#### `help`

Print a general help message or the help of the given subcommand(s).

#### `log-file`

Reads the specified input file.

#### `stdin`

Reads from `stdin`.

## Installation and Updating

Installation is very easy with Cargo. Just execute the following line in a
terminal:

```bash
cargo install --git https://github.com/wert007/commit-analyzer
```

This command will install a release compilation of the latest version
locally to your user account. The same command also works for regular
updates, this is, new commits have had been introduced since the last
installation.

<!------------------------------------------------------------------------->
