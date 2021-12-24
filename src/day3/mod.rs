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

pub fn part2()
{
    if let Ok(lines) = utils::read_lines("./src/day3/input")
    {
        let mut values: Vec<std::string::String> = Vec::new();

        // Read all lines.
        for line in lines
        {
            if let Ok(l) = line
            {
                // Store value.
                values.push(l);
            }
        }

        let mut i = 0;
        let mut oxygen_values: Vec<&std::string::String> = values.iter().collect();
        while oxygen_values.len() > 1
        {
            // Calculate amount of ones and zeros for position i.
            let mut ones: u32 = 0;
            let mut zeros: u32 = 0;
            for v in &oxygen_values
            {
                if v.chars().nth(i).unwrap() == '1'
                {
                    ones += 1;
                }
                else
                {
                    zeros += 1;
                }
            }

            if ones > zeros || ones == zeros
            {
                // Keep 1.
                oxygen_values = oxygen_values.into_iter().filter(|v| v.chars().nth(i).unwrap() == '1').collect();
            }
            else
            {
                // Keep 0.
                oxygen_values = oxygen_values.into_iter().filter(|v| v.chars().nth(i).unwrap() == '0').collect();
            }

            i += 1;
        }

        i = 0;
        let mut co2_values: Vec<&std::string::String> = values.iter().collect();
        while co2_values.len() > 1
        {
            // Calculate amount of ones and zeros for position i.
            let mut ones: u32 = 0;
            let mut zeros: u32 = 0;
            for v in &co2_values
            {
                if v.chars().nth(i).unwrap() == '1'
                {
                    ones += 1;
                }
                else
                {
                    zeros += 1;
                }
            }

            if zeros < ones || zeros == ones
            {
                // Keep 0.
                co2_values = co2_values.into_iter().filter(|v| v.chars().nth(i).unwrap() == '0').collect();
            }
            else
            {
                // Keep 1.
                co2_values = co2_values.into_iter().filter(|v| v.chars().nth(i).unwrap() == '1').collect();
            }

            i += 1;
        }

        let oxygen: u32 = u32::from_str_radix(oxygen_values[0], 2).unwrap();
        let co2: u32 = u32::from_str_radix(co2_values[0], 2).unwrap();

        println!("part 2: {}", oxygen * co2);
    }
}
