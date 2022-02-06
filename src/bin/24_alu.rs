/// 2021 AoC Day 24: Arithmetic Logic Unit
///
/// Find the largest and smallest 14-digit values which make a custom assembly program output zero.
///
/// The trick was to not naively iterate over all possible inputs to find the min/max accepted ones, as this would have
/// taken multiple days, even with a very fast Rust implementation.
///
/// Instead, I observed that the given ALU program actually decomposes into 14 similar chunks, one for each input digit,
/// and each chunk actually takes just two inputs - the digit and the current value of the ALU's 'z' register. This
/// allowed a much faster solution by only running a chunk of the program on the cross-product of possible 'z' and digit
/// inputs. Since the range of 'z' values which can be output by a chunk is (sort of) bounded, this made the search much
/// more efficient, though still slow - the program takes ~10 minutes to find a solution. Perhaps we can figure out an
/// even better way of filtering a chunk's valid 'z' inputs.
///
/// Looking at hints from other people on Reddit, it seems it may be beneficial to start from the end. The 14-chunk
/// observation is definitely 100% a correct insight. I also wonder whether we can chunk the program differently, so as
/// to exploit the modulo operations better. There's probably also a higher-level meaning to the 'z' register.


use std::fs;
use lazy_static::lazy_static;
use std::collections::HashMap;
use std::collections::HashSet;
use std::str::FromStr;

#[derive(Debug, Eq, PartialEq)]
enum Expr {
    LITERAL(i64),
    REG(char)
}

impl Expr {
    fn from(input: &str) -> Expr {
        match i64::from_str(input) {
            Ok(val) => Expr::LITERAL(val),
            Err(e) => Expr::REG(input.chars().nth(0).unwrap())
        }
    }
}

#[derive(Debug, Eq, PartialEq)]
enum ALUInstruction {
    INP(Expr),
    ADD(Expr, Expr),
    MUL(Expr, Expr),
    DIV(Expr, Expr),
    MOD(Expr, Expr),
    EQL(Expr, Expr)
}

fn parse_alu_instruction(raw: &str) -> ALUInstruction {
    let parts: Vec<&str> = raw.split(" ").collect();
    match parts[0] {
        "inp" => ALUInstruction::INP(Expr::from(parts[1])),
        "add" => ALUInstruction::ADD(Expr::from(parts[1]), Expr::from(parts[2])),
        "mul" => ALUInstruction::MUL(Expr::from(parts[1]), Expr::from(parts[2])),
        "div" => ALUInstruction::DIV(Expr::from(parts[1]), Expr::from(parts[2])),
        "mod" => ALUInstruction::MOD(Expr::from(parts[1]), Expr::from(parts[2])),
        "eql" => ALUInstruction::EQL(Expr::from(parts[1]), Expr::from(parts[2])),
        _ => panic!("Invalid instruction name: {}", parts[0])
    }
}

