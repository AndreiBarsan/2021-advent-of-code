/// 2021 AoC Day 13
///
/// Transparent paper folding - start with a big sheet with random-looking sparse dots, and keep folding along specified
/// lines until you are through and a specific pattern of big letters emerges.
use std::fs;
use std::collections::HashSet;
use std::str::FromStr;


#[derive(Debug)]
struct Fold {
    dim: String,
    idx: usize
}

#[derive(Debug)]
struct Paper {
    width: usize,
    height: usize,
    dots: HashSet<(usize, usize)>,
}

fn parse_dots(dot_specs: &[&str]) -> HashSet<(usize, usize)> {
    let mut dots = HashSet::new();
    for spec in dot_specs {
        let parts: Vec<&str> = spec.split(",").collect();
        let xx = usize::from_str(parts[0]).expect("dot spec int parse failed");
        let yy = usize::from_str(parts[1]).expect("dot spec int parse failed");
        dots.insert((xx, yy));
    }
    dots
}

fn parse_folds(fold_specs: &[&str]) -> Vec<Fold> {
    let mut folds = Vec::new();
    for fold_spec in fold_specs {
        let parts: Vec<&str> = fold_spec.split(" ").collect::<Vec<&str>>()[2usize].split("=").collect();
        let lhs = parts[0];
        let rhs = parts[1];

        folds.push(Fold{ dim: lhs.to_string(), idx: usize::from_str(rhs).expect("") });
    }

    folds
}

fn fold_dot(dot: &(usize, usize), fold: &Fold) -> (usize, usize) {
    let (mut xx, mut yy) = dot;
    if fold.dim == "x" {
        if xx > fold.idx {
            xx = fold.idx - (xx - fold.idx);
        }
        (xx, yy)
    }
    else if fold.dim == "y" {
        if yy > fold.idx {
            yy = fold.idx - (yy - fold.idx);
        }
        (xx, yy)
    }
    else {
        panic!("Invalid fold dimension: {:?}", fold.dim);
    }
}

/// Returns the page resulting from the fold of 'dots' with 'fold'.
fn fold_dots(dots: &HashSet<(usize, usize)>, fold: &Fold) -> HashSet<(usize, usize)> {
    dots.iter().map(|dot| fold_dot(dot, fold)).collect()
}

fn day_13_origami() {
    let data = fs::read_to_string("input/13.txt").expect("Unable to read file.");
    // let data = fs::read_to_string("input/13-demo.txt").expect("Unable to read file.");
    let in_lines: Vec<&str> = data.split("\n").collect();

    let blank_line = in_lines.iter().position(|l| l.len() == 0).expect("yep");
    println!("{:?}", blank_line);

    let dot_lines = &in_lines[..blank_line];
    let fold_instruction_lines = &in_lines[(blank_line + 1)..];

    let dots: HashSet<(usize, usize)> = parse_dots(dot_lines);
    let mut width = dots.iter().map(|dot| dot.1).max().unwrap();
    let mut height = dots.iter().map(|dot| dot.0).max().unwrap();

    let folds: Vec<Fold> = parse_folds(fold_instruction_lines);
    // println!("{:?}", dots);
    // println!("{:?}", folds);

    let part_1_board = fold_dots(&(dots.clone()), &folds[0]);
    let part_1_answer = part_1_board.len();
    println!("Part 1: {:?} dots after the first fold", part_1_answer);

    println!("Part 2:");
    println!("Before folding:   width={} height={}", width, height);
    let mut part_2_board = dots.clone();
    for fold in &folds {
        part_2_board = fold_dots(&part_2_board, fold);

        if fold.dim == "y" {
            height = fold.idx;
        }
        if fold.dim == "x" {
            width = fold.idx;
        }
    }
    println!("After folding:    width={} height={}", width, height);

    let mut cart_dots: Vec<Vec<char>> = Vec::new();
    for _ in 0..height {
        cart_dots.push(vec![' '; width]);
    }
    for dot in part_2_board {
        cart_dots[dot.1][dot.0] = 'X';
    }

    println!("Final code:");
    for row in &cart_dots {
        let row_str: String = row.iter().copied().collect();
        println!("{}", &row_str);
    }

}

fn main() {
    day_13_origami();
}