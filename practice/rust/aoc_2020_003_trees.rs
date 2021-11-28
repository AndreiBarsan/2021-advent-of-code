use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::str::FromStr;


fn day_03_trees() {
    let input_path = Path::new("../2020/003-input.txt");

    if let Ok(lines) = read_lines(input_path) {
        for line in lines {
            if let Ok(line_str) = line {

}


fn main() {
    day_03_trees()
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}