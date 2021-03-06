extern crate clap;

use clap::{Arg, App};

mod utils;
mod day1;
mod day2;
mod day3;
mod day4;

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
            println!("Day 1");
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
        "2" =>
        {
            println!("Day 2");
            match part
            {
                "1" => day2::part1(),
                "2" => day2::part2(),
                "all" =>
                {
                    day2::part1();
                    day2::part2();
                },
                _ => ()
            }
        },
        "3" =>
        {
            println!("Day 3");
            match part
            {
                "1" => day3::part1(),
                "2" => day3::part2(),
                "all" =>
                {
                    day3::part1();
                    day3::part2();
                },
                _ => ()
            }
        }
        "4" =>
        {
            println!("Day 4");
            match part
            {
                "1" => day4::part1(),
                "2" => day4::part2(),
                "all" =>
                {
                    day4::part1();
                    day4::part2();
                },
                _ => ()
            }
        },
        "all" =>
        {
            println!("Day 1");
            day1::part1();
            day1::part2();

            println!("Day 2");
            day2::part1();
            day2::part2();

            println!("Day 3");
            day3::part1();
            day3::part2();

            println!("Day 4");
            day4::part1();
            day4::part2();
        }
        _ => ()
    }
}
