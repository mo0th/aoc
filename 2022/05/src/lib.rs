use std::borrow::Borrow;

pub fn get_input() -> String {
    String::from(include_str!("../input"))
}

pub fn get_sample_input() -> String {
    String::from(include_str!("../sample"))
}

pub fn solve_a(input: String) -> String {
    let (init, moves) = input.split_once("\n\n").unwrap();
    let init: Vec<_> = init
        .lines()
        .map(|line| {
            let mut line = line.to_string();
            line.push_str(" ");

            line.chars()
                .collect::<Vec<_>>()
                .chunks(4)
                .map(|chunk| chunk[1])
                .collect::<Vec<_>>()
        })
        .rev()
        .skip(1)
        .collect();

    let mut stacks: Vec<Vec<char>> = vec![];

    for _ in init[0].iter() {
        stacks.push(vec![]);
    }

    for line in init.iter() {
        for (i, ch) in line.iter().enumerate() {
            if ch != &' ' {
                stacks[i].push(*ch);
            }
        }
    }

    for m in moves.lines() {
        let m = m
            .replace("move", "")
            .replace("from", "")
            .replace("to", "")
            .trim()
            .split_whitespace()
            .map(|s| s.parse::<i64>().unwrap())
            .collect::<Vec<_>>();
        let amount = m[0];
        let from = m[1] - 1;
        let to = m[2] - 1;

        for _ in 0..amount {
            let to_move = stacks[from as usize].pop().unwrap();
            stacks[to as usize].push(to_move);
        }
    }

    stacks.iter().map(|stack| stack.last().unwrap()).collect()
}

pub fn solve_b(input: String) -> String {
    let (init, moves) = input.split_once("\n\n").unwrap();
    let init: Vec<_> = init
        .lines()
        .map(|line| {
            let mut line = line.to_string();
            line.push_str(" ");

            line.chars()
                .collect::<Vec<_>>()
                .chunks(4)
                .map(|chunk| chunk[1])
                .collect::<Vec<_>>()
        })
        .rev()
        .skip(1)
        .collect();

    let mut stacks: Vec<Vec<char>> = vec![];

    for _ in init[0].iter() {
        stacks.push(vec![]);
    }

    for line in init.iter() {
        for (i, ch) in line.iter().enumerate() {
            if ch != &' ' {
                stacks[i].push(*ch);
            }
        }
    }

    for m in moves.lines() {
        let m = m
            .replace("move", "")
            .replace("from", "")
            .replace("to", "")
            .trim()
            .split_whitespace()
            .map(|s| s.parse::<i64>().unwrap())
            .collect::<Vec<_>>();

        let amount = m[0];
        let from = (m[1] - 1) as usize;
        let to = (m[2] - 1) as usize;

        let mut to_move: Vec<char> = vec![];
        for _ in 0..amount {
            to_move.push(stacks[from].pop().unwrap());
        }
        stacks[to].extend(to_move.iter().rev());
    }

    stacks.iter().map(|stack| stack.last().unwrap()).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_eg_1() {
        assert_eq!(solve_a(get_sample_input()), "CMZ");
    }

    #[test]
    fn a() {
        assert_eq!(solve_a(get_input()), "HNSNMTLHQ");
    }

    #[test]
    fn b_eg_1() {
        assert_eq!(solve_b(get_sample_input()), "MCD");
    }

    #[test]
    fn b() {
        assert_eq!(solve_b(get_input()), "RNLFDJMCT");
    }

    //
}
