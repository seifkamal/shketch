![Build](https://github.com/safe-k/shketch/workflows/Build/badge.svg)

# shketch

A command line ASCII drawing tool.

## Installation

### Cargo

```shell script
cargo install --git https://github.com/safe-k/shketch
```

## Usage

```shell script
> shketch --help
 ______                 ________________
|        |    |  |     /       |  |       |    |
|______  |____|  |____/_____   |  |       |____|
     /  /    /  /     \        |  |      /    /
____/  /    /  /       \____   |  \_____/    /


Shketch 0.1.0
An ASCII drawing tool

USAGE:
    shketch [OPTIONS]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -b <backward_diagonal>        Cursor character for this direction
    -d <down>                     Cursor character for this direction
    -f <forward_diagonal>         Cursor character for this direction
    -l <left>                     Cursor character for this direction
    -r <right>                    Cursor character for this direction
    -u <up>                       Cursor character for this direction

Run to start drawing on a new canvas
```
