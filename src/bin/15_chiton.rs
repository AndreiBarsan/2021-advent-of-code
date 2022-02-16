use std::cmp::Ordering;
use std::collections::BinaryHeap;
/// 2021 AoC Day 15: Chiton
///
/// Basically just Dijkstra's on a grid.
use std::fs;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
struct Candidate {
    cost: u32,
    prev: (usize, usize),
}

/// Based on the example from the rust book, adapted to work with grid graphs.
impl Ord for Candidate {
    fn cmp(&self, other: &Self) -> Ordering {
        // Reverse order so we create a min-heap - we always want to pop the candidate with the smallest cost first.
        other
            .cost
            .cmp(&self.cost)
            .then_with(|| self.prev.0.cmp(&other.prev.0))
            .then_with(|| self.prev.1.cmp(&other.prev.1))
    }
}

impl PartialOrd for Candidate {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

/// Finds the lowest-cost path from 'start' to 'end' using Dijkstra's algorithm.
///
/// Assumes all sub-vectors of 'grid' are the same length. The graph is a lattice defined by 'grid' using a 4-neighbor
/// connectivity pattern.
fn get_path(
    start: (usize, usize),
    end: (usize, usize),
    grid: &Vec<Vec<u32>>,
) -> Vec<(usize, usize)> {
    let max_cost: u32 = u32::MAX;
    let undefined: (usize, usize) = (usize::MAX, usize::MAX);
    let rows = grid.len();
    let cols = grid[0].len();

    let mut cost: Vec<Vec<u32>> = Vec::new();
    let mut prev: Vec<Vec<(usize, usize)>> = Vec::new();
    let mut queue = BinaryHeap::new();

    for _ in 0..rows {
        cost.push(vec![max_cost; cols]);
        prev.push(vec![undefined; cols]);
    }
    queue.push(Candidate {
        cost: 0,
        prev: start,
    });
    cost[start.0][start.1] = 0;

    while let Some(candidate) = queue.pop() {
        let r = candidate.prev.0 as i32;
        let c = candidate.prev.1 as i32;
        for (rr, cc) in [(r - 1, c), (r + 1, c), (r, c - 1), (r, c + 1)] {
            if rr >= 0 && rr < rows as i32 && cc >= 0 && cc < cols as i32 {
                let next = Candidate {
                    cost: candidate.cost + grid[rr as usize][cc as usize],
                    prev: (rr as usize, cc as usize),
                };

                if next.cost < cost[next.prev.0][next.prev.1] {
                    queue.push(next);
                    cost[next.prev.0][next.prev.1] = next.cost;
                    prev[next.prev.0][next.prev.1] = (r as usize, c as usize);
                }
            }
        }
    }

    let mut result_path: Vec<(usize, usize)> = Vec::new();
    let mut cur = end.clone();
    while cur != undefined {
        result_path.push(cur);
        cur = prev[cur.0][cur.1];
    }

    result_path
}

/// Computes the cost of 'path' over 'grid', not including the start position.
///
/// Decoupled from the main Dijkstra for simplicity.
fn cost(path: &Vec<(usize, usize)>, grid: &Vec<Vec<u32>>) -> u32 {
    path.iter().rev().map(|(r, c)| grid[*r][*c]).skip(1).sum()
}

/// A version of modulo that is 1-based - wraps numbers into the range [1..max[
fn wrap(x: u32, max: u32) -> u32 {
    (x - 1) % (max - 1) + 1
}

/// Returns 'grid' tiled 'factor' times in X and in Y, with increasing values in more distant clones.
fn enlarge_grid(grid: &Vec<Vec<u32>>, factor: usize) -> Vec<Vec<u32>> {
    let rows = grid.len();
    let cols = grid[0].len();

    let mut enlarged_grid = Vec::new();
    for r in 0..(factor * rows) {
        enlarged_grid.push(vec![0; factor * cols]);
    }

    for chunk_r in 0..factor {
        for chunk_c in 0..factor {
            let offset = (chunk_r + chunk_c) as u32;
            for rr in 0..rows {
                for cc in 0..cols {
                    let original_x = grid[rr][cc];
                    enlarged_grid[chunk_r * rows + rr][chunk_c * cols + cc] =
                        wrap(original_x + offset, 10u32);
                }
            }
        }
    }

    enlarged_grid
}

fn day_15_chiton() {
    let input_fname = "input/15.txt";
    // let input_fname = "input/15-demo.txt";

    // Input data processing
    let data: Vec<String> = fs::read_to_string(input_fname)
        .expect("Unable to read file.")
        .split("\n")
        .map(|x| x.to_string())
        .collect();

    let mut grid: Vec<Vec<u32>> = Vec::new();
    for row in data {
        let entries: Vec<u32> = row.chars().map(|x| (x as u32) - ('0' as u32)).collect();
        grid.push(entries);
    }

    println!("Input grid:");
    // Pretty prints the small grid
    for row in &grid {
        let row_str: String = row
            .iter()
            .map(|x| char::from_digit(*x, 10).unwrap())
            .collect();
        println!("{}", row_str);
    }

    let end = (grid.len() - 1, grid[0].len() - 1);
    let path = get_path((0usize, 0usize), end, &grid);

    // This was bugged because my cost was evaluating the path in the wrong direction, so it was dropping the part of
    // the cost resulting from 'end', when it should have been dropping it for 'start'.
    let part_1_cost = cost(&path, &grid);
    println!("Part 1 result: {}", part_1_cost);

    println!("Part 2:");
    let big_grid = enlarge_grid(&grid, 5usize);
    // println!("{:?}", big_grid);
    let big_end = (big_grid.len() - 1, big_grid[0].len() - 1);
    let big_path = get_path((0usize, 0usize), big_end, &big_grid);
    let part_2_cost = cost(&big_path, &big_grid);
    println!("Part 2 result: {}", part_2_cost);
}

fn main() {
    day_15_chiton();
}
