use std::{time::Instant, usize};

fn main() {
    let start = Instant::now();
    let input = include_str!("../input.txt");
    let sections: Vec<&str> = input.split("\n\n").collect();
    let mut registers: Vec<i64> = vec![];
    let mut instructions: Vec<&str> = vec![];

    for line in sections[0].lines() {
        let Some(tmp) = line.split_once(':') else {
            unreachable!()
        };
        let number = tmp.1.trim().parse::<i64>().unwrap();
        registers.push(number);
    }
    // should just be one line
    for line in sections[1].lines() {
        let Some(tmp) = line.split_once(':') else {
            unreachable!()
        };
        instructions = tmp.1.trim().split(",").collect();
    }

    let p1 = part_one(&registers, &instructions);
    println!("Part 1: {}", p1);
    // let r = vec![36371366, 0, 0];
    //    let p2 = part_one(&r, &instructions);
    //println!("Part 2: {}", p2);
    part_two(&instructions, instructions.join(","));
    println!("Time elapsed: {:.3?}", start.elapsed());
}

fn to_match(n: usize, expected: &String) -> String {
    let ret: String = expected
        .chars()
        .rev()
        .take(n)
        .collect::<Vec<_>>()
        .into_iter()
        .rev()
        .collect();
    return ret;
}

fn part_two(instructions: &Vec<&str>, expected: String) {
    // instructions from Input: 2,4,1,5,7,5,1,6,0,3,4,1,5,5,3,0
    // output len = 16
    // bst A - store A mod 4 in B
    // bxl 5- store B xor 5 in B
    // cdv 5 - store A / 2^B in C
    // bxl 6 - store B xor 6 in B
    // adv 3 - store A / 2^3 in A
    // bxc 1 - store B xor C in B
    // out 5 - output B mod 8
    // jnz 0 - pointer set to 0
    //
    // from back -> B = 8
    //              A = 0

    let mut match_n = 1;

    let mut m = to_match(match_n, &expected);

    let mut current = 0;

    loop {
        let r: Vec<i64> = vec![current, 0, 0];
        let ans = part_one(&r, &instructions);
        if ans == expected {
            println!("Part 2: {}", current);
            break;
        }
        // match one number at a time.
        // when match happens, multiply x 8 to retain the last number
        if ans == m {
            match_n += 2;
            m = to_match(match_n, &expected);
            current = current * 8;
            // get to lowest mult of 8
            current -= current % 8
        } else {
            current += 1;
        }
    }
}

fn part_one(registers: &Vec<i64>, ins: &Vec<&str>) -> String {
    let mut reg = registers.clone();
    let mut pointer = 0;
    let mut outputs: Vec<String> = vec![];
    loop {
        if pointer >= ins.len() {
            break;
        }
        let instr = ins[pointer];
        let combo = ins[pointer + 1];
        match &instr {
            &"0" => {
                let c = calc_combo(&reg, combo);
                // combo operand
                adv(&mut reg, &c);
                pointer += 2;
            }
            &"1" => {
                // literal operand
                bxl(&mut reg, combo);
                pointer += 2;
            }
            &"2" => {
                // combo operand
                let c = calc_combo(&reg, combo);
                bst(&mut reg, &c);
                pointer += 2;
            }
            &"3" => {
                // literal operand
                pointer = jnz(&mut reg, combo, &pointer);
            }
            &"4" => {
                bxc(&mut reg, combo);
                pointer += 2;
            }
            &"5" => {
                // combo operand
                let c = calc_combo(&reg, combo);
                outputs.extend(vec![out(&mut reg, &c)]);
                pointer += 2;
            }
            &"6" => {
                let c = calc_combo(&reg, combo);
                // combo operand
                bdv(&mut reg, &c);
                pointer += 2;
            }
            &"7" => {
                let c = calc_combo(&reg, combo);
                // combo operand
                cdv(&mut reg, &c);
                pointer += 2;
            }
            &"8" => {}
            _ => unreachable!(),
        }
    }
    //"1,5,0,3,7,3,0,3,1"
    return outputs.join(",");
}

fn calc_combo(reg: &Vec<i64>, combo: &str) -> i64 {
    match combo {
        "0" => 0,
        "1" => 1,
        "2" => 2,
        "3" => 3,
        "4" => reg[0],
        "5" => reg[1],
        "6" => reg[2],
        _ => 0,
    }
}

fn cdv(reg: &mut Vec<i64>, combo: &i64) {
    let numerator = reg[0];
    let base: i64 = 2;
    let denomenator = base.pow(*combo as u32);
    if denomenator == 0 {
        reg[2] = 1;
    } else {
        let res = numerator / denomenator;
        reg[2] = res;
    }
}

fn bdv(reg: &mut Vec<i64>, combo: &i64) {
    let numerator = reg[0];
    let base: i64 = 2;
    let denomenator = base.pow(*combo as u32);
    if denomenator == 0 {
        reg[1] = 1;
    } else {
        let res = numerator / denomenator;
        reg[1] = res;
    }
}

fn out(_reg: &mut Vec<i64>, combo: &i64) -> String {
    let out_val = combo % 8;
    let v = out_val.to_string();
    return v;
}

fn bxc(reg: &mut Vec<i64>, _combo: &str) {
    let b_rec = reg[1];
    let c_rec = reg[2];
    let xord = b_rec ^ c_rec;
    reg[1] = xord;
}

fn jnz(reg: &mut Vec<i64>, combo: &str, pointer: &usize) -> usize {
    let a_record = reg[0];
    let num_val = combo.parse::<usize>().unwrap();
    match a_record {
        0 => return *pointer + 2,
        _ => return num_val,
    }
}

fn bst(reg: &mut Vec<i64>, combo: &i64) {
    let m = combo % 8;
    reg[1] = m;
}

fn bxl(reg: &mut Vec<i64>, combo: &str) {
    let num_val = combo.parse::<i64>().unwrap();
    let b_val = reg[1];
    let xord = b_val ^ num_val;
    reg[1] = xord;
}

fn adv(reg: &mut Vec<i64>, combo: &i64) {
    let numerator = reg[0];
    let base: i64 = 2;
    // yolo
    let denomenator = base.pow(*combo as u32);
    if denomenator == 0 {
        reg[1] = 1;
    } else {
        let res = numerator / denomenator;
        reg[0] = res;
    }
}
