use std::fs;
use std::collections::HashMap;


/// Returns the number of flashes that occur after n_steps given the initial condition.
fn simulate_octopi(initial_octopi: &Vec<Vec<u32>>, n_steps: u32) -> u32 {
    let mut octopi = initial_octopi.to_vec();
    let mut flashed = initial_octopi.to_vec();
    let mut total_flashes: u32 = 0;
    let max_energy: u32 = 9;
    let rows = octopi.len();
    let cols = octopi[0].len();

    for step in 0..n_steps {
        // println!("Step: {}", step);
        // if step < 10 {
        //     println!("Before: {:?}", octopi);
        // }
        // Base increase
        for row in 0..rows {
            for col in 0..cols {
                octopi[row][col] += 1;
                flashed[row][col] = 0;
            }
        }

        let mut flashes_this_step = 0u32;

        // Flashing
        loop {
            let mut new_flashes: u32 = 0;
            for row in 0..rows {
                for col in 0..cols {
                    if flashed[row][col] == 1u32 {
                        continue;
                    }

                    let mut boost = 0;
                    if octopi[row][col] > max_energy {
                        boost = 1;
                        new_flashes += 1;
                        flashed[row][col] = 1;
                    }

                    for rr in -1..2 {
                        for cc in -1..2 {
                            let new_r = ((row as i32) + rr) as usize;
                            let new_c = ((col as i32) + cc) as usize;
                            if new_r >= 0 && new_r < rows && new_c >= 0 && new_c < cols {
                                octopi[new_r][new_c] += boost;
                            }
                        }
                    }

                }
            }

            flashes_this_step += new_flashes;

            // Loop until stable
            if new_flashes == 0u32 {
                break;
            }

        }

        total_flashes += flashes_this_step;

        // Reset
        for row in 0..rows {
            for col in 0..cols {
                if octopi[row][col] > max_energy {
                    octopi[row][col] = 0u32;
                }
            }
        }

        if flashes_this_step as usize == rows * cols {
            println!("Part 2: Mega-flash at step {}", step + 1);
            break;
        }

    }

    total_flashes
}


fn day_11_dumbo() {
    // let data = fs::read_to_string("input/11-demo.txt").expect("Unable to read file.");
    let data = fs::read_to_string("input/11.txt").expect("Unable to read file.");
    let n_steps_part_1: u32 = 100;
    let n_steps_part_2: u32 = 100000;

    let mut octo: Vec<Vec<u32>> = Vec::new();
    let zero_char = '0' as u32;
    for row in data.split("\n") {
        if row.len() <= 2 {
            continue;
        }
        let entries: Vec<u32> = row.chars().map(|x| u32::from(x) - zero_char).collect();
        octo.push(entries);
    }

    let n_flashes = simulate_octopi(&octo, n_steps_part_1);
    let _ = simulate_octopi(&octo, n_steps_part_2);

    println!("{:?}", octo);
    println!("{:?}", n_flashes);
}

fn main() {
    day_11_dumbo();
}