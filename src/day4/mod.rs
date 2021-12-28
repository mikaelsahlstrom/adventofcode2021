use crate::utils;

pub fn part1()
{
    if let Ok(mut lines) = utils::read_lines("./src/day4/input")
    {
        // Parse numbers to draw.
        let draw: Vec<u32>;
        if let Some(Ok(l)) = lines.next()
        {
            draw = l.split(",").map(|n| n.parse().unwrap()).collect();
        }
        else
        {
            panic!("Draw not nice.");
        }

        let mut boards: Vec<[[u32; 7]; 6]> = Vec::new();
        let mut board: [[u32; 7]; 6] = [[0; 7]; 6];
        let mut row = 0;

        // Parse boards.
        for line in lines
        {
            if let Ok(l) = line
            {
                if l != ""
                {
                    if row == 5
                    {
                        boards.push(board);
                        row = 0;
                        board = [[0; 7]; 6];
                    }

                    for (i, num) in l.split_whitespace().enumerate()
                    {
                        let new_num: u32 = num.parse().unwrap();
                        board[row][i] = new_num;
                        board[row][6] += new_num;
                    }

                    row += 1;
                }
            }
        }

        // Add last board.
        boards.push(board);

        for num in draw
        {
            for board in &mut boards
            {
                for i in 0..5
                {
                    for j in 0..5
                    {
                        if board[i][j] == num
                        {
                            board[i][5] += 1;
                            board[5][j] += 1;

                            board[i][6] -= num;

                            if board[i][5] == 5 || board[5][j] == 5
                            {
                                let mut sum: u32 = 0;
                                for n in 0..6
                                {
                                    sum += board[n][6];
                                }

                                println!("part 1: {}", sum * num);
                                return;
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn part2()
{
    if let Ok(mut lines) = utils::read_lines("./src/day4/input")
    {
        // Parse numbers to draw.
        let draw: Vec<u32>;
        if let Some(Ok(l)) = lines.next()
        {
            draw = l.split(",").map(|n| n.parse().unwrap()).collect();
        }
        else
        {
            panic!("Draw not nice.");
        }

        let mut boards: Vec<[[u32; 7]; 6]> = Vec::new();
        let mut board: [[u32; 7]; 6] = [[0; 7]; 6];
        let mut row = 0;

        // Parse boards.
        for line in lines
        {
            if let Ok(l) = line
            {
                if l != ""
                {
                    if row == 5
                    {
                        boards.push(board);
                        row = 0;
                        board = [[0; 7]; 6];
                    }

                    for (i, num) in l.split_whitespace().enumerate()
                    {
                        let new_num: u32 = num.parse().unwrap();
                        board[row][i] = new_num;
                        board[row][6] += new_num;
                    }

                    row += 1;
                }
            }
        }

        // Add last board.
        boards.push(board);

        for num in draw
        {
            for board in &mut boards
            {
                for i in 0..5
                {
                    for j in 0..5
                    {
                        if board[i][j] == num
                        {
                            board[i][5] += 1;
                            board[5][j] += 1;

                            board[i][6] -= num;
                        }
                    }
                }
            }

            if boards.len() > 1
            {
                boards.retain(|b| !b.iter().any(|v| v[5] == 5) && !b[5].iter().any(|v| *v == 5));
            }
            else if boards[0].iter().any(|v| v[5] == 5) || boards[0][5].iter().any(|v| *v == 5)
            {
                let mut sum: u32 = 0;
                for n in 0..6
                {
                    println!("{:?}", boards[0][n]);
                    sum += boards[0][n][6];
                }

                println!("part 2: {}", sum * num);
                return;
            }
        }

    }
}
