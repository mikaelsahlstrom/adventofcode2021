use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

pub fn part1()
{
    if let Ok(lines) = read_lines("./src/day2/input")
    {
        let mut h_pos: u32 = 0;
        let mut d_pos: u32 = 0;

        for line in lines
        {
            if let Ok(l) = line
            {
                let mut values = l.split_whitespace();
                match values.next()
                {
                    Some("up") =>
                    {
                        match values.next()
                        {
                            Some(x) => d_pos -= x.parse::<u32>().unwrap(),
                            None => ()
                        }
                    },
                    Some("down") =>
                    {
                        match values.next()
                        {
                            Some(x) => d_pos += x.parse::<u32>().unwrap(),
                            None => ()
                        }
                    },
                    Some("forward") =>
                    {
                        match values.next()
                        {
                            Some(x) => h_pos += x.parse::<u32>().unwrap(),
                            None => ()
                        }
                    }
                    Some(&_) => (),
                    None => ()
                }
            }
        }

        println!("part 1: {}", h_pos * d_pos);
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
