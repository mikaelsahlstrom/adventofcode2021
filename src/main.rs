extern crate clap;

use clap::{Arg, App};

mod day1;


const VERSION: &'static str = env!("CARGO_PKG_VERSION");
const NAME: &'static str = env!("CARGO_PKG_NAME");
const AUTHORS: &'static str = env!("CARGO_PKG_AUTHORS");
const DESCRIPTION: &'static str = env!("CARGO_PKG_DESCRIPTION");

fn main()
{
    let matches = App::new(NAME)
                    .version(VERSION)
                    .author(AUTHORS)
                    .about(DESCRIPTION)
                    .arg(Arg::with_name("day")
                         .help("Which day to run.")
                         .short("d")
                         .long("day")
                         .value_name("NUMBER")
                         .takes_value(true))
                    .arg(Arg::with_name("part")
                         .help("Which part of the day, if any.")
                         .short("p")
                         .long("part")
                         .value_name("NUMBER")
                         .takes_value(true))
                    .get_matches();

    let day = matches.value_of("day").unwrap_or("all");
    let part = matches.value_of("part").unwrap_or("all");

    match day
    {
        "1" =>
        {
            match part
            {
                "1" => day1::part1(),
                "2" => day1::part2(),
                "all" =>
                {
                    day1::part1();
                    day1::part2();
                },
                _ => ()
            }
        },
        "all" =>
        {
            day1::part1();
            day1::part2();
        }
        _ => ()
    }
}
