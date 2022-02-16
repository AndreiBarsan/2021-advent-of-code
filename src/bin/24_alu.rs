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
use std::collections::HashMap;
use std::fs;
use std::str::FromStr;

#[derive(Debug, Eq, PartialEq)]
enum Expr {
    LITERAL(i64),
    REG(char),
}

impl Expr {
    fn from(input: &str) -> Expr {
        match i64::from_str(input) {
            Ok(val) => Expr::LITERAL(val),
            Err(e) => Expr::REG(input.chars().nth(0).unwrap()),
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
    EQL(Expr, Expr),
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
        _ => panic!("Invalid instruction name: {}", parts[0]),
    }
}

fn execute(
    instructions: &Vec<ALUInstruction>,
    input: &Vec<i64>,
    initial_z: i64,
    start_instr: usize,
    end_instr: usize,
) -> i64 {
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
            }
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
            _ => panic!("Invalid instruction format: {:?}", inst),
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

fn vec_to_num(digits: &Vec<i64>) -> i64 {
    let mut nr = 0;
    for d in digits {
        nr = nr * 10 + d;
    }
    nr
}

fn get_program_spec() -> Vec<(i64, i64, i64)> {
    // 1  = peek
    // 26 = pop
    vec![
        (1, 14, 12), // d00
        (1, 10, 9),
        (1, 13, 8),
        (26, -8, 3),
        (1, 11, 0),
        (1, 11, 11),
        (1, 14, 10),
        (26, -11, 13),
        (1, 14, 3),
        (26, -1, 10),
        (26, -8, 10), // d10
        (26, -5, 14), // d11
        (26, -16, 6), // d12
        (26, -6, 5),  // d13
    ]
}

fn digit_block(digit: i64, z_in: i64, a: i64, b: i64, c: i64) -> i64 {
    // if a == 1
    // PEEK into x
    // elif a == 26
    // POP into x
    // end
    // x += b
    let mut x = z_in % 26 + b; // b is a possibly negative constant
    let mut z = z_in / a; // a is either 1 or 26

    if x != digit {
        // PUSH (digit + c)
        z = z * 26 + digit + c;
    }
    z
}

/// This version relies on the hint that the instructions implement a simple stack.
///
/// The stack is represented as the digits of a base-26 number stored in 'z'. The insight is you want the stack to be
/// empty at the conclusion of the program, which means you need the digits to satisfy certain conditions so as to
/// prevent any PUSH operations onto the stack which can be presented.
///
/// 'z = z * 26 + foo' pushes 'foo' onto the stack.
/// 'x = z % 26; z /= 1' peeks at the top of the stack, putting the value into x.
/// 'x = z % 26; z /= 26' pops x off z.
///
/// I found this final hint in the AoC Day 24 reddit thread, but did not actually look at any solution code /
/// spreadsheet.
fn solve_version_c() -> (Option<i64>, Option<i64>) {
    // d00 == d13 - 6
    // d01 == d12 + 7
    // d02 == d03
    // d04 == d11 + 5
    // d05 == d10 - 3
    // d06 == d07 + 1
    // d08 == d09 - 2

    // This can be solved manually based on the invariants from above, which, in turn, are inferred from the assembly.
    let digits_max = vec![
        3, 9, 9, 9, 9, 6, 9, 8, 7, 9, 9, // d10
        4, // d11
        2, // d12
        9, // d13
    ];
    let digits_min = vec![
        1, 8, 1, 1, 6, 1, 2, 1, 1, 3, 4, // d10
        1, // d11
        1, // d12
        7, // d13
    ];

    (Some(vec_to_num(&digits_max)), Some(vec_to_num(&digits_min)))
}

/// Incomplete optimized modification of solution A which attempts to start from the end.
/// Not currently functional - I gave up since this did not seem promising enough in terms of speed, and instead focused
/// more on reverse-engineering the assembly for formulating version C.
fn solve_version_b(commands: &Vec<ALUInstruction>) -> (Option<i64>, Option<i64>) {
    let mut target_zs: HashMap<i64, i64> = HashMap::new();
    target_zs.insert((0i64), (0i64));

    let spec = get_program_spec();

    for digit_idx in (0..14usize).rev() {
        let mut new_target_zs: HashMap<i64, i64> = HashMap::new();
        let mul = 10i64.pow(13u32 - (digit_idx as u32));
        println!("Digit {}", digit_idx);
        for x in 1..10 {
            let mut digits = vec![x];
            let instructions_per_digit = 18usize;
            let start_instruction = instructions_per_digit * digit_idx;
            let end_instruction = instructions_per_digit * (digit_idx + 1usize);

            let candidates = if digit_idx == 0 {
                0..1
            } else {
                0..((target_zs.len() + 1) * 200)
            };

            for candidate_z in candidates {
                // let classic_z_val = execute(&commands, &digits, candidate_z, start_instruction, end_instruction);
                let z_val = exec_spec(&digits, &vec![spec[digit_idx]], candidate_z as i64);
                // println!("{}, {}", z_val, rust_z_val);
                // assert_eq!(z_val, rust_z_val);

                if target_zs.contains_key(&z_val) {
                    // println!("z_val = {} @ digit = {} is a good last-block input", candidate_z, x);
                    new_target_zs.insert(candidate_z as i64, x * mul + target_zs[&z_val]);
                }
            }
        }
        println!("Target Zs contains: {}", new_target_zs.len());
        target_zs = new_target_zs;

        if digit_idx == 1 {
            println!("{:?}", target_zs);
        }

        // if digit_idx == 12 {
        //     break;
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
                let z_val = execute(
                    &commands,
                    &digits,
                    *initial_z,
                    start_instruction,
                    end_instruction,
                );
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
                let z_val = execute(
                    &commands,
                    &digits,
                    *initial_z,
                    start_instruction,
                    end_instruction,
                );
                // See previous loop for heuristic explanation.
                if digit_idx > 8 && z_val > (26 * 26 * 26 * 26) {
                    continue;
                }

                // Always keep the smallest value, which is done by the 'rev()' in how we visit the digits (I think).
                let aux = if min_input == &-1 { 0i64 } else { *min_input };
                new_z_to_min_input.insert(z_val, aux * 10 + x);

                z_idx += 1;
                if z_idx % 150000 == 0 {
                    let prog = (z_idx as f32) / (z_to_min_input.len() as f32);
                    // println!("{}", end - x);
                    println!("Progress: {:.2}%", prog * 100f32);
                }
            }
        }

        println!(
            "z_val_max lookup has {} initial z's.",
            new_z_to_max_input.len()
        );
        println!(
            "z_val_min lookup has {} initial z's.",
            new_z_to_min_input.len()
        );
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
    let program_spec: Vec<(i64, i64, i64)> = get_program_spec();

    // let (part_one_opt, part_two_opt) = solve_version_a(&commands);
    // let (part_one_opt, part_two_opt) = solve_version_b(&commands);
    let (part_one_opt, part_two_opt) = solve_version_c();

    // Computed with the slow method, used to validate the faster re-implementations.
    let expected_part_one_solution: i64 = 39999698799429;
    let expected_part_two_solution: i64 = 18116121134117;
    println!("Part One:");
    if part_one_opt.is_some() {
        println!("Solution to part I found: {}", part_one_opt.unwrap());
    } else {
        println!("Could not find a solution!!!");
    }

    println!("\nPart Two:");
    match part_two_opt {
        Some(solution) => {
            println!("Solution to part II found: {}", part_two_opt.unwrap())
        }
        None => println!("Could not find a solution!!!"),
    };
}
