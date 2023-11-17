<div align="center">

# Stdirgen - CLI File/Directory Generation

<a href="https://www.rust-lang.org/learn"> ![Static Badge](https://img.shields.io/badge/rust_1.70%2B-orange?style=for-the-badge)
</a>
![Codecov](https://img.shields.io/codecov/c/github/jozhw/stdirgen?style=for-the-badge)
![GitHub Workflow Status (with event)](https://img.shields.io/github/actions/workflow/status/jozhw/stdirgen/test.yml?style=for-the-badge&label=tests)

</div>

<div align="center">

<br>

</div>

### What is Stdirgen?

Stdirgen is a Rust CLI tool used to generate enumerated files or directories.

### Commands

```
A standardized file and directory generator.

Usage: stdirgen <COMMAND>

Commands:
 directory
 file
 help       Print this message or the help of the given subcommand(s)

Options:
 -h, --help     Print help
 -V, --version  Print version

```

### Sub Commands

#### Directory

```
Usage: stdirgen directory [OPTIONS] --name <NAME>

Options:
  -n, --name <NAME>                Name of directory, enumeration will be appended at the end fo the name
  -p, --path <PATH>                Optional: Path to directory of interest to start the creation if null will use current d
irectory
  -i, --iter <ITER>                Sets the number of times iteration occurs, if null value is set at 1 [default: 1]
  -s, --start <START>              Optional: Sets the initial numbering, if null begins at 1 [default: 1]
  -f, --files <FILES>              Optional: Creates files within each directory created. Note: Requires the delimiter "."
and currently only allows for one delimiter "."
      --files-iter <FILES_ITER>    Sets the number of times iteration occurs
      --files-start <FILES_START>  Sets the start
  -h, --help                       Print help

```

#### File

```
Usage: stdirgen file [OPTIONS] --name <NAME>

Options:
  -n, --name <NAME>    Name of file, enumeration will be appended at the end of the name. Note: Must contain the delimiter
"." currently only allows for one delimiter "."
  -p, --path <PATH>    Optional: Path to directory of interest to start the creation if null will use current directory
  -i, --iter <ITER>    Sets the number of times iteration occurs, if null value is set at 1 [default: 1]
  -s, --start <START>  Optional: Sets the initial numbering, if null begins at 1 [default: 1]
  -h, --help           Print help

```
