# game-of-life-rs
An implementation of Game of Life as CLI

This project uses Rust `v1.50.0`

## How to use this CLI?

```shell

gol 0.0.1
Simulate Conway's Game of Life on the command line

USAGE:
    game-of-life-rs [OPTIONS]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -c, --columns <columns>    Sets the number of columns (default: 50)
        --delay <delay>        Describes the duration of time between updates (default: 1 second)
    -r, --rows <rows>          Sets the number of rows (default: 50)
    -s, --states <states>      Maximum number of states to be iterated (default: 20)

```
