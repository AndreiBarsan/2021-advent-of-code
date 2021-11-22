// My first sort-of-real program in Rust, 2021-11-21.

use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::str::FromStr;

fn main() {
    let input_path = Path::new("../2020/001-input.txt");

    let mut nrs: [i32; 200] = [0; 200];
    let mut idx: usize = 0;

    if let Ok(lines) = read_lines(input_path) {
        for line in lines {
            if let Ok(nr) = line {
                // println!(" --> {}", nr);
                nrs[idx] = i32::from_str(&nr).unwrap();
                idx += 1;
            }
        }
    }
    // println!("{:?}", nrs);

    let N: usize = nrs.len();
    let target: i32 = 2020;

    'outer: for i in 0..N-1 {
        for j in i..N {
            if nrs[i] + nrs[j] == target {
                let result = nrs[i] * nrs[j];

                println!("\n{}\n", result);
                break 'outer;
            }
        }
    };


    // let mut file = match File::open(&input_path) {
    //     Err(why) => panic!("Could not open {}: {}", input_path.display(), why),
    //     Ok(file) => file,
    // };

    // let mut s = String::new();
    // match file.read_to_string(&mut s) {
    //     Err(why) => panic!("Could not read {}: {}", input_path.display(), why),
    //     Ok(_) => print!("Contents:\n{}", s),
    // };

}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}