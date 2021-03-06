use std::io::{self, BufRead};
use clap::{App, AppSettings, Arg};

mod lib;

fn main() {
    const VERSION: &'static str = env!("CARGO_PKG_VERSION");
    const AUTHOR: &'static str = env!("CARGO_PKG_AUTHORS");

    let matches = App::new("Time Butler")
        .global_setting(
          AppSettings::ColoredHelp
        )
        .version(VERSION)
        .author(AUTHOR)
        .about("A tool for easy datetime conversions")
        .arg(
            Arg::new("arg_datetime")
                .about("Datetime to parse")
                .index(1)
        )
        .arg(
            Arg::new("stdin")
                .short('i')
                .long("stdin")
                .about("Read from stdin")
        )
        .get_matches();

    if matches.is_present("stdin") {
        for line in io::stdin().lock().lines() {
            let line = line.expect("Could not read from stdin");
            let parsed = lib::replace_datetime_in_str(&line);
            match parsed {
                Some(o) => println!("{}", o),
                None => println!("{}", line)
            }
        }
    } else if let Some(o) = matches.value_of("arg_datetime") {
        let time_str = o.to_string();
        std::process::exit(match lib::replace_datetime_in_str(&time_str) {
            Some(local_time) => {
                println!("{}", local_time);
                0
            }
            None => 1,
        });
    } else {
        println!("Must pass datetime as an argument");
        std::process::exit(1);
    }
}
