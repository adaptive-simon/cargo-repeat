<div align="center">

# cargo-repeat

Repeats a command until it exits with the right code
</div>

## Introduction

This crate is a nice little utility function that lets you repeat a command indefinitely until it matches a certain exit
code (default 0).

## Usage

```bash
# Output of cargo repeat --help
cargo-repeat 0.1.0
Repeats a command until it exits with the right code

USAGE:
    cargo-repeat.exe [OPTIONS] [-- <TARGET>...]

ARGS:
    <TARGET>...    Command and arguments to repeat

OPTIONS:
    -c, --code <i32>     Exit code to repeat until it's hit [default: 0]
    -h, --help           Print help information
    -s, --sleep <f64>    Delay between retries [default: 1]
    -v, --verbose        Verbose
    -V, --version        Print version information
```

## Examples

```bash
# Keeps saying "hello world" until it exits with a 0 status
cargo repeat -- echo "hello world"

# Will keep running "python test.py" until it exits with status 5
cargo repeat -c 5 -- python test.py

# Keeps saying "hello" with a 0.5 second delay
cargo repeat -s 0.5 -- echo "hello" 

# Says hello and outputs some statistics at the end
cargo repeat -v -- echo "hello"
```


