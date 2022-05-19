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
main directory of this repository.  Its text is as follows:

> MIT License
>
> Copyright (c) 2022 Pablo Hamacher
>
> Permission is hereby granted, free of charge, to any person obtaining a copy
> of this software and associated documentation files (the "Software"), to deal
> in the Software without restriction, including without limitation the rights
> to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
> copies of the Software, and to permit persons to whom the Software is
> furnished to do so, subject to the following conditions:
>
> The above copyright notice and this permission notice shall be included in all
> copies or substantial portions of the Software.
>
> THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
> IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
> FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
> AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
> LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
> OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
> SOFTWARE.

## Software Requirements

| Requirement   | Type          | Role              |
|:--------------|:-------------:|:------------------|
| Cargo         | application   | Rust build system |

The application is build using one of the following commands.

```
cargo build             # Debug build.
cargo build --release   # Release build without debugging symbols.
```

The compilation is stored in the `./target/` directory.