fn execute(instructions: &Vec<ALUInstruction>, input: &Vec<i64>, initial_z: i64, start_instr: usize, end_instr: usize) -> i64 {
    // TODO(andrei): If needed, refactor this into a "context" and pass it around to separate functions
    // each implementing an operation.
    let mut reg: HashMap<char, i64> = HashMap::new();
    reg.insert('w', 0i64);
    reg.insert('x', 0i64);
    reg.insert('y', 0i64);
    reg.insert('z', initial_z);

    let mut in_ptr = 0usize;

    for (instruction_idx, inst) in instructions.iter().enumerate() {
        if instruction_idx < start_instr {
            continue;
        }
        if end_instr > 0 && instruction_idx >= end_instr {
            break;
        }
        match inst {
            ALUInstruction::INP(Expr::REG(reg_name)) => {
                *reg.entry(*reg_name).or_insert(0i64) = input[in_ptr];
                in_ptr += 1;
                // if in_ptr == 2 {
                //     println!("---------------------");

                // }
                // println!("z = {:?}", reg[&'z']);
                // println!("{:?}", reg);
                // }
            },
            ALUInstruction::ADD(Expr::REG(target_reg), Expr::REG(source_reg)) => {
                *reg.entry(*target_reg).or_insert(0i64) += reg[source_reg];
            }
            ALUInstruction::ADD(Expr::REG(target_reg), Expr::LITERAL(lit)) => {
                *reg.entry(*target_reg).or_insert(0i64) += lit;
            }
            ALUInstruction::MUL(Expr::REG(target_reg), Expr::REG(source_reg)) => {
                // println!("{:?}", reg);
                let initial = reg[target_reg];
                *reg.entry(*target_reg).or_insert(0i64) *= reg[source_reg];
                // println!("Mul: {:?} (= {:?}) * {:?} (= {:?}) ==> {:?}",
                // target_reg, initial, source_reg, reg[source_reg], reg[target_reg]);
            }
            ALUInstruction::MUL(Expr::REG(target_reg), Expr::LITERAL(lit)) => {
                *reg.entry(*target_reg).or_insert(0i64) *= lit;
            }
            ALUInstruction::DIV(Expr::REG(target_reg), Expr::REG(source_reg)) => {
                *reg.entry(*target_reg).or_insert(0i64) /= reg[source_reg];
            }
            ALUInstruction::DIV(Expr::REG(target_reg), Expr::LITERAL(lit)) => {
                *reg.entry(*target_reg).or_insert(0i64) /= lit;
            }
            ALUInstruction::MOD(Expr::REG(target_reg), Expr::REG(source_reg)) => {
                *reg.entry(*target_reg).or_insert(0i64) %= reg[source_reg];
            }
            ALUInstruction::MOD(Expr::REG(target_reg), Expr::LITERAL(lit)) => {
                *reg.entry(*target_reg).or_insert(0i64) %= lit;
            }
            ALUInstruction::EQL(Expr::REG(target_reg), Expr::REG(source_reg)) => {
                let result: bool = reg[target_reg] == reg[source_reg];
                *reg.entry(*target_reg).or_insert(0i64) = i64::from(result);
            }
            ALUInstruction::EQL(Expr::REG(target_reg), Expr::LITERAL(lit)) => {
                let result: bool = reg[target_reg] == *lit;
                *reg.entry(*target_reg).or_insert(0i64) = i64::from(result);
            }
            _ => panic!("Invalid instruction format: {:?}", inst)
        }

    }

    reg[&'z']
}

fn digit_vec(num: i64) -> Vec<i64> {
    let mut aux: Vec<i64> = Vec::new();
    let mut num_cur = num;
    while num_cur > 0 {
        aux.push(num_cur % 10);
        num_cur /= 10;
    }
    // aux.reverse();
    aux
}

/// Brain dump of me trying to understand the input program - not actually used at runtime.
// fn monad_rust(input: &Vec<i64>) {
//     let mut mem_idx = 0usize;
//     let mut w = 0i64;
//     let mut x = 0i64;
//     let mut y = 0i64;
//     let mut z = 0i64;

//     w = input[mem_idx];
//     mem_idx += 1;
//     // Guaranteed z == 0 for the first digit
//     // x = z % 26 + 14;
//     x = 1;

//     // Guaranteed x == 0 for the first digit, since (x := 14) != digit.
//     // Thus y is guaranteed to be 1.
//     y = 25 * x + 1;
//     z = z * y;  // == 0
//     // Guaranteed to be w + 12
//     y = (w + 12) * x;       // 12 + digit
//     z = z + y;              // 12 + first digit, always (13--21 inclusive)

//     // Second digit
//     w = input[mem_idx];
//     mem_idx += 1;
//     x = z + 10;         // 23--31 inclusive
//     x = 1;              // always this, since 23 > 9
//     y = (25 * x + 1); // == 26, always
//     z = z * y; // (13--21) * 26
//     y = (w + 9) * x;       // 9 + second digit, always (10--18 inclusive)
//     z = z + y;  // (12 + d1) * 26 + (d2 + 9)

//     // Third digit
//     // ...
//     w = input[mem_idx];
//     mem_idx += 1;
//     x = z % 26 + 13;
//     x = 1;      // Since z % 26 + 13 will always be > any digit
//     y = 25 * 1 + 1;
//     z = z * y;  // == z * 26
//     z += (w + 8) * 1;
//     // ((12 + d1) * 26 + (d2 + 9)) * 26 + (d3 + 8)

//     // Fourth digit (the first to have a negative coef)
//     w = input[mem_idx];
//     mem_idx += 1;

