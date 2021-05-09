use std::env;

mod lib;

fn main() {
    let args: Vec<String> = env::args().collect();

    let time_str = &args[1];

    std::process::exit(match lib::date_parse(time_str) {
        Ok(local_time) => {
            println!("{:?}", local_time);
            0
        }
        Err(_) => 1,
    });
}
