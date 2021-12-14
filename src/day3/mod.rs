use crate::utils;

pub fn part1()
{
    if let Ok(lines) = utils::read_lines("./src/day3/input")
    {
        let mut ones: [u32; 12] = [0; 12];
        let mut zeros: [u32; 12] = [0; 12];
        for line in lines
        {
            if let Ok(l) = line
            {
                for (i, c) in l.chars().enumerate()
                {
                    if c == '0'
                    {
                        zeros[i] += 1;
                    }
                    else if c == '1'
                    {
                        ones[i] += 1;
                    }
                }
            }
        }

        let mut gamma: u32 = 0;
        let mut epsilon: u32 = 0;

        for i in 0..12
        {
            if ones[i] > zeros[i]
            {
                gamma += 1 << (11 - i);
            }
            else if ones[i] < zeros[i]
            {
                epsilon += 1 << (11 - i);
            }
        }

        println!("part 1: {}", gamma * epsilon);
    }
}
