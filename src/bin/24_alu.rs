/// 2021 AoC Day 24: Arithmetic Logic Unit


use std::fs;
use lazy_static::lazy_static;
use std::collections::HashMap;
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

fn monad_rust(input: &Vec<i64>) {
    let mut mem_idx = 0usize;
    let mut w = 0i64;
    let mut x = 0i64;
    let mut y = 0i64;
    let mut z = 0i64;

    w = input[mem_idx];
    mem_idx += 1;
    // Guaranteed z == 0 for the first digit
    // x = z % 26 + 14;
    x = 1;

    // Guaranteed x == 0 for the first digit, since (x := 14) != digit.
    // Thus y is guaranteed to be 1.
    y = 25 * x + 1;
    z = z * y;  // == 0
    // Guaranteed to be w + 12
    y = (w + 12) * x;       // 12 + digit
    z = z + y;              // 12 + first digit, always (13--21 inclusive)

    // Second digit
    w = input[mem_idx];
    mem_idx += 1;
    x = z + 10;         // 23--31 inclusive
    x = 1;              // always this, since 23 > 9
    y = (25 * x + 1); // == 26, always
    z = z * y; // (13--21) * 26
    y = (w + 9) * x;       // 9 + second digit, always (10--18 inclusive)
    z = z + y;  // (12 + d1) * 26 + (d2 + 9)

    // Third digit
    // ...
    w = input[mem_idx];
    mem_idx += 1;
    x = z % 26 + 13;
    x = 1;      // Since z % 26 + 13 will always be > any digit
    y = 25 * 1 + 1;
    z = z * y;  // == z * 26
    z += (w + 8) * 1;
    // ((12 + d1) * 26 + (d2 + 9)) * 26 + (d3 + 8)

    // Fourth digit (the first to have a negative coef)
    w = input[mem_idx];
    mem_idx += 1;

    x = z % 26 - 8;     // Can be negative!! Need the above expression % 26 to be small enough.
    z = z / 26;
    if x == w {
        x = 0;
    }
    else {
        x = 1;
    }
    z = z * (25 * x + 1);
    z = z + (w + 3) * x;

    // Fifth digit
    w = input[mem_idx];
    mem_idx += 1;
    // Sixth digit
    w = input[mem_idx];
    mem_idx += 1;
    // Seventh digit
    w = input[mem_idx];
    mem_idx += 1;

    // Eight digit
    w = input[mem_idx];
    mem_idx += 1;
    x = z % 26 - 11;
    z = z / 26;

    // 13th digit

}


fn main() {
    let input_fname = "input/24.txt";
    let inputs = fs::read_to_string(input_fname).expect("Unable to read file.");

    let commands: Vec<ALUInstruction> = inputs.split("\n").map(parse_alu_instruction).collect();
    for cmd in &commands {
        println!("{:?}", cmd);
    }

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
            //
            // for (initial_z, max_input) in &z_to_max_input {
            //     let z_val = execute(&commands, &digits, *initial_z, start_instruction,  end_instruction);

            //     // Since we visit x in ascending order, this will always have the biggest X for each Z.
            //     new_z_to_max_input.insert(z_val, max_input * 10 + x);

            //     z_idx += 1;
            //     if z_idx % 500000 == 0 {
            //         let prog = (z_idx as f32) / (z_to_max_input.len() as f32);
            //         // println!("{}", end - x);
            //         println!("Progress: {:.2}%", prog * 100f32);
            //     }
            // }
        }

        for x in (start_a..end_a).rev() {
            let mut z_idx = 0i64;
            let mut digits = digit_vec(x);
            assert_eq!(digits.len(), 1);
            let instructions_per_digit = 18usize;
            let start_instruction = instructions_per_digit * digit_idx;
            let end_instruction = instructions_per_digit * (digit_idx + 1usize);
            for (initial_z, min_input) in &z_to_min_input {
                let z_val = execute(&commands, &digits, *initial_z, start_instruction,  end_instruction);

                // ...for the minimum, we just do the opposite. Always keep the first (smallest) option!
                if (! new_z_to_max_input.contains_key(&z_val)) || new_z_to_max_input[&z_val] == -1 {
                    let aux = if min_input == &-1 {
                        0i64
                    }
                    else {
                        *min_input
                    };
                    new_z_to_min_input.insert(z_val, aux * 10 + x);
                }

                z_idx += 1;
                if z_idx % 500000 == 0 {
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

    println!("Part One:");
    // Attempt 1: 39999698799429 - correct
    if z_to_max_input.contains_key(&0) {
        println!("Solution to part I found: {}", z_to_max_input[&0]);
    }
    else {
        println!("Could not find a solution!!!");
    }

    println!("\nPart Two:");
    if z_to_min_input.contains_key(&0) {
        println!("Solution to part I found: {}", z_to_min_input[&0]);
    }
    else {
        println!("Could not find a solution!!!");
    }

}