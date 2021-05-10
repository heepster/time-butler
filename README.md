Time Butler: A CLI tool for easy datetime conversions to
=====================================================
![Build Status](https://travis-ci.com/heepster/time-butler.svg?branch=main)

⚠️ Under Development ⚠️

> “There is no future. There is no past. Do you see? Time is simultaneous, an intricately structured jewel that humans insist on viewing one edge at a time, when the whole design is visible in every facet.” - Dr. Manhanttan

Time Butler converts datetimes of various formats into your local time.

## Build

```
cargo build --release
```
This will create a binary called `tb` in `./target/release/`

## Usage

```
USAGE:
    tb [FLAGS] [arg_datetime]

ARGS:
    <arg_datetime>    Datetime to parse

FLAGS:
    -h, --help       Prints help information
    -i, --stdin      Read from stdin
    -V, --version    Prints version information
```

Pass an EPOCH timestamp as an argument:
```
tb 1620601103
=> 2021-05-09T15:58:23-07:00
```

Pass a standard ISO8601 timestamp as an argument:
```
tb "2021-05-09T23:03:18+00:00"
=> 2021-05-09T16:03:18-07:00
```

Read from STDIN.
```
echo 1620601103 | tb -i
=> 2021-05-09T15:58:23-07:00
```

## Roadmap
* Continously pipe into stdin and transform only datetimes
* More parseable datetime formats