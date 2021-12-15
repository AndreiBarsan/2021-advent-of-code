use std::fs;
use std::collections::HashMap;
use std::collections::BTreeMap;
use std::str::FromStr;

// TODO(andrei): Remove BTreeMap as it's not helpful...

#[derive(Debug)]
struct InsertionRule {
    pattern: String,
    addition: String,
}


fn parse_rule(rule_spec: &String) -> InsertionRule {
    let parts: Vec<&str> = rule_spec.split(" -> ").collect();
    let lhs = parts[0];
    let rhs = parts[1];
    InsertionRule{ pattern: lhs.to_string(), addition: rhs.to_string() }
}


fn parse_insertion_rules(rule_specs: &[String]) -> Vec<InsertionRule> {
    rule_specs.iter().map(|spec| parse_rule(spec)).collect()
}


fn polymerize(polymer_template: &String, insertion_rules: &HashMap<String, char>) -> String {
    let mut new_chars: Vec<char> = Vec::new();
    if polymer_template.len() < 2 {
        panic!("Invalid polymer template, length must be at least 2. [polymer_template={:?}]", polymer_template);
    }

    for idx in 0..(polymer_template.len() - 1) {
        let identifier = &polymer_template[idx..idx+2];

        new_chars.push(identifier.chars().nth(0).unwrap());
        let maybe_sub = insertion_rules.get(identifier);
        // XXX(andrei): flat_map
        if let Some(sub) = maybe_sub {
            new_chars.push(*sub);
        }
    }
    // Add the last character, as there's no next pattern to trigger its insertion.
    new_chars.push(polymer_template.chars().nth(polymer_template.len() - 1).unwrap());

    // Collect the chars into a convenient String
    new_chars.iter().collect()
}


fn polymerize_fast(
    polymer_template: &BTreeMap<String, usize>,
    tokens: &Vec<String>,
    insertion_rules: &HashMap<String, char>
) -> (BTreeMap<String, usize>, Vec<String>)
{
    let mut new_polymer: BTreeMap<String, usize> = BTreeMap::new();
    let mut new_tokens = Vec::new();
    if polymer_template.len() < 1 {
        panic!("Invalid polymer template, length must be at least 1 pair. [polymer_template={:?}]", polymer_template);
    }

    // for (pair, initial_count) in polymer_template {
    for pair in tokens {
        let initial_count = polymer_template[pair];
        let maybe_sub = insertion_rules.get(pair);
        // XXX(andrei): flat_map
        if let Some(sub) = maybe_sub {
            let a = vec![pair.chars().nth(0).unwrap(), *sub];
            let b = vec![*sub, pair.chars().nth(1).unwrap()];

            let a_str: String = a.iter().collect();
            let b_str: String = b.iter().collect();
            // println!("{} -> {}, {} @ {}", pair, a_str, b_str, initial_count);
            *new_polymer.entry(a_str.to_string()).or_insert(0usize) += initial_count;
            *new_polymer.entry(b_str.to_string()).or_insert(0usize) += initial_count;

            if ! new_tokens.contains(&a_str.to_string()) {
                new_tokens.push(a_str.to_string());
            }
            if ! new_tokens.contains(&b_str.to_string()) {
                new_tokens.push(b_str.to_string());
            }
        }
        else {
            *new_polymer.entry(pair.to_string()).or_insert(0usize) += initial_count;
            new_tokens.push(pair.to_string());
        }
    }

    (new_polymer, new_tokens)
}

fn letter_stats(string: &String) -> HashMap<char, usize> {
    let mut stats = HashMap::new();

    for ch in string.chars() {
        *stats.entry(ch).or_insert(0usize) += 1;
    }

    stats
}

fn letter_stats_hist(data: &BTreeMap<String, usize>, tokens: &Vec<String>) -> HashMap<char, usize> {
    let mut stats = HashMap::new();

    // let data_vec: Vec<(&String, &usize)> = data.iter().collect();
    // println!("WTF: {:?}", data_vec[0]);

    for string in tokens {
        let count = data[string];
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
    // In this implementation I chose to have to add 'A' manually.
    let very_first_ch = tokens[0].chars().nth(0).unwrap();
    let first_count = data[&tokens[0]];
    *stats.entry(very_first_ch).or_insert(0usize) += 1;

    println!("Very first char: {}, offset  = {}", very_first_ch, first_count);

    stats
}


/// Returns the difference between the most frequent and the least frequent.
fn part_1_code(stats: &HashMap<char, usize>) -> usize {
    let mut stats_vec: Vec<&usize> = stats.iter().map(|(_, v)| v).collect();
    stats_vec.sort();

    let mut stats_vec_w_key: Vec<(&char, &usize)> = stats.iter().collect();
    stats_vec_w_key.sort_by(|a, b| a.1.partial_cmp(b.1).unwrap());
    println!("Stats: {:?}", stats_vec_w_key);

    /*
    With adjustment:

    O: 198828019091
    H: 206420145579
    ...
    P: 6287180922765
    F: 12464638059775

    Without adjustment:
    'K', 193200271244, becomes the smallest!
    */

    stats_vec[stats_vec.len() - 1] - stats_vec[0]
}

fn day_14_polymerization() {
    let input_fname = "input/14.txt";
    // let input_fname = "input/14-demo.txt";
    let data: Vec<String> = fs::read_to_string(input_fname).expect("Unable to read file.")
        .split("\n").map(|x| x.to_string()).collect();

    let base = &data[0];
    let insertion_rule_specs = &data[2..];

    let insertion_rules = parse_insertion_rules(insertion_rule_specs);
    let insertion_rules_map: HashMap<String, char> = insertion_rules.iter()
        .map(|r| (r.pattern.to_string(), r.addition.chars().nth(0).unwrap())).collect();

    let mut poly = base.to_string();
    for _ in 0..10 {
        poly = polymerize(&poly, &insertion_rules_map);
    }

    let poly_stats = letter_stats(&poly);
    println!("Naive mode stats: {:?}", poly_stats);
    let part_1_result = part_1_code(&poly_stats);
    println!("Part 1 solution: {}", part_1_result);

    println!("Part 2!");
    let mut poly_hist: BTreeMap<String, usize> = BTreeMap::new();
    let mut tokens: Vec<String> = Vec::new();
    for idx in 0..(base.len() - 1) {
        let identifier = &base[idx..idx+2];
        *poly_hist.entry(identifier.to_string()).or_insert(0usize) += 1;
        tokens.push(identifier.to_string());
    }
    for step_idx in 0..40 {
        // println!("Initial: {:?}", poly_hist);
        // WTF??
        // XXX
        let aux = polymerize_fast(&poly_hist, &tokens, &insertion_rules_map);
        poly_hist = aux.0;
        tokens = aux.1;
        let count: usize = 1 + poly_hist.iter().map(|(k, v)| v).sum::<usize>();
        // println!("Total count: {}", count);
    }

    // 12271437788531 is not good - too high
    // 12271437788531 after converting to use tokens but not fixing the final counting
    // 12265810040684 after tweaking the counting - smaller, but wrong - too high
    let poly_hist_stats = letter_stats_hist(&poly_hist, &tokens);
    println!("{:?}", tokens);
    println!("{:?}", poly_hist);
    println!("{:?}", poly_hist_stats);
    let part_2_result = part_1_code(&poly_hist_stats);
    println!("Part 2 solution: {}", part_2_result);
}


fn main() {
    day_14_polymerization();
}