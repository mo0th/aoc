use itertools::{repeat_n, Itertools};

pub fn get_input() -> String {
    String::from(include_str!("../input"))
}

pub fn get_sample_input() -> String {
    String::from(include_str!("../sample"))
}

fn parse_i64(input: &str) -> i64 {
    input.parse().unwrap()
}

type Equation = (i64, Vec<i64>);

#[derive(Debug, PartialEq, Eq)]
enum Op {
    Add,
    Multiply,
    Concat,
}

impl Op {
    fn apply(&self, a: i64, b: i64) -> Option<i64> {
        match self {
            Op::Add => a.checked_add(b),
            Op::Multiply => a.checked_mul(b),
            Op::Concat => format!("{a}{b}").parse().ok(),
        }
    }
}

fn to_ops_a(n: u32) -> Vec<Op> {
    return format!("{:0>16b}", n)
        .chars()
        .map(|c| match c {
            '0' => Op::Add,
            '1' => Op::Multiply,
            _ => panic!("invalid char '{}'", c),
        })
        .rev()
        .collect();
}

fn is_equation_ok_a(equation: &Equation) -> bool {
    let (target, values) = equation;
    let values = values.clone();

    let start = values[0];

    let max_ops_n = 2_u32.pow(values.len() as u32 - 1);
    for ops in 0..max_ops_n {
        let ops = to_ops_a(ops);

        let mut curr = start;

        let mut broke = false;

        for (i, next_val) in values.iter().skip(1).enumerate() {
            let op = &ops[i];

            curr = if let Some(updated) = op.apply(curr, *next_val) {
                if updated > *target {
                    broke = true;
                    break;
                }

                updated
            } else {
                broke = true;
                break;
            }
        }

        if target == &curr && !broke {
            return true;
        }
    }

    return false;
}

pub fn solve_a(input: String) -> i64 {
    let equations = input
        .lines()
        .map(|line| {
            let (target, values) = line.split_once(": ").unwrap();

            return (
                parse_i64(target),
                values.split(" ").map(parse_i64).collect::<Vec<_>>(),
            );
        })
        .collect::<Vec<_>>();

    let valid_eqs: i64 = equations
        .iter()
        .filter_map(|eq| {
            if is_equation_ok_a(eq) {
                Some(eq.0)
            } else {
                None
            }
        })
        .sum();

    valid_eqs
}

fn is_equation_ok_b(equation: &Equation) -> bool {
    let (target, values) = equation;
    let values = values.clone();

    let start = values[0];

    let ops_options = vec![Op::Add, Op::Multiply, Op::Concat];

    // dbg!(max_ops_n);
    for ops in repeat_n(ops_options.iter(), values.len() - 1).multi_cartesian_product() {
        let mut curr = start;

        let mut broke = false;

        for (i, next_val) in values.iter().skip(1).enumerate() {
            let op = &ops[i];

            curr = if let Some(updated) = op.apply(curr, *next_val) {
                if updated > *target {
                    broke = true;
                    break;
                }

                updated
            } else {
                broke = true;
                break;
            }
        }

        if target == &curr && !broke {
            return true;
        }
    }

    return false;
}

pub fn solve_b(input: String) -> i64 {
    let equations = input
        .lines()
        .map(|line| {
            let (target, values) = line.split_once(": ").unwrap();

            return (
                parse_i64(target),
                values.split(" ").map(parse_i64).collect::<Vec<_>>(),
            );
        })
        .collect::<Vec<_>>();

    let valid_eqs: i64 = equations
        .iter()
        .filter_map(|eq| {
            if is_equation_ok_b(eq) {
                Some(eq.0)
            } else {
                None
            }
        })
        .sum();

    valid_eqs
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_eg_1() {
        assert_eq!(solve_a(get_sample_input()), 3749);
    }

    #[test]
    fn a() {
        assert_eq!(solve_a(get_input()), 267566105056);
    }

    #[test]
    fn b_eg_1() {
        assert_eq!(solve_b(get_sample_input()), 11387);
    }

    #[test]
    fn b() {
        assert_eq!(solve_b(get_input()), 116094961956019);
    }

    //
}
