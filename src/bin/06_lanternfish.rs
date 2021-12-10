use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::str::FromStr;

const NEW_FISH_OFFSET: usize = 2usize;
const SPAWN_INTERVAL: usize = 6usize;
const MAX_AGE: usize = SPAWN_INTERVAL + NEW_FISH_OFFSET;

/// Simulates the initial state for the given number of step and returns the final number of fish.
fn simulate(initial_state: &Vec<usize>, max_sim_steps: usize) -> usize {

    let mut fish_by_age: [usize; MAX_AGE + 1] = [0; MAX_AGE + 1];
    for &fish in initial_state {
        fish_by_age[fish] += 1;
    }

    for _ in 0..max_sim_steps {
        let new_fish = fish_by_age[0];
        let new_reset = fish_by_age[0];

        for idx in 0..MAX_AGE {
            fish_by_age[idx] = fish_by_age[idx + 1];
        }
        fish_by_age[MAX_AGE] = new_fish;
        fish_by_age[SPAWN_INTERVAL] += new_reset;
        // println!("{:?}", fish_by_age);
    }

    let n_fish: usize = fish_by_age.into_iter().sum();
    n_fish
}

fn day_06_lanternfish() {
    let input_path = Path::new("input/06.txt");
    // let input_path = Path::new("input/06-demo.txt");

    let mut data: String = "".to_string();
    if let Ok(lines) = read_lines(input_path) {
        for line in lines {
            if let Ok(line_str) = line {
                data = line_str;
            }
        }
    }

    let initial_state: Vec<usize> = data.split(",").map(|x| usize::from_str(x).unwrap()).collect();

    let n_fish_part_1 = simulate(&initial_state, 80usize);
    println!("Fish after {} days: {}", 80usize, n_fish_part_1);

    let n_fish_part_2 = simulate(&initial_state, 256usize);
    println!("Fish after {} days: {}", 256usize, n_fish_part_2);

    // Naive solution
    //
    // for sim_step in 0..max_sim_steps {
    //     let mut n_babies = 0u32;

    //     for fish_idx in 0..state.len() {
    //         let fish_val = state[fish_idx];
    //         if 0 == fish_val {
    //             n_babies += 1;
    //             state[fish_idx] = SPAWN_INTERVAL;
    //         }
    //         else {
    //             state[fish_idx] = fish_val - 1;
    //         }
    //     }

    //     for _ in 0..n_babies {
    //         state.push(SPAWN_INTERVAL + NEW_FISH_OFFSET);
    //     }

    //     if sim_step < 18 {
    //         println!("{:?}", state);
    //     }
    // }
    // println!("Fish after {} days: {}", max_sim_steps, state.len());
}


fn main() {
    day_06_lanternfish();
}


fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
