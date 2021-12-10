use crate::utils;

pub fn part1()
{
    if let Ok(lines) = utils::read_lines("./src/day2/input")
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

pub fn part2()
{
    if let Ok(lines) = utils::read_lines("./src/day2/input")
    {
        let mut h_pos: u32 = 0;
        let mut d_pos: u32 = 0;
        let mut aim: u32 = 0;

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
                            Some(x) => aim -= x.parse::<u32>().unwrap(),
                            None => ()
                        }
                    },
                    Some("down") =>
                    {
                        match values.next()
                        {
                            Some(x) => aim += x.parse::<u32>().unwrap(),
                            None => ()
                        }
                    },
                    Some("forward") =>
                    {
                        match values.next()
                        {
                            Some(x) =>
                            {
                                let v = x.parse::<u32>().unwrap();
                                h_pos += v;
                                d_pos += aim * v;
                            },
                            None => ()
                        }
                    }
                    Some(&_) => (),
                    None => ()
                }
            }
        }

        println!("part 2: {}", h_pos * d_pos);
    }
}
