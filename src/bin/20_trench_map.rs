/// 2021 AoC Day 20: Trench Map
///
/// For each position, count the pixels in the surrounding 3x3 region to get a binary number which you then use to do a
/// look-up and find the correct value for that pixel after the current iteration. Needs to support infiniely large
/// inputs and perform this update many times in order to produce the final result.
///
/// Hint used: If all zeroes cause a '#', then it's useful to look at what all ones goes into! Turns out, it has to be
/// '0', since otherwise the answer would be infinity.
use std::fs;

const DARK_PIXEL: char = '.';
const LIGHT_PIXEL: char = '#';
const N_LOOKUP_BITS: usize = 512;

struct World {
    finite_map: Vec<Vec<bool>>,
    // The color representing EVERYTHING beyond the known world.
    background: bool,
}

fn bool_to_pixel(val: &bool) -> char {
    if *val {
        LIGHT_PIXEL
    } else {
        DARK_PIXEL
    }
}

fn pixel_to_bool(pixel: char) -> bool {
    if pixel == LIGHT_PIXEL {
        true
    } else if pixel == DARK_PIXEL {
        false
    } else {
        panic!(
            "Invalid color character: {} (code = {})",
            pixel, pixel as u32
        );
    }
}

fn parse_input(raw_string: &String) -> (Vec<bool>, Vec<Vec<bool>>) {
    let parts: Vec<&str> = raw_string.split("\n\n").collect();
    let lookup_bits: Vec<bool> = parts[0].chars().map(pixel_to_bool).collect();

    if lookup_bits.len() != N_LOOKUP_BITS {
        panic!("Invalid number of lookup bits. Got: {}", lookup_bits.len());
    }

    let mut initial_image = Vec::new();
    for row in parts[1].split("\n") {
        let row_vec: Vec<bool> = row.chars().map(pixel_to_bool).collect();
        initial_image.push(row_vec);
    }

    (lookup_bits, initial_image)
}

fn bin_to_dec(bits: &Vec<bool>) -> i64 {
    let mut idx: i64 = (bits.len() - 1) as i64;
    let mut acc: i64 = 0;
    let mut exp: i64 = 1;

    loop {
        if bits[idx as usize] {
            acc += exp;
        }
        exp = exp * 2;
        idx -= 1;
        if idx < 0 {
            break;
        }
    }

    acc
}

fn conv2d_world(world: &World, lookup: &Vec<bool>, kernel_size: usize) -> World {
    if kernel_size % 2 == 0 {
        panic!("Kernel size must be odd");
    }

    let off: i32 = (kernel_size / 2) as i32;
    let total_padding = 1 * 2;
    let n_in_rows = world.finite_map.len();
    let n_in_cols = world.finite_map[0].len();

    // Prepare the output image
    let mut out_image = Vec::new();
    for _ in 0..(n_in_rows + total_padding) {
        out_image.push(vec![false; n_in_cols + total_padding]);
    }

    for out_row in 0..out_image.len() {
        for out_col in 0..out_image[0].len() {
            let mut bits: Vec<bool> = Vec::new();

            for row_off in -off..=off {
                for col_off in -off..=off {
                    let row_in = (out_row as i32) - 1i32 + row_off;
                    let col_in = (out_col as i32) - 1i32 + col_off;

                    let val = if row_in >= 0
                        && row_in < (n_in_rows as i32)
                        && col_in >= 0
                        && col_in < (n_in_cols as i32)
                    {
                        world.finite_map[row_in as usize][col_in as usize]
                    } else {
                        world.background
                    };

                    bits.push(val);
                }
            }

            let bit_value: i64 = bin_to_dec(&bits);
            let pixel = lookup[bit_value as usize];
            out_image[out_row][out_col] = pixel;
        }
    }

    let new_background: bool = if world.background {
        lookup[511]
    } else {
        lookup[0]
    };

    World {
        finite_map: out_image,
        background: new_background,
    }
}

