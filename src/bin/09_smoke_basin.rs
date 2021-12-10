use std::fs;

fn is_min(height: &Vec<Vec<u32>>, row: usize, col: usize) -> bool {
    let rows = height.len() as i32;
    let cols = height[0].len() as i32;

    let cur = height[row][col];
    let offsets: Vec<(i32, i32)> = vec![(-1, 0), (1, 0), (0, -1), (0, 1)];
    for (rr, cc) in offsets {
        let n_row = (row as i32) + rr;
        let n_col = (col as i32) + cc;
        if n_row < 0 || n_row >= rows {
            continue;
        }
        if n_col < 0 || n_col >= cols {
            continue;
        }
        if height[n_row as usize][n_col as usize] <= cur {
            return false;
        }
    }

    return true;
}

fn find_low_points(height: &Vec<Vec<u32>>) -> Vec<(usize, usize)> {
    let rows = height.len();
    let cols = height[0].len();

    let mut low_points = Vec::new();
    for row in 0..rows {
        for col in 0..cols {
            if is_min(height, row, col) {
                low_points.push((row, col));
            }
        }
    }


    low_points
}

/// Return a vector of basin sizes
fn find_basins(original_height: &Vec<Vec<u32>>, cliff_id: u32) -> Vec<usize> {
    // Operate on a mutable copy where we can scribble where we've been.
    let mut height = original_height.to_vec();
    // The cliff values are meant to be the highest available.
    let marker = cliff_id + 1;
    let mut basin_sizes = Vec::new();

    let rows = height.len();
    let cols = height[0].len();

    for row in 0..rows {
        for col in 0..cols {
            if height[row][col] == marker || height[row][col] == cliff_id {
                continue;
            }

            // We're on a regular tile - time to search for a basin. BFS time!
            let mut queue: Vec<(usize, usize)> = Vec::new();
            let mut size: usize = 1;
            queue.push((row, col));
            height[row as usize][col as usize] = marker;


            while ! queue.is_empty() {
                let (c_row, c_col) = queue.pop().unwrap();

                let offsets: Vec<(i32, i32)> = vec![(-1, 0), (1, 0), (0, -1), (0, 1)];
                for (rr, cc) in offsets {
                    let n_row = (c_row as i32) + rr;
                    let n_col = (c_col as i32) + cc;
                    if n_row < 0 || n_row >= rows as i32 {
                        continue;
                    }
                    if n_col < 0 || n_col >= cols as i32 {
                        continue;
                    }
                    let n_height = height[n_row as usize][n_col as usize];
                    if n_height != cliff_id && n_height != marker {
                        queue.push((n_row as usize, n_col as usize));
                        size += 1;
                        // Scribble the tile as visited.
                        height[n_row as usize][n_col as usize] = marker;
                    }
                }
            }

            basin_sizes.push(size);
        }
    }

    basin_sizes
}

fn main() {
    let data = fs::read_to_string("input/09.txt").expect("Unable to read file");
    // let data = fs::read_to_string("input/09-demo.txt").expect("Unable to read file");

    let mut height: Vec<Vec<u32>> = Vec::new();
    let zero_char = '0' as u32;
    for row in data.split("\n") {
        let entries: Vec<u32> = row.chars().map(|x| u32::from(x) - zero_char).collect();
        height.push(entries);
    }

    let low_point_coords = find_low_points(&height);
    let mut sum = 0u32;
    for (row, col) in low_point_coords {
        let risk = 1u32 + height[row][col];
        sum += risk;
    }

    println!("Part 1: {:?}", sum);

    let mut basin_sizes = find_basins(&height, 9u32);
    basin_sizes.sort_by(|a, b| b.partial_cmp(a).unwrap());
    if basin_sizes.len() < 3 {
        panic!("Insufficient basins found! Need at least 3 but found {}.", basin_sizes.len());
    }
    println!("Part 2:");
    println!("Found {} basins.", basin_sizes.len());
    println!("Basin sizes:\n{:?}", basin_sizes);
    let basin_score = basin_sizes[0] * basin_sizes[1] * basin_sizes[2];
    println!("Score = {}", basin_score);
}
