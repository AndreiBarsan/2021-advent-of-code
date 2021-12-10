use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::str::FromStr;
use regex::Regex;
use lazy_static::lazy_static;
use std::cmp::{max, min};


#[derive(Debug)]
struct Seafloor {
    width: usize,
    height: usize,
    // Not contiguous but good enough for this problem.
    data: Vec<Vec<i32>>,
}

#[derive(Debug)]
struct Point {
    x: u32,
    y: u32
}

#[derive(Debug)]
struct LineSegment {
    start: Point,
    end: Point
}

impl Seafloor {
    fn new(width: usize, height: usize) -> Self {
        let mut data = Vec::new();
        for _ in 0..height {
            let new_vec = vec![0; width];
            data.push(new_vec);
        }
        Seafloor {
            width: width,
            height: height,
            data: data
        }
    }

    fn register(&mut self, seg: &LineSegment) {
        let pts = seg.as_point_vec();
        for p in pts {
            self.data[p.y as usize][p.x as usize] += 1;
        }
    }

    fn count_gte(&self, min_val: i32) -> usize {
        let mut count = 0usize;

        for row in 0..self.height {
            for col in 0..self.width {
                if self.data[row][col] >= min_val {
                    count += 1;
                }
            }
        }

        count
    }
}

impl LineSegment {
    fn from_str(spec: String) -> Self {
        lazy_static! {
            static ref LINE_PARSE_RE: Regex = Regex::new(r"(\d+),(\d+)\s*->\s*(\d+),(\d+)").unwrap();
        }

        let caps = LINE_PARSE_RE.captures(&spec).unwrap();
        let s_x = u32::from_str(&caps[1]).unwrap();
        let s_y = u32::from_str(&caps[2]).unwrap();
        let e_x = u32::from_str(&caps[3]).unwrap();
        let e_y = u32::from_str(&caps[4]).unwrap();

        LineSegment {
            start: Point{x: s_x, y: s_y},
            end: Point{x: e_x, y: e_y},
        }
    }

    fn as_point_vec(&self) -> Vec<Point> {
        let mut ret = Vec::new();
        let min_x = min(self.start.x, self.end.x);
        let max_x = (max(self.start.x, self.end.x) + 1);
        let min_y = min(self.start.y, self.end.y);
        let max_y = (max(self.start.y, self.end.y) + 1);

        if self.start.x != self.end.x && self.start.y != self.end.y {
            // Diagonal lines are assumed to always be 45 deg (otherwise I guess we'd have to have a threshold
            // parameter and implement Bresenham).
            let mut step_x = 1i32;
            let mut step_y = 1i32;

            if self.end.x < self.start.x {
                step_x = -1i32;
            }
            if self.end.y < self.start.y {
                step_y = -1i32;
            }

            let len = max_x - min_x;
            for ii in 0..len {
                let new_x = ((self.start.x as i32) + ((ii as i32) * step_x)) as u32;
                let new_y = ((self.start.y as i32) + ((ii as i32) * step_y)) as u32;
                ret.push(Point {x: new_x, y: new_y});
            }

        }
        else if self.start.x != self.end.x {
            // horizontal segment
            for xx in min_x..max_x {
                ret.push(Point {x: xx, y: self.start.y});
            }
        }
        else if self.start.y != self.end.y {
            // vertical segment
            for yy in min_y..max_y {
                ret.push(Point {x: self.start.x, y: yy});
            }
        }
        ret
    }

}


fn day_05_hydro_vents() {
    let input_path = Path::new("input/05.txt");
    // let input_path = Path::new("input/05-demo.txt");
    let mut line_segments = Vec::new();

    if let Ok(lines) = read_lines(input_path) {
        for line in lines {
            if let Ok(line_str) = line {
                line_segments.push(LineSegment::from_str(line_str));
            }
        }
    }

    let mut max_x: u32 = 0;
    let mut max_y: u32 = 0;
    for seg in &line_segments {
        max_x = max(max_x, max(seg.start.x, seg.end.x));
        max_y = max(max_y, max(seg.start.y, seg.end.y));
    }
    // All coordinates are 1-based.
    max_x += 1;
    max_y += 1;

    let mut seafloor = Seafloor::new(max_y as usize, max_x as usize);

    // println!("{:?}", seafloor);
    for seg in &line_segments {
        seafloor.register(seg);
    }
    println!("{:?}", seafloor);
    let part_1_result = seafloor.count_gte(2);
    println!("Part 1: {}", part_1_result);
}

fn main() {
    day_05_hydro_vents();
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}