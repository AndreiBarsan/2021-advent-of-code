use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::str::FromStr;

use std::collections::HashMap;
use std::collections::HashSet;

const UNKNOWN: u32 = 1000;

/**
    Completely unambiguous:
        - 2 chars --> 1
        - 3 chars --> 7
        - 4 chars --> 4
        - 5 chars --> 2, 3, or 5
        - 6 chars --> 0, 9 or 6
        - 7 chars --> 8
        - 7 & 1 --> (7 \ 1 -> 'a' (the top bit))
        - 4 & 1 --> (b, d) or (d, b)
        - Look at the three 6-char ones -- the char from '1' missing in one of
        them will be 'c'. The other char from '1' will be 'f'. The 6-char digit
        you found here will be '6'.
        - The '6' will be a superset of exactly one 5-char digit -- '5'. 2 and 3
        both have stuff 6 doesn't. This extra character is 'e'.
        - We now know 'a', 'e', 'f', 'c'.
        - 0 \ 7 yields b, e, g. We know e and are left with b and g. We
        intersect this with (b, d) from above. The segment in common is 'b'. The
        other non-common segment from the first tuple is 'd'. We can also infer
        'g'.
        - We now know 'a', 'b', 'c', 'd', 'e', 'f', 'g'. Done.
*/

fn string_to_digit_easy(input: &str) -> u32 {
    match input.len() {
        2 => 1,
        7 => 8,
        3 => 7,
        4 => 4,
        _ => UNKNOWN,
    }
}

fn sorted_string(input: &str) -> String {
    let mut chrs: Vec<char> = input.chars().collect();
    chrs.sort();
    let result: String = chrs.into_iter().collect();
    result
}

fn decode(input: &str, mapping: &HashMap<char, char>) -> Vec<char> {
    let mut result: Vec<char> = input
        .chars()
        .map(|x| mapping.get(&x).unwrap())
        .copied()
        .collect();
    result.sort();
    result
}

fn readout(code: &Vec<char>) -> u32 {
    let segment_to_number: HashMap<String, u32> = HashMap::from([
        (String::from("abcefg"), 0u32),
        (String::from("cf"), 1u32),
        (String::from("acdeg"), 2u32),
        (String::from("acdfg"), 3u32),
        (String::from("bcdf"), 4u32),
        (String::from("abdfg"), 5u32),
        (String::from("abdefg"), 6u32),
        (String::from("acf"), 7u32),
        (String::from("abcdefg"), 8u32),
        (String::from("abcdfg"), 9u32),
    ]);

    let code_str: String = code.iter().copied().collect();
    *segment_to_number.get(&code_str).unwrap()
}

