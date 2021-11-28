use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::str::FromStr;
use std::collections::HashMap;

/// Represents a password rule.
/// The interpretation depends on the part of the problem being evaluated.
///   - In (1), lo and hi represent occurrence limits for ch.
///   - In (2), lo and hi represent the indices in the string we wish to check contain ch.
#[derive(Debug)]
struct Rule {
    lo: i32,
    hi: i32,
    ch: char,
}

fn letter_stats(input: &str) -> HashMap<char, i32> {
    let mut stats = HashMap::new();
    for ch in input.chars() {
        *stats.entry(ch).or_insert(0) += 1
    }
    stats
}

fn check_with_rule(password: &str, rule: &Rule) -> bool {
    let stats = letter_stats(password);
    let count: i32 = match stats.get(&rule.ch) {
        Some(count) => *count,
        None => 0
    };

    rule.lo <= count && count <= rule.hi
}

fn check_with_second_rule(password: &str, rule: &Rule) -> bool {
    if (password.len() as i32) < (rule.lo - 1) || (password.len() as i32) < (rule.hi - 1) {
        return false;
    }

    let lo_char: char = match password.chars().nth(rule.lo as usize) {
        Some(ch) => ch,
        None => panic!("")
    };
    let hi_char: char = match password.chars().nth(rule.hi as usize) {
        Some(ch) => ch,
        None => panic!("")
    };

    (lo_char == rule.ch) ^ (hi_char == rule.ch)
}


fn main() {
    let input_path = Path::new("../2020/002-input.txt");
    let mut counter_first: i32 = 0;
    let mut counter_second: i32 = 0;
    let mut total: i32 = 0;

    if let Ok(lines) = read_lines(input_path) {
        for line in lines {
            if let Ok(line_str) = line {
                let rule_and_password: Vec<&str>  = line_str.split(":").collect();
                let rule_str = rule_and_password[0];
                let rule = parse_rule(rule_str);
                let password = rule_and_password[1];

                let pass_first = check_with_rule(password, &rule);
                let pass_second = check_with_second_rule(password, &rule);
                if pass_first {
                    counter_first += 1;
                }
                if pass_second {
                    counter_second += 1;
                }

                total += 1;
            }
        }
    }

    println!("{} total lines", total);
    println!("{}", counter_first);
    println!("{}", counter_second);

    // let rule = parse_rule("12-16 s");
    // println!("{:?}", rule);
}

// TODO(andrei): What does the 'where' construct do? Why does the file reading
// function need generics here?
fn parse_rule(rule_str: &str) -> Rule {
    // TODO(andrei): Use pattern matching!
    let range_and_char: Vec<&str>  = rule_str.split(" ").collect();
    let range = range_and_char[0];
    let ch = match range_and_char[1].chars().nth(0) {
        Some(the_ch) => the_ch,
        None => panic!("Invalid format"),
    };
    let lo_hi_s: Vec<&str> = range.split("-").collect();
    let lo = i32::from_str(&lo_hi_s[0]).unwrap();
    let hi = i32::from_str(&lo_hi_s[1]).unwrap();

    Rule { lo: lo, hi: hi, ch: ch }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}