use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::str::FromStr;

const CMD_DOWN: &str = "down";
const CMD_UP: &str = "up";
const CMD_FORWARD: &str = "forward";


fn day_02_dive() {
    let input_path = Path::new("input/02.txt");

    let mut depth_a: i32 = 0;
    let mut depth_b: i32 = 0;
    let mut horizontal_a: i32 = 0;
    let mut horizontal_b: i32 = 0;
    let mut aim: i32 = 0;

    if let Ok(lines) = read_lines(input_path) {
        for line in lines {
            if let Ok(line_str) = line {
                let cmd_and_distance: Vec<&str> = line_str.split(" ").collect();
                let cmd = cmd_and_distance[0];
                let distance = i32::from_str(cmd_and_distance[1]).unwrap();

                if cmd == CMD_DOWN {
                    depth_a += distance;
                    aim += distance;
                }
                else if cmd == CMD_UP {
                    depth_a -= distance;
                    aim -= distance;
                }
                else if cmd == CMD_FORWARD {
                    horizontal_a += distance;
                    horizontal_b += distance;
                    depth_b += aim * distance;
                }
                else {
                    panic!("Invalid command {:?}.", cmd);
                }

            }
        }
    }

    println!("Part A: {} x {} = {}", depth_a, horizontal_a, depth_a * horizontal_a);
    println!("Part B: {} x {} = {}", depth_b, horizontal_b, depth_b * horizontal_b);
}

fn main() {
    day_02_dive();
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}