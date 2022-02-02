# Commit Analyzer

This is a simple tool, which takes the output of `git log
--output=outputfile.txt` and analyzes how much time somebody worked on it. E.g.
simply call `commit-analyzer outputfile.txt --author-equals "wert007"` to see
all commits, that fit the criteria and how much time there was put in it.