//     x = z % 26 - 8;     // Can be negative!! Need the above expression % 26 to be small enough.
//     z = z / 26;
//     if x == w {
//         x = 0;
//     }
//     else {
//         x = 1;
//     }
//     z = z * (25 * x + 1);
//     z = z + (w + 3) * x;

//     // Fifth digit
//     w = input[mem_idx];
//     mem_idx += 1;
//     // Sixth digit
//     w = input[mem_idx];
//     mem_idx += 1;
//     // Seventh digit
//     w = input[mem_idx];
//     mem_idx += 1;

//     // Eight digit
//     w = input[mem_idx];
//     mem_idx += 1;
//     x = z % 26 - 11;
//     z = z / 26;
// }


fn digit_block(digit: i64, z_in: i64, a: i64, b: i64, c: i64) -> i64 {

    let mut x = z_in % 26 + b;      // b is a possibly negative constant
    let mut z = z_in / a;       // a is either 1 or 26

    if x != digit {
        z = z * 26 + digit + c;
    }

    z
}


fn solve_version_b(commands: &Vec<ALUInstruction>) -> (Option<i64>, Option<i64>) {

    let mut target_zs: HashMap<i64, i64> = HashMap::new();
    target_zs.insert((0i64), (0i64));

    for digit_idx in (0..14usize).rev() {
        let mut new_target_zs: HashMap<i64, i64> = HashMap::new();
        println!("{}", digit_idx);
        for x in 1..10 {
            let mut digits = vec!(x);
            let instructions_per_digit = 18usize;
            let start_instruction = instructions_per_digit * digit_idx;
            let end_instruction = instructions_per_digit * (digit_idx + 1usize);

            for candidate_z in 0..100 {
                let z_val = execute(&commands, &digits, candidate_z, start_instruction, end_instruction);
                let rust_z_val = exec_spec(&digits, &vec![(26, -6, 5)], candidate_z);
                // println!("{}, {}", z_val, rust_z_val);
                assert_eq!(z_val, rust_z_val);

                if target_zs.contains_key(&z_val) {
                    println!("z_val = {} @ digit = {} is a good last-block input", candidate_z, x);
                    new_target_zs.insert(candidate_z, x);
                }
            }

            // for (initial_z, min_input) in &z_to_min_input {
        }
        println!("Target Zs contains: {}", new_target_zs.len());
        target_zs = new_target_zs;

        // if digit_idx == 12 {
        break;
        // }
    }


    (Some(-1i64), Some(-1i64))
}


/// The first attempt to solve the problem - using per-module caching.
///
/// Works, but very slowly - up to 15 min on a 2018 i9 (not parallelized).
fn solve_version_a(commands: &Vec<ALUInstruction>) -> (Option<i64>, Option<i64>) {
    // Maps each output 'z' value of a partial program to the maximum input number that produced it.
    let mut z_to_max_input: HashMap<i64, i64> = HashMap::new();
    let mut z_to_min_input: HashMap<i64, i64> = HashMap::new();
    z_to_max_input.insert(0, 0);
    z_to_min_input.insert(0, -1);

    for digit_idx in 0..14usize {
        println!("Processing digit {} / 14", digit_idx + 1);

        let start_a = 1i64;
        let end_a = 10i64;

        let mut new_z_to_max_input: HashMap<i64, i64> = HashMap::new();
        let mut new_z_to_min_input: HashMap<i64, i64> = HashMap::new();

        for x in start_a..end_a {
            let mut digits = digit_vec(x);
            assert_eq!(digits.len(), 1);
            let instructions_per_digit = 18usize;
            let start_instruction = instructions_per_digit * digit_idx;
            let end_instruction = instructions_per_digit * (digit_idx + 1usize);

            let mut z_idx = 0i64;

            for (initial_z, max_input) in &z_to_max_input {
                let z_val = execute(&commands, &digits, *initial_z, start_instruction,  end_instruction);
                // Heuristic to limit the search space when we know we can't possibly 'div' z enough to reach zero by
                // the end.
                if digit_idx > 8 && z_val > (26 * 26 * 26 * 26) {
                    continue;
                }

                // Since we visit x in ascending order, this will always have the biggest X for each Z.
                new_z_to_max_input.insert(z_val, max_input * 10 + x);

                z_idx += 1;
                if z_idx % 150000 == 0 {
                    let prog = (z_idx as f32) / (z_to_max_input.len() as f32);
                    // println!("{}", end - x);
                    println!("Progress: {:.2}%", prog * 100f32);
                }
            }
        }

        for x in (start_a..end_a).rev() {
            let mut z_idx = 0i64;
            let mut digits = vec![x];
            let instructions_per_digit = 18usize;
            let start_instruction = instructions_per_digit * digit_idx;
            let end_instruction = instructions_per_digit * (digit_idx + 1usize);
            for (initial_z, min_input) in &z_to_min_input {
                let z_val = execute(&commands, &digits, *initial_z, start_instruction,  end_instruction);
                // See previous loop for heuristic explanation.
                if digit_idx > 8 && z_val > (26 * 26 * 26 * 26) {
                    continue;
                }

                // Always keep the smallest value, which is done by the 'rev()' in how we visit the digits (I think).
                let aux = if min_input == &-1 {
                    0i64
                }
                else {
                    *min_input
                };
                new_z_to_min_input.insert(z_val, aux * 10 + x);

                z_idx += 1;
                if z_idx % 150000 == 0 {
                    let prog = (z_idx as f32) / (z_to_min_input.len() as f32);
                    // println!("{}", end - x);
                    println!("Progress: {:.2}%", prog * 100f32);
                }
            }
        }

        println!("z_val_max lookup has {} initial z's.", new_z_to_max_input.len());
        println!("z_val_min lookup has {} initial z's.", new_z_to_min_input.len());
        z_to_max_input = new_z_to_max_input;
        z_to_min_input = new_z_to_min_input;
    }

    // The 'cloned' simply dereferences the option, which is trivial since it's just an i64.
    let part_one_sol = z_to_max_input.get(&0).cloned();
    let part_two_sol = z_to_min_input.get(&0).cloned();

    (part_one_sol, part_two_sol)
}


