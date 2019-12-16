use clap::{Arg, App};

pub fn app() -> App<'static, 'static> {
    App::new("Advent of Code 2019")
        .arg(Arg::with_name("day")
            .help("Which day to execute")
            .short("d")
            .long("day")
            .takes_value(true)
            .multiple(true)
            .value_delimiter(",")
            .required(true)
            .validator(valid_day))
        .arg(Arg::with_name("input")
            .help("Path to puzzle input file")
            .requires("day")
            .short("i")
            .long("input")
            .multiple(false)
            .takes_value(true))
}

fn valid_day(s: String) -> Result<(), String> {
    if let Ok(u) = s.parse::<u8>() {
        if u <= 25 {
            Ok(())
        } else {
            Err("Must be less than or equal to 25!".to_string())
        }
    } else {
        Err("Must be a number less than or equal to 25!".to_string())
    }
}