/// Decodes the LED matching using heuristics.
///
/// I wonder if we could formulate this task as a MIP and solve it with an off-the-shelf solver.
fn part_2_decoding(input: &Vec<&str>) -> HashMap<char, char> {
    let mut code_by_length_uniq: HashMap<usize, HashSet<String>> = HashMap::new();
    for code in input {
        let my_vec = code_by_length_uniq
            .entry(code.len())
            .or_insert(HashSet::new());
        // I *think* the 'to_string' here is meant to make a copy.
        (*my_vec).insert(sorted_string(code));
    }

    let mut code_by_length: HashMap<usize, Vec<String>> = HashMap::new();
    for (ll, codes) in code_by_length_uniq {
        // 'to_string' to create a copy
        let set_as_vec: Vec<String> = codes.iter().map(|x| x.to_string()).collect();
        code_by_length.insert(ll, set_as_vec);
    }

    let chars_in_one: HashSet<char> = code_by_length[&2usize][0].chars().collect();
    let chars_in_four: HashSet<char> = code_by_length[&4usize][0].chars().collect();
    let chars_in_seven: HashSet<char> = code_by_length[&3usize][0].chars().collect();

    let a_aux: Vec<char> = (&chars_in_seven - &chars_in_one).into_iter().collect();
    let a = a_aux[0];
    // println!("Code for 'a' is {}", a);

    // Ok, so no funny business in the data.
    //
    // if code_by_length.get(&6usize).unwrap().len() != 3usize {
    //     panic!("Oopsie!");
    // }

    let c_and_f: HashSet<char> = chars_in_seven.iter().copied().filter(|x| x != &a).collect();
    if c_and_f.len() != 2 {
        panic!("");
    }
    let mut b_and_d: Vec<char> = (&chars_in_four - &chars_in_one).into_iter().collect();
    if b_and_d.len() != 2 {
        panic!("");
    }

    // Look at the three 6-char ones -- the char from '1' missing in one of them will be 'c'. The other char from '1'
    // will be 'f'. The 6-char digit you found here will be '6'.
    let mut c: char = 'X';
    let mut chars_in_six: HashSet<char> = HashSet::new();
    for code in &code_by_length[&6usize] {
        let chars_in_current: HashSet<char> = code.chars().collect();
        let aux: Vec<char> = (&chars_in_one - &chars_in_current).into_iter().collect();

        if aux.len() == 1 {
            if c != 'X' {
                panic!("");
            }

            // We found the six.
            c = aux[0];
            chars_in_six = chars_in_current;
        }
    }
    if c == 'X' {
        panic!("Could not resolve which LED is 'c'!")
    }

    let only_f: Vec<char> = c_and_f.iter().copied().filter(|x| x != &c).collect();
    if only_f.len() != 1 {
        panic!(":(");
    }
    let f: char = only_f[0];
    // println!("Code for 'c' is {}, code for 'f' is {}", c, f);

    // The '6' will be a superset of exactly one 5-char digit -- '5'. 2 and 3
    // both have stuff 6 doesn't. This extra character is 'e'.
    let mut e = 'X';
    let mut chars_in_five: HashSet<char> = HashSet::new();
    for code in &code_by_length[&5usize] {
        let chars_in_current: HashSet<char> = code.chars().collect();
        let aux: Vec<char> = (&chars_in_current - &chars_in_six).into_iter().collect();
        if aux.len() == 0 {
            // We found the five.
            let aux_reversed: Vec<char> = (&chars_in_six - &chars_in_current).into_iter().collect();
            e = aux_reversed[0];
            chars_in_five = chars_in_current;
        }
    }
    if e == 'X' {
        panic!("Could not resolve which LED is 'e'!")
    }

    // 0 \ 7 yields b, e, g. We know e and are left with b and g. We intersect this with (b, d) from above. The segment
    // in common is 'b'. The other non-common segment from the first tuple is 'd'. We can also infer 'g'.
    //
    // Find 0 out of 0/6/9 by containing both e and c
    let only_zero: Vec<String> = code_by_length[&6usize]
        .iter()
        .map(|x| x.to_string())
        .filter(|code| code.contains(e) && code.contains(c))
        .collect();

    if only_zero.len() != 1 {
        panic!("Could not infer zero!");
    }
    let chars_in_zero: HashSet<char> = only_zero[0].chars().collect();
    let mut b_and_g: Vec<char> = (&chars_in_zero - &chars_in_seven)
        .into_iter()
        .filter(|x| x != &e)
        .collect();
    if b_and_g.len() != 2 {
        panic!("");
    }

    // println!("b_and_g: {:?}", b_and_g);
    // println!("b_and_d: {:?}", b_and_d);

    let mut b: char = 'X';
    let mut g: char = 'X';
    let mut d: char = 'X';
    // Manual checks are OK for this sort of intersection
    if b_and_g[0] == b_and_d[0] {
        b = b_and_g[0];
        g = b_and_g[1];
        d = b_and_d[1];
    } else if b_and_g[0] == b_and_d[1] {
        b = b_and_g[0];
        g = b_and_g[1];
        d = b_and_d[0];
    } else if b_and_g[1] == b_and_d[0] {
        b = b_and_g[1];
        g = b_and_g[0];
        d = b_and_d[1];
    } else if b_and_g[1] == b_and_d[1] {
        b = b_and_g[1];
        g = b_and_g[0];
        d = b_and_d[0];
    } else {
        panic!("b-and-g check failed");
    }

    HashMap::from([
        (a, 'a'),
        (b, 'b'),
        (c, 'c'),
        (d, 'd'),
        (e, 'e'),
        (f, 'f'),
        (g, 'g'),
    ])
}

fn day_08_seven_segment() {
    // let input_path = Path::new("input/08-demo.txt");
    let input_path = Path::new("input/08.txt");
    let mut part_1_total: u32 = 0;
    let mut part_2_total: u32 = 0;

    if let Ok(lines) = read_lines(input_path) {
        for line in lines {
            if let Ok(line_str) = line {
                let in_and_out: Vec<&str> = line_str.split(" | ").collect();
                // println!("{:?}", in_and_out[1]);

                let in_strings: Vec<&str> = in_and_out[0].split(" ").collect();
                let out_strings: Vec<&str> = in_and_out[1].split(" ").collect();

                let mut all_raw_codes = in_strings.to_vec();
                all_raw_codes.extend(out_strings.to_vec());

                let res: Vec<u32> = out_strings
                    .iter()
                    .copied()
                    .map(string_to_digit_easy)
                    .filter(|x| x != &UNKNOWN)
                    .collect();
                part_1_total += res.len() as u32;

                let char_mapping = part_2_decoding(&all_raw_codes);
                let out_decoded: Vec<Vec<char>> = out_strings
                    .to_vec()
                    .iter()
                    .map(|x| decode(&x, &char_mapping))
                    .collect();

                let out_numbers: Vec<u32> =
                    out_decoded.to_vec().iter().map(|x| readout(&x)).collect();

                // println!("{:?}", char_mapping);
                // println!("{:?}", out_decoded);
                // println!("{:?}", out_numbers);

                let output = out_numbers[0] * 1000
                    + out_numbers[1] * 100
                    + out_numbers[2] * 10
                    + out_numbers[3];
                // println!("{:?}", output);
                part_2_total += output;
            }
        }
    }

    println!("Part 1 result: {}", part_1_total);
    println!("Part 2 result: {}", part_2_total);
}

fn main() {
    day_08_seven_segment();
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
