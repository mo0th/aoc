use rayon::prelude::*;
use std::collections::HashMap;
use tqdm::Iter;

pub fn get_input() -> String {
    String::from(include_str!("../input"))
}

pub fn get_sample_input() -> String {
    String::from(include_str!("../sample"))
}

#[derive(Debug, Clone)]
enum Action {
    Yell(i64),
    Add(String, String),
    Sub(String, String),
    Mul(String, String),
    Div(String, String),
    Eq(String, String),
}
use rayon::prelude::IntoParallelIterator;
use Action::*;

impl Action {
    fn get_args(&self) -> Option<(&String, &String)> {
        match self {
            Add(a, b) => Some((a, b)),
            Sub(a, b) => Some((a, b)),
            Mul(a, b) => Some((a, b)),
            Div(a, b) => Some((a, b)),
            Eq(a, b) => Some((a, b)),
            _ => None,
        }
    }

    fn compute(&self, a: i64, b: i64) -> Option<i64> {
        match self {
            Add(_, _) => Some(a + b),
            Sub(_, _) => Some(a - b),
            Mul(_, _) => Some(a * b),
            Div(_, _) => Some(a / b),
            Eq(_, _) => Some(if a == b { 1 } else { -1 }),
            _ => None,
        }
    }
}

fn get_value(monkeys: &mut HashMap<String, Action>, monkey: &String) -> i64 {
    let action = if let Some(action) = monkeys.get(monkey) {
        action.clone()
    } else {
        return 0;
    };

    if let Yell(n) = action {
        return n;
    }

    let (a, b) = action.get_args().unwrap();
    let a = get_value(monkeys, a);
    let b = get_value(monkeys, b);
    let value = action.compute(a, b).unwrap();
    monkeys.insert(monkey.clone(), Yell(value));
    return value;

    0
}

fn get_value_safe(monkeys: &mut HashMap<String, Action>, monkey: &String) -> Option<i64> {
    let action = monkeys.get(monkey)?.clone();

    if let Yell(n) = action {
        return Some(n);
    }

    let (a, b) = action.get_args().unwrap();
    let a = get_value_safe(monkeys, a)?;
    let b = get_value_safe(monkeys, b)?;
    let value = action.compute(a, b).unwrap();
    monkeys.insert(monkey.clone(), Yell(value));
    Some(value)
}

pub fn solve_a(input: String) -> i64 {
    let mut monkeys = input
        .lines()
        .map(|line| {
            let (name, line) = line.split_once(": ").unwrap();
            let action = match line.parse() {
                Ok(n) => Yell(n),
                Err(_) => {
                    let parts = line.split_whitespace().collect::<Vec<_>>();
                    let arg0 = parts[0].to_string();
                    let arg1 = parts[2].to_string();

                    match parts[1] {
                        "+" => Add(arg0, arg1),
                        "-" => Sub(arg0, arg1),
                        "*" => Mul(arg0, arg1),
                        "/" => Div(arg0, arg1),
                        _ => panic!(),
                    }
                }
            };
            (name.to_string(), action)
        })
        .collect::<HashMap<_, _>>();

    get_value(&mut monkeys, &"root".to_string())
}

/// i thinks there's a range of solutions for this one. e.g. the sample has solutions 301 and 302
pub fn solve_b(input: String) -> i64 {
    let mut monkeys = input
        .lines()
        .map(|line| {
            let (name, line) = line.split_once(": ").unwrap();
            let action = match line.parse() {
                Ok(n) => Yell(n),
                Err(_) => {
                    let parts = line.split_whitespace().collect::<Vec<_>>();
                    let arg0 = parts[0].to_string();
                    let arg1 = parts[2].to_string();

                    match parts[1] {
                        "+" => Add(arg0, arg1),
                        "-" => Sub(arg0, arg1),
                        "*" => Mul(arg0, arg1),
                        "/" => Div(arg0, arg1),
                        _ => panic!(),
                    }
                }
            };
            (name.to_string(), action)
        })
        .collect::<HashMap<_, _>>();
    let root_action = monkeys.get("root").unwrap().clone();
    let (a, b) = root_action.get_args().unwrap();
    let root = "root".to_string();
    let human = "humn".to_string();
    monkeys.insert(root.clone(), Eq(a.clone(), b.clone()));
    monkeys.remove(&human);

    // dbg!(&monkeys.values().filter(|a| matches!(a, Yell(_))).count());
    let _ = get_value_safe(&mut monkeys, &root);
    // dbg!(&monkeys.values().filter(|a| matches!(a, Yell(_))).count());

    {
        let mut monkeys = monkeys.clone();
        monkeys.insert(human.clone(), Yell(3352886133831));
        dbg!(get_value(&mut monkeys, &root));
    }

    let (left_monkey, right_monkey) = monkeys.get(&root).unwrap().get_args().unwrap();

    let mut left = 0;
    let mut right = 10_000_000_000_000;

    let mut result = 0;

    while left < right {
        let middle = (left + right) / 2 + 1;
        let mut monkeys = monkeys.clone();
        monkeys.insert(human.clone(), Yell(middle));
        let left_monkey = get_value(&mut monkeys, left_monkey);
        let right_monkey = get_value(&mut monkeys, right_monkey);
        println!("l{left_monkey} r{right_monkey}");

        if left_monkey == right_monkey {
            let result = middle;
            break;
        }

        if left_monkey < right_monkey {
            right = middle + 1;
        } else {
            left = middle - 1;
        }
    }

    while result > 0 && {
        let mut monkeys = monkeys.clone();
        monkeys.insert(human.clone(), Yell(result));
        let left_monkey = get_value(&mut monkeys, left_monkey);
        let right_monkey = get_value(&mut monkeys, right_monkey);
        println!("l{left_monkey} r{right_monkey}");
        left_monkey == right_monkey
    } {
        result -= 1;
    }

    // (0..=10_000_000_000_000)
    //     .into_par_iter()
    //     .find_any(|n| {
    //         let mut monkeys = monkeys.clone();
    //         monkeys.insert(human.clone(), Yell(*n));
    //         get_value(&mut monkeys, &root) == 1
    //     })
    //     .unwrap_or(-1)
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[ignore]
    fn a_eg_1() {
        assert_eq!(solve_a(get_sample_input()), 152);
    }

    #[test]
    #[ignore]
    fn a() {
        assert_eq!(solve_a(get_input()), 158661812617812);
    }

    #[test]
    #[ignore]
    fn b_eg_1() {
        assert_eq!(solve_b(get_sample_input()), 301);
    }

    #[test]
    #[ignore]
    fn b() {
        assert_eq!(solve_b(get_input()), 3352886133831);
    }

    //
}
