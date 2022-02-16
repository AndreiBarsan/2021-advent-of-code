/// 2021 AoC Day 25: Sea Cucumber
use std::fs;

const EAST: char = '>';
const SOUTH: char = 'v';
const EMPTY: char = '.';

/// State update which applies the rules of the sea cucumber simulation. Returns the new state and # of moved cucumbers.
fn step(cucumbers: &Vec<Vec<char>>) -> (Vec<Vec<char>>, usize) {
    let n_rows = cucumbers.len();
    let n_cols = cucumbers[0].len();
    let mut moved_east: usize = 0;
    let mut moved_south: usize = 0;

    // NOTE(andrei): I can pre-allocate the intermediate buffers if further speed is necessary.
    let mut next_state: Vec<Vec<char>> = Vec::new();
    for _ in 0..n_rows {
        next_state.push(vec![EMPTY; n_cols]);
    }

    // As per the problem specification, East-facing sea cucumbers move first.
    for row_idx in 0..n_rows {
        for col_idx in 0..n_cols {
            if cucumbers[row_idx][col_idx] == EAST {
                let next_col = (col_idx + 1) % n_cols;
                if cucumbers[row_idx][next_col] == EMPTY {
                    next_state[row_idx][next_col] = EAST;
                    moved_east += 1;
                } else {
                    next_state[row_idx][col_idx] = EAST;
                }
            }
        }
    }

    let mut next_next_state = Vec::new();
    for _ in 0..n_rows {
        next_next_state.push(vec![EMPTY; n_cols]);
    }

    // As per the problem specification, South-facing sea cucumbers move second.
    for row_idx in 0..n_rows {
        for col_idx in 0..n_cols {
            if cucumbers[row_idx][col_idx] == SOUTH {
                let next_row = (row_idx + 1) % n_rows;
                if next_state[next_row][col_idx] == EMPTY && cucumbers[next_row][col_idx] != SOUTH {
                    next_next_state[next_row][col_idx] = SOUTH;
                    moved_south += 1;
                } else {
                    next_next_state[row_idx][col_idx] = SOUTH;
                }
            }
        }
    }
    for row_idx in 0..n_rows {
        for col_idx in 0..n_cols {
            if next_state[row_idx][col_idx] == EAST {
                next_next_state[row_idx][col_idx] = EAST;
            }
        }
    }

    let moved_total = moved_east + moved_south;

    (next_next_state, moved_total)
}

fn print_cucumbers(cucumbers: &Vec<Vec<char>>) {
    for row in cucumbers {
        let row_str: String = row.iter().collect();
        println!("{}", row_str);
    }
}

fn day_25_sea_cucumber() {
    let input_fname = "input/25.txt";
    // let input_fname = "input/25-demo.txt";

    let data: Vec<Vec<char>> = fs::read_to_string(input_fname)
        .expect("Unable to read file.")
        .split("\n")
        .map(|x| x.to_string().chars().collect())
        .collect();

    println!("{} x {}", data.len(), data[0].len());
    print_cucumbers(&data);

    let mut state = data;
    let max_steps = 100000;

    for step_idx in 0..max_steps {
        let (mut new_state, mut n_moved) = step(&state);
        // println!("");
        // print_cucumbers(&new_state);
        state = new_state;
        if n_moved == 0 {
            println!("Zero moves at step {}. Stopping.", step_idx + 1);
            break;
        }

        if step_idx > 0 && step_idx % 1000 == 0 {
            println!("Step {} / max {}, {} moves", step_idx, max_steps, n_moved);
        }
    }

    println!("Final state:");
    print_cucumbers(&state);
}

fn main() {
    day_25_sea_cucumber();
}
