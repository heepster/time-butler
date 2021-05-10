use std::io;
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

    let time_str;

    if matches.is_present("stdin") {
        let mut stdin_str = String::new();
        if let Ok(_) = io::stdin().read_line(&mut stdin_str) {
            time_str = stdin_str
        } else {
            println!("Couldn't read from stdin");
            std::process::exit(1);
        }
    } else if let Some(o) = matches.value_of("arg_datetime") {
        println!("Must pass datetime as an argument");
        time_str = o.to_string();
    } else {
        std::process::exit(1);
    }

    std::process::exit(match lib::date_parse(&time_str) {
        Ok(local_time) => {
            println!("{:?}", local_time);
            0
        }
        Err(_) => 1,
    });
}
