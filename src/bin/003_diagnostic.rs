use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use structopt::StructOpt;


fn filter_by_bit(codes: &Vec<String>, bit_idx: usize, most_common: bool) -> Vec<String> {
    let n_codes = codes.len() as u32;
    if n_codes <= 1 {
        // We make a copy of the original in this case
        return codes.to_vec();
    }

    let n_ones: usize = codes.into_iter().filter(|code| code.chars().nth(bit_idx) == Some('1')).count();

    let mut target: char = '0';
    if most_common {
        if n_ones >= (n_codes as f32 / 2.0).ceil() as usize {
            target = '1';
        }
    }
    else
    {
        if n_ones < (n_codes as f32 / 2.0).ceil() as usize {
            target = '1';
        }
    }
    // println!("{}, {}/{} ones => target = {}", bit_idx, n_ones, n_codes, target);

    codes.to_vec().into_iter().filter(|code| code.chars().nth(bit_idx) == Some(target)).collect()
}

#[derive(StructOpt)]
#[structopt(name = "AoC '21 Day 3: Diagnostic")]
struct Cli {
    #[structopt(long, parse(from_os_str))]
    input_fpath: std::path::PathBuf,
}


fn day_03_diagnostic(args: &Cli) {
    let mut bits: [i32; 128] = [0; 128];
    let mut n_lines: usize = 0;
    let mut n_digits: usize = 0;

    let mut codes = Vec::new();
    if let Ok(lines) = read_lines(&args.input_fpath) {
        for line in lines {
            if let Ok(line_str) = line {
                n_digits = line_str.len();

                for (idx, ch) in line_str.chars().enumerate() {
                    if ch == '1' {
                        bits[idx] += 1;
                    }
                }

                n_lines += 1;
                codes.push(line_str);
            }
        }
    }

    let mut bits_gamma: [i32; 128] = [0; 128];
    let mut bits_epsilon: [i32; 128] = [0; 128];

    // "Dispatch" the bits into the right numbers to solve Part 1
    for idx in 0..n_digits {
        if bits[idx] > (n_lines as i32 / 2i32) {
            bits_gamma[idx] = 1;
        }
        else {
            bits_epsilon[idx] = 1;
        }
    }

    let gamma = bin_to_dec((&bits_gamma[..n_digits]).to_vec());
    let epsilon = bin_to_dec((&bits_epsilon[..n_digits]).to_vec());

    println!("{}", gamma * epsilon);

    // Part 2
    let mut codes_oxygen = codes.to_vec();
    let mut codes_co2 = codes.to_vec();
    for bit_idx in 0..n_digits {
        println!("{}, {}", codes_oxygen.len(), codes_co2.len());
        codes_oxygen = filter_by_bit(&codes_oxygen, bit_idx, true);
        codes_co2 = filter_by_bit(&codes_co2, bit_idx, false);
    }
    println!("{}, {}", codes_oxygen.len(), codes_co2.len());
    println!("{:?}, {:?}", codes_oxygen[0], codes_co2[0]);

    let oxygen_val = bin_to_dec(codes_oxygen[0].chars().into_iter().map(|x| (x as i32) - ('0' as i32)).collect());
    let co2_val = bin_to_dec(codes_co2[0].chars().into_iter().map(|x| (x as i32) - ('0' as i32)).collect());
    println!("{}", oxygen_val);
    println!("{}", co2_val);
    println!("{}", oxygen_val * co2_val);
}

fn bin_to_dec(stuff: Vec<i32>) -> i32
{
    let mut idx: i32 = (stuff.len() - 1) as i32;
    let mut acc: i32 = 0;
    let mut exp: i32 = 1;

    loop {
        if stuff[idx as usize] == 1 {
            acc += exp;
        }
        exp = exp * 2;
        idx -= 1;
        if idx < 0 {
            break
        }
    }

    acc
}


fn main() {
    let args = Cli::from_args();
    day_03_diagnostic(&args);
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}