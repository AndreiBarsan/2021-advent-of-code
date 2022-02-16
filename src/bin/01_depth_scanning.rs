use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::str::FromStr;

fn main() {
    // Note: This path is relative to the repo root
    let input_path = Path::new("input/01.txt");

    let mut prev: i32 = 99999999;
    let mut inc: i32 = 0;
    let mut inc_sum: i32 = 0;

    let mut nrs: i32 = 0;
    let mut q: [i32; 3] = [0i32, 0i32, 0i32];
    let mut prev_sum: i32 = 0;

    if let Ok(lines) = read_lines(input_path) {
        for maybe_line in lines {
            if let Ok(line) = maybe_line {
                // Part 1 of the problem
                let cur = i32::from_str(&line).unwrap();
                if cur > prev {
                    inc += 1;
                }
                prev = cur;

                // Part 2 of the problem
                for ii in 0..2 {
                    q[ii] = q[ii + 1];
                }
                q[2] = cur;
                let cur_sum = q.iter().sum();

                nrs += 1;
                if nrs >= 4 {
                    if cur_sum > prev_sum {
                        inc_sum += 1;
                    }
                }
                prev_sum = cur_sum;
            }
        }
    }

    println!("Final result {}", inc);
    println!("Final result {}", inc_sum);
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