fn exec_spec(digits: &Vec<i64>, spec: &Vec<(i64, i64, i64)>, start_z: i64) -> i64 {
    let mut z = start_z;
    for i in 0..spec.len() {
        let (a, b, c) = spec[i];
        z = digit_block(digits[i], z, a, b, c);
    }
    z
}


fn main() {
    let input_fname = "input/24.txt";
    let inputs = fs::read_to_string(input_fname).expect("Unable to read file.");

    let commands: Vec<ALUInstruction> = inputs.split("\n").map(parse_alu_instruction).collect();
    // for cmd in &commands {
    //     println!("{:?}", cmd);
    // }

    let program_spec: Vec<(i64, i64, i64)> = vec![
        (1, 14, 12),
        (1, 10, 9),
        (1, 13, 8),
        (26, -8, 3),
        (1, 11, 0),
        (1, 11, 11),
        (1, 14, 10),
        (26, -11, 13),
        (1, 14, 3),
        (26, -1, 10),
        (26, -8, 10),
        (26, -5, 14),
        (26, -16, 6),
        (26, -6, 5),
    ];


    let mut inp = digit_vec(18116121134117);
    inp.reverse();
    // println!(
    //     "{}",
    //     execute(&commands, &inp, 0i64, 0usize, commands.len() + 1usize)
    // );
    // let mut inp_b = digit_vec(18116121134111);
    // inp_b.reverse();
    // println!(
    //     "{}",
    //     execute(&commands, &inp_b, 0i64, 0usize, commands.len() + 1usize)
    // );
    let zz = exec_spec(&inp, &program_spec, 0i64);
    assert_eq!(zz, 0i64);
    println!("OK!");
    // return;

    // let (part_one_opt, part_two_opt) = solve_version_a(&commands);
    let (part_one_opt, part_two_opt) = solve_version_b(&commands);
    return;

    // Computed with the slow method, used to validate the faster re-implementations.
    let expected_part_one_solution: i64 = 39999698799429;
    let expected_part_two_solution: i64 = 18116121134117;
    println!("Part One:");
    if part_one_opt.is_some() {
        println!("Solution to part I found: {}", part_one_opt.unwrap());
    }
    else {
        println!("Could not find a solution!!!");
    }

    println!("\nPart Two:");
    match part_two_opt {
        Some(solution) => {
            println!("Solution to part II found: {}", part_two_opt.unwrap())
        }
        None =>  println!("Could not find a solution!!!")
    };
}