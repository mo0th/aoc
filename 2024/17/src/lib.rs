use rayon::prelude::*;
use std::ops::BitXor;

pub fn get_input() -> String {
    String::from(include_str!("../input"))
}

pub fn get_sample_input() -> String {
    String::from(include_str!("../sample"))
}

fn combo_operand(operand: i64, registers: &Vec<i64>) -> i64 {
    match operand {
        0..=3 => operand,
        4..=6 => registers[(operand - 4) as usize],
        _ => unreachable!(),
    }
}

fn get_output(registers: &Vec<i64>, program: &Vec<i64>) -> Vec<i64> {
    let mut output = vec![];
    let mut registers = registers.clone();

    let mut i = 0;

    while i < program.len() {
        let opcode = program[i];
        i += 1;
        let operand = program[i];
        i += 1;

        match opcode {
            0 => {
                registers[0] = registers[0] >> (combo_operand(operand, &registers) as u32);
            }
            1 => {
                registers[1] = registers[1].bitxor(operand);
            }
            2 => {
                registers[1] = combo_operand(operand, &registers) % 8;
            }
            3 => {
                if registers[0] == 0 {
                    continue;
                }
                i = operand as usize;
            }
            4 => {
                registers[1] = registers[1].bitxor(registers[2]);
            }
            5 => {
                output.push(combo_operand(operand, &registers) % 8);
            }
            6 => {
                registers[1] = registers[0] >> (combo_operand(operand, &registers) as u32);
            }
            7 => {
                registers[2] = registers[0] >> (combo_operand(operand, &registers) as u32);
            }

            _ => unreachable!(),
        }
    }

    output
}

pub fn solve_a(input: String) -> String {
    let (registers, program) = input.split_once("\n\n").unwrap();
    let registers = registers
        .lines()
        .map(|line| line.split(" ").last().unwrap().parse::<i64>().unwrap())
        .collect::<Vec<_>>();

    let program = program
        .replace("Program: ", "")
        .split(",")
        .map(|n| n.parse::<i64>().unwrap())
        .collect::<Vec<_>>();

    let output = get_output(&registers, &program);

    output
        .iter()
        .map(|n| n.to_string())
        .collect::<Vec<_>>()
        .join(",")
}

pub fn solve_b(input: String) -> i64 {
    let (_, program) = input.split_once("\n\n").unwrap();

    let program = program
        .replace("Program: ", "")
        .split(",")
        .map(|n| n.parse::<i64>().unwrap())
        .collect::<Vec<_>>();

    println!("P\t\t=> {program:?}");
    // (1..1_000_000_000i64)
    //     .into_par_iter()
    //     // .par_bridge()
    //     .find_first(|i| {
    //         let output = get_output(&vec![*i, 0, 0], &program);
    //         println!("{i}");
    //         output == program
    //     })
    //     .unwrap_or(-1);

    for i in 1..=10 {
        let i = i + 10i64.pow(program.len() as u32) + 20;
        let output = get_output(&vec![i, 0, 0], &program);
        println!("{i}\t\t=> {output:?}");
    }

    return -1;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_eg_1() {
        assert_eq!(solve_a(get_sample_input()), "4,6,3,5,6,3,5,2,1,0");
    }

    #[test]
    fn a() {
        assert_eq!(solve_a(get_input()), "7,6,1,5,3,1,4,2,6");
    }

    #[test]
    #[ignore]
    fn b_eg_1() {
        assert_eq!(
            solve_b(
                "
Register A: 2024
Register B: 0
Register C: 0

Program: 0,3,5,4,3,0
"
                .trim()
                .to_string()
            ),
            117440
        );
    }

    #[test]
    #[ignore]
    fn b() {
        assert_eq!(solve_b(get_input()), 1);
    }

    //
}
