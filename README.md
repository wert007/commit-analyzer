# Commit Analyzer

This is a simple tool, which takes the output of `git log
--output=outputfile.txt` and analyzes how much time somebody worked on it. E.g.
simply call `commit-analyzer outputfile.txt --author-equals "wert007"` to see
all commits, that fit the criteria and how much time there was put in it.

You can also use the output of `git log --numstat --output=outputfile.txt` and
if you specify an output parameter in commit analyzer (e.g. `commit-analyzer
outputfile.txt --output "commits-and-loc-by-date.csv"`) it will generate an csv
with the date, the number of commits made on that day and the loc changes that
have been made.