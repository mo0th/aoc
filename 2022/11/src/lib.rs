use std::collections::HashMap;

#[derive(Debug, Clone)]
enum Op {
    Add(i128),
    Mult(i128),
    Square,
}
use Op::*;

impl Op {
    fn run(&self, old: i128) -> i128 {
        match self {
            Add(n) => old + (*n),
            Mult(n) => old * (*n),
            Square => old * old,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Monkey {
    items: Vec<i128>,
    op: Op,
    test_num: i128,
    true_target: usize,
    false_target: usize,
}

impl Monkey {
    fn test(&self, worry: i128) -> usize {
        if worry % self.test_num == 0 {
            self.true_target
        } else {
            self.false_target
        }
    }
}

pub fn get_input() -> Vec<Monkey> {
    vec![
        Monkey {
            items: vec![83, 88, 96, 79, 86, 88, 70],
            op: Mult(5),
            test_num: 11,
            true_target: 2,
            false_target: 3,
        },
        Monkey {
            items: vec![59, 63, 98, 85, 68, 72],
            op: Mult(11),
            test_num: 5,
            true_target: 4,
            false_target: 0,
        },
        Monkey {
            items: vec![90, 79, 97, 52, 90, 94, 71, 70],
            op: Add(2),
            test_num: 19,
            true_target: 5,
            false_target: 6,
        },
        Monkey {
            items: vec![97, 55, 62],
            op: Add(5),
            test_num: 13,
            true_target: 2,
            false_target: 6,
        },
        Monkey {
            items: vec![74, 54, 94, 76],
            op: Square,
            test_num: 7,
            true_target: 0,
            false_target: 3,
        },
        Monkey {
            items: vec![58],
            op: Add(4),
            test_num: 17,
            true_target: 7,
            false_target: 1,
        },
        Monkey {
            items: vec![66, 63],
            op: Add(6),
            test_num: 2,
            true_target: 7,
            false_target: 5,
        },
        Monkey {
            items: vec![56, 56, 90, 96, 68],
            op: Add(7),
            test_num: 3,
            true_target: 4,
            false_target: 1,
        },
    ]
}

pub fn get_sample_input() -> Vec<Monkey> {
    vec![
        Monkey {
            items: vec![79, 98],
            op: Mult(19),
            test_num: 23,
            true_target: 2,
            false_target: 3,
        },
        Monkey {
            items: vec![54, 65, 75, 74],
            op: Add(6),
            test_num: 19,
            true_target: 2,
            false_target: 0,
        },
        Monkey {
            items: vec![79, 60, 97],
            op: Square,
            test_num: 13,
            true_target: 1,
            false_target: 3,
        },
        Monkey {
            items: vec![74],
            op: Add(3),
            test_num: 17,
            true_target: 0,
            false_target: 1,
        },
    ]
}

pub fn solve_a(input: Vec<Monkey>) -> i128 {
    let mut inspected = vec![0; input.len()];
    let mut monkeys = input.clone();

    for _ in 0..20 {
        let mut additional_items: HashMap<usize, Vec<i128>> = HashMap::new();

        for (i, monkey) in monkeys.iter_mut().enumerate() {
            let mut items = monkey.items.clone();
            if let Some(more_items) = additional_items.remove(&i) {
                items.extend(more_items);
            }
            for item in items.iter() {
                let new_worry = monkey.op.run(*item) / 3;
                let target = monkey.test(new_worry);

                let vec = additional_items.entry(target).or_default();
                (*vec).push(new_worry);
            }
            inspected[i] += items.len();
            monkey.items = vec![];
        }

        for (i, new_items) in additional_items {
            monkeys[i].items = new_items;
        }
    }

    inspected.sort();
    inspected.iter().rev().take(2).product::<usize>() as i128
}

pub fn solve_b(input: Vec<Monkey>) -> i128 {
    let mut inspected = vec![0; input.len()];
    let mut monkeys = input.clone();

    let md = monkeys.iter().map(|m| m.test_num).product::<i128>();

    for _ in 0..10_000 {
        let mut additional_items: HashMap<usize, Vec<i128>> = HashMap::new();

        for (i, monkey) in monkeys.iter_mut().enumerate() {
            let mut items = monkey.items.clone();
            if let Some(more_items) = additional_items.remove(&i) {
                items.extend(more_items);
            }
            for item in items.iter() {
                let new_worry = monkey.op.run(*item) % md;
                let target = monkey.test(new_worry);

                let vec = additional_items.entry(target).or_default();
                (*vec).push(new_worry);
            }
            inspected[i] += items.len();
            monkey.items = vec![];
        }

        for (i, new_items) in additional_items {
            monkeys[i].items = new_items;
        }
    }

    inspected.sort();
    inspected.iter().rev().take(2).product::<usize>() as i128
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[ignore]
    fn a_eg_1() {
        assert_eq!(solve_a(get_sample_input()), 10605);
    }

    #[test]
    #[ignore]
    fn a() {
        assert_eq!(solve_a(get_input()), 64032);
    }

    #[test]
    #[ignore]
    fn b_eg_1() {
        assert_eq!(solve_b(get_sample_input()), 2713310158);
    }

    #[test]
    // #[ignore]
    fn b() {
        assert_eq!(solve_b(get_input()), 12729522272);
    }

    //
}