/// The hacky v1 solution. Tries to avoid explicitly modeling the infinite background but doesn't produce the right
/// output for Part 2, probably due to some artifacts regarding the border.
fn conv2d_with_lookup(
    base_image: &Vec<Vec<bool>>,
    lookup: &Vec<bool>,
    kernel_size: usize,
) -> Vec<Vec<bool>> {
    if kernel_size % 2 == 0 {
        panic!("Kernel size must be odd");
    }

    let off: i32 = (kernel_size / 2) as i32;
    let total_padding = (kernel_size - 1) * 8;
    let n_in_rows = base_image.len();
    let n_in_cols = base_image[0].len();

    // Prepare the output image
    let mut out_image = Vec::new();
    for _ in 0..(n_in_rows + total_padding) {
        out_image.push(vec![false; n_in_cols + total_padding]);
    }

    for out_row in 0..out_image.len() {
        for out_col in 0..out_image[0].len() {
            // TODO(andrei): Clean this up!
            // if out_row < 2 || out_row > (out_image.len() - 2) {
            //     continue;
            // }
            // if out_col < 2 || out_col > (out_image[0].len() - 2) {
            //     continue;
            // }

            let mut bits: Vec<bool> = Vec::new();

            for row_off in -off..=off {
                for col_off in -off..=off {
                    // let row_in = (out_row as i32) - (kernel_size - 1) as i32 + row_off;
                    // let col_in = (out_col as i32) - (kernel_size - 1) as i32 + col_off;
                    let row_in = (out_row as i32) - ((kernel_size - 1) * 4) as i32 + row_off;
                    let col_in = (out_col as i32) - ((kernel_size - 1) * 4) as i32 + col_off;

                    let val = if row_in >= 0
                        && row_in < (n_in_rows as i32)
                        && col_in >= 0
                        && col_in < (n_in_cols as i32)
                    {
                        base_image[row_in as usize][col_in as usize]
                    } else {
                        false
                    };

                    bits.push(val);
                }
            }

            let bit_value: i64 = bin_to_dec(&bits);
            let pixel = lookup[bit_value as usize];
            out_image[out_row][out_col] = pixel;
        }
    }

    out_image
}

fn count_lights(data: &Vec<Vec<bool>>) -> usize {
    let mut n_lights: usize = 0;

    let border = 0;

    for row in &data[border..data.len() - border] {
        for el in &row[border..data.len() - border] {
            if *el {
                n_lights += 1;
            }
        }
    }

    n_lights
}

fn print_image(data: &Vec<Vec<bool>>) {
    for row in data {
        let row_str: String = row.iter().map(bool_to_pixel).collect();
        println!("{}", row_str);
    }
}

fn day_20_trench_map() {
    let input_fname = "input/20.txt";
    // let input_fname = "input/20-demo.txt";
    let raw_lines: String = fs::read_to_string(input_fname).expect("Unable to read file.");

    let (lookup_bits, initial_image) = parse_input(&raw_lines);
    let mut cur_world = World {
        finite_map: initial_image.clone(),
        background: false,
    };

    for iter_idx in 0..50 {
        if iter_idx % 5 == 0 {
            println!("Iter idx: {}", iter_idx + 1);
        }
        cur_world = conv2d_world(&cur_world, &lookup_bits, 3);

        if iter_idx == 1 {
            // 5392 is not correct... (too high)
            // 5223 is also too high
            // 4917 was right - the trick is to think about border condidtions
            let part_1_result = count_lights(&cur_world.finite_map);
            println!("Part 1 result: {}", part_1_result);
        }
    }

    // For part 2
    // 323361 is too high - I think I had a bug with my hardcoded border
    //  68689 is still too high. Hmm.
    //  18038 is still too high? Wtf.
    //  16389 is right - got it INSTANTLY after coding the problem in the non-hacky way! Woo!
    let part_2_result = count_lights(&cur_world.finite_map);
    println!("Part 2 result: {}", part_2_result);
}

fn main() {
    day_20_trench_map();
}
