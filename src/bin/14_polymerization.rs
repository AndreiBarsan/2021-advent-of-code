use std::collections::HashMap;
/// 2021 AoC Day 14
///
/// Polymer "evolution" similar to the lantern fish population, except a little trickier with the counting.
use std::fs;

fn parse_rule(rule_spec: &String) -> (String, char) {
    let parts: Vec<&str> = rule_spec.split(" -> ").collect();
    let lhs = parts[0];
    let rhs = parts[1];
    (lhs.to_string(), rhs.chars().nth(0).unwrap())
}

fn parse_insertion_rules(rule_specs: &[String]) -> HashMap<String, char> {
    rule_specs.iter().map(|spec| parse_rule(spec)).collect()
}

fn apply_rule(identifier: &str, insertion_rules: &HashMap<String, char>) -> Vec<char> {
    let first_char = identifier.chars().nth(0).unwrap();
    if let Some(substitution) = insertion_rules.get(identifier) {
        vec![first_char, *substitution]
    } else {
        vec![first_char]
    }
}

fn polymerize(polymer_template: &String, insertion_rules: &HashMap<String, char>) -> String {
    if polymer_template.len() < 2 {
        panic!(
            "Invalid polymer template, length must be at least 2. [polymer_template={:?}]",
            polymer_template
        );
    }

    let mut new_chars: Vec<char> = (0..(polymer_template.len() - 1))
        .flat_map(|idx| apply_rule(&polymer_template[idx..idx + 2], insertion_rules))
        .collect();

    // Add the last character, as there's no next pattern to trigger its insertion
    new_chars.push(
        polymer_template
            .chars()
            .nth(polymer_template.len() - 1)
            .unwrap(),
    );

    // Collect the chars into a convenient String
    new_chars.iter().collect()
}

/// Like 'polymerize', only operating on histogram-based representations of polymers, for vastly improved efficiency.
fn polymerize_fast(
    polymer_template: &HashMap<String, usize>,
    insertion_rules: &HashMap<String, char>,
) -> HashMap<String, usize> {
    let mut new_polymer: HashMap<String, usize> = HashMap::new();
    if polymer_template.len() < 1 {
        panic!(
            "Invalid polymer template, length must be at least 1 pair. [polymer_template={:?}]",
            polymer_template
        );
    }

    for (pair, initial_count) in polymer_template {
        let maybe_sub = insertion_rules.get(pair);

        if let Some(sub) = maybe_sub {
            let a = vec![pair.chars().nth(0).unwrap(), *sub];
            let b = vec![*sub, pair.chars().nth(1).unwrap()];

            let a_str: String = a.iter().collect();
            let b_str: String = b.iter().collect();

            *new_polymer.entry(a_str.to_string()).or_insert(0usize) += initial_count;
            *new_polymer.entry(b_str.to_string()).or_insert(0usize) += initial_count;
        } else {
            *new_polymer.entry(pair.to_string()).or_insert(0usize) += initial_count;
        }
    }

    new_polymer
}

fn letter_stats(string: &String) -> HashMap<char, usize> {
    let mut stats = HashMap::new();

    for ch in string.chars() {
        *stats.entry(ch).or_insert(0usize) += 1;
    }

    stats
}

/// Computes the number of times each letter appears in the polymer represented by 'polymer'.
///
/// 'original', the initial t = 0 polymer, is needed to compute the count for the first letter correctly.
fn letter_stats_hist(original: &String, polymer: &HashMap<String, usize>) -> HashMap<char, usize> {
    let mut stats = HashMap::new();

    for (string, count) in polymer {
        let ch = string.chars().nth(1).unwrap();
        *stats.entry(ch).or_insert(0usize) += count;
    }

    // for (string, count) in data_vec {
    //     let ch = string.chars().nth(1).unwrap();
    //     *stats.entry(ch).or_insert(0usize) += count;
    // }

    // We represent something like AXBCYD as
    // AX
    //  XB
    //   BC
    //    CY
    //     YD
    //
    // Which means that we should make sure we only count each letter in a pair once, since almost each letter appears
    // in two pairs. The only exception are the first and last letters. Depending on how we count the other letters, we
    // need to manually add either 'A' or 'D's count.
    //
    // In this implementation I chose to have to add 'A' manually. (Note that this means ONE 'A', not adding ALL the
    // occurrences of 'AX' to 'A's statistics.)
    let very_first_ch = original.chars().nth(0).unwrap();
    // let first_count = data[&tokens[0]];
    *stats.entry(very_first_ch).or_insert(0usize) += 1;

    stats
}

/// Returns the difference between the most frequent and the least frequent.
fn part_1_code(stats: &HashMap<char, usize>) -> usize {
    let mut stats_vec: Vec<&usize> = stats.iter().map(|(_, v)| v).collect();
    stats_vec.sort();
    stats_vec[stats_vec.len() - 1] - stats_vec[0]
}

fn day_14_polymerization() {
    let n_steps_part_1: usize = 10;
    let n_steps_part_2: usize = 40;
    let input_fname = "input/14.txt";
    // let input_fname = "input/14-demo.txt";

    // Input data processing
    let data: Vec<String> = fs::read_to_string(input_fname)
        .expect("Unable to read file.")
        .split("\n")
        .map(|x| x.to_string())
        .collect();
    let base_polymer = data[0].to_string();
    let insertion_rule_specs = &data[2..];
    let insertion_rules_map = parse_insertion_rules(insertion_rule_specs);

    // Part 1 solution using naive strings
    let mut poly = base_polymer.clone();
    for _ in 0..n_steps_part_1 {
        poly = polymerize(&poly, &insertion_rules_map);
    }

    let poly_stats = letter_stats(&poly);
    // println!("Naive mode stats: {:?}", poly_stats);
    let part_1_result = part_1_code(&poly_stats);
    println!("Part 1 solution: {}", part_1_result);

    // Part 1 solution using the same idea as in the lantern fish case - represent the population (in this case, polymer
    // components) as a histogram.
    println!("Part 2!");
    let mut poly_hist: HashMap<String, usize> = HashMap::new();
    let mut tokens: Vec<String> = Vec::new();
    for idx in 0..(base_polymer.len() - 1) {
        let identifier = &base_polymer[idx..idx + 2];
        *poly_hist.entry(identifier.to_string()).or_insert(0usize) += 1;
        tokens.push(identifier.to_string());
    }
    for _ in 0..n_steps_part_2 {
        poly_hist = polymerize_fast(&poly_hist, &insertion_rules_map);
    }

    let poly_hist_stats = letter_stats_hist(&poly, &poly_hist);
    // println!("{:?}", tokens);
    // println!("{:?}", poly_hist);
    // println!("{:?}", poly_hist_stats);
    let part_2_result = part_1_code(&poly_hist_stats);
    println!("Part 2 solution: {}", part_2_result);
}

fn main() {
    day_14_polymerization();
}
