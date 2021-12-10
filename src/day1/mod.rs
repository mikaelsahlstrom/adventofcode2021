use crate::utils;

pub fn part1()
{
    if let Ok(lines) = utils::read_lines("./src/day1/input")
    {
        let mut amount: u32 = 0;
        let mut last: u32 = std::u32::MAX;

        for line in lines
        {
            if let Ok(l) = line
            {
                let curr: u32 = l.parse().unwrap();
                if curr > last
                {
                    amount += 1;
                }
                last = curr;
            }
        }

        println!("part 1: {}", amount);
    }
}

pub fn part2()
{
    if let Ok(lines) = utils::read_lines("./src/day1/input")
    {
        let mut buf: [u32; 4] = [0; 4];
        let mut amount: u32 = 0;
        for (i, line) in lines.enumerate()
        {
            if let Ok(l) = line
            {
                if i > 3
                {
                    let sum1: u32 = buf[0] + buf[1] + buf[2];
                    let sum2: u32 = buf[1] + buf[2] + buf[3];
                    if sum2 > sum1
                    {
                        amount += 1;
                    }

                    buf[0] = buf[1];
                    buf[1] = buf[2];
                    buf[2] = buf[3];
                    buf[3] = l.parse().unwrap();
                }
                else
                {
                    buf[i] = l.parse().unwrap();
                }
            }
        }

        let sum1: u32 = buf[0] + buf[1] + buf[2];
        let sum2: u32 = buf[1] + buf[2] + buf[3];
        if sum2 > sum1
        {
            amount += 1;
        }

        println!("part 2: {}", amount);
    }
}
