use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::str::FromStr;

#[derive(Debug)]
struct Board {
    values: [[u32; 5]; 5],
    seen: [[bool; 5]; 5],
}

impl Board {
    fn set(&mut self, val: u32) {
        for i in 0..5 {
            for j in 0..5 {
                if self.values[i][j] == val {
                    self.seen[i][j] = true;
                }
            }
        }
    }

    /// Checks whether there is a BINGO on the board.
    fn has_bingo(&self) -> bool {
        // Rows
        for i in 0..5 {
            let mut row_sum = 0u32;
            for j in 0..5 {
                if self.seen[i][j] {
                    row_sum += 1;
                }
            }
            if row_sum == 5 {
                return true;
            }
        }

        // Columns
        for j in 0..5 {
            let mut col_sum = 0u32;
            for i in 0..5 {
                if self.seen[i][j] {
                    col_sum += 1;
                }
            }
            if col_sum == 5 {
                return true;
            }
        }

        false
    }

    /// The sum of all unmarked numbers.
    fn sum_unmarked(&self) -> u32 {
        let mut sum = 0u32;
        for i in 0..5 {
            for j in 0..5 {
                if !self.seen[i][j] {
                    sum += self.values[i][j];
                }
            }
        }
        sum
    }

    /// Calls out a drawn number, returning true if BINGO was triggered.
    fn new_draw(&mut self, val: u32) -> bool {
        self.set(val);
        self.has_bingo()
    }
}

fn day_04_squid_bingo() {
    // let input_path = Path::new("input/04-demo.txt");
    let input_path = Path::new("input/04.txt");
    let mut raw_lines = Vec::new();

    if let Ok(lines) = read_lines(input_path) {
        for line in lines {
            if let Ok(line_str) = line {
                raw_lines.push(line_str);
            }
        }
    }

    let draws: Vec<u32> = raw_lines[0]
        .split(",")
        .map(|x| u32::from_str(x).unwrap())
        .collect();
    let n_boards = (raw_lines.len() - 1) / 6;
    println!("Will parse {} boards.", n_boards);

    let mut boards = Vec::new();
    for board_idx in 0..n_boards {
        let mut values = [[0u32; 5]; 5];
        let seen = [[false; 5]; 5];
        let mut row_idx = 0;
        for row_idx in 0..5 {
            let raw_row = &raw_lines[2 + board_idx * 6 + row_idx];
            let row_vals: Vec<u32> = raw_row
                .split(" ")
                .map(|x| x.trim())
                .filter(|x| x.len() > 0)
                .map(|x| u32::from_str(x).unwrap())
                .collect();
            // TODO(andrei): Can we do this parsing in a cleaner way?
            for col_idx in 0..5 {
                values[row_idx][col_idx] = row_vals[col_idx];
            }
        }
        let mut board = Board {
            values: values,
            seen: seen,
        };
        boards.push(board);
    }

    let mut won = Vec::new();
    let mut n_won = (0 as usize);
    let n_boards = boards.len();
    for board in &mut boards {
        won.push(false);
    }

    for draw in draws {
        for (board_idx, board) in (&mut boards).into_iter().enumerate() {
            if board.new_draw(draw) {
                if !won[board_idx] {
                    println!("Board {} just won!", board_idx);
                    if n_won == 0 {
                        let result = board.sum_unmarked() * draw;
                        println!("Part 1 result: {}", result);
                    }
                    if n_won == n_boards - 1 {
                        println!("Last winner.");
                        let result = board.sum_unmarked() * draw;
                        println!("Part 2 result: {}", result);
                    }

                    won[board_idx] = true;
                    n_won += 1;
                }
            }
        }
    }
}

fn main() {
    day_04_squid_bingo();
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
