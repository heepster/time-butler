Time Butler: A CLI tool for easy datetime conversions
=====================================================
![Build Status](https://travis-ci.com/heepster/time-butler.svg?branch=main)

⚠️ Under Active Development ⚠️

Time Butler converts datetimes of various formats into your local time.


## Usage

```
cargo run 1620601103
=> 2021-05-09T15:58:23-07:00

cargo run "2021-05-09T23:03:18+00:00"
=> 2021-05-09T16:03:18-07:00
```