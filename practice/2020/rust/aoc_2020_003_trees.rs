use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;


#[derive(Debug)]
struct Toboggan {
    step_right: usize,
    step_down: usize,
}

const TREE: char = '#';

/// Solves Day 3 Part 1 by counting trees encountered going down with the given toboggan.
///
/// Part of the Part 2 solution, where we just call the this multiple times with multiple toboggan configurations.
fn whee(toboggan: &Toboggan, trees: &Vec<String>) -> usize {
    let mut cur_col: usize = 0;
    let mut cur_row: usize = 0;
    let mut tree_count: usize = 0;
    let height: usize = trees.len();
    let block_width: usize = trees[0].len();

    // I *feel* like doing .chars() all the time may be slow in Rust, but I'm not 100% sure.
    while cur_row < height {
        let cur_char = match trees[cur_row].chars().nth(cur_col) {
            Some(ch) => ch,
            None => panic!("WTF")
        };
        if cur_char == TREE {
            tree_count += 1;
        }
        cur_row += toboggan.step_down;
        cur_col = (cur_col + toboggan.step_right) % block_width;
    }

    tree_count
}


fn day_03_trees() {
    let input_path = Path::new("../input/003-input.txt");

    let mut trees: Vec<String> = Vec::new();

    if let Ok(lines) = read_lines(input_path) {
        for line in lines {
            if let Ok(line_str) = line {
                trees.push(line_str);
            }
        }
    }

    let tobs = [
        Toboggan{step_right: 1, step_down: 1},
        Toboggan{step_right: 3, step_down: 1},
        Toboggan{step_right: 5, step_down: 1},
        Toboggan{step_right: 7, step_down: 1},
        Toboggan{step_right: 1, step_down: 2},
    ];
    let mut cur: usize = 1;
    for tob in tobs {
        let tob_trees = whee(&tob, &trees);
        println!("{:?} would hit {:} trees.", tob, tob_trees);
        cur *= tob_trees;
    }
    println!("{}", cur);
}


fn main() {
    day_03_trees()
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}