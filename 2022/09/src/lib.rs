use std::{
    collections::{HashSet, VecDeque},
    iter::FromIterator,
};

pub fn get_input() -> String {
    String::from(include_str!("../input"))
}

pub fn get_sample_input() -> String {
    String::from(include_str!("../sample"))
}

enum Dir {
    UP,
    DOWN,
    LEFT,
    RIGHT,
}

type Point = (i64, i64);

fn is_adjacent_or_covering(a: Point, b: Point) -> bool {
    return (a.0 - b.0).abs() <= 1 && (a.1 - b.1).abs() <= 1;
}

pub fn solve_a(input: String) -> i64 {
    let mut head = (0, 0);
    let mut tail = (0, 0);
    let mut visited = HashSet::<Point>::new();

    visited.insert(tail);

    let moves = input
        .lines()
        .map(|line| {
            line.split_once(" ")
                .map(|(dir, amt)| {
                    (
                        match dir {
                            "U" => Dir::UP,
                            "D" => Dir::DOWN,
                            "L" => Dir::LEFT,
                            "R" => Dir::RIGHT,
                            _ => panic!(),
                        },
                        amt.parse::<i64>().unwrap(),
                    )
                })
                .unwrap()
        })
        .collect::<Vec<_>>();

    for (dir, amt) in moves {
        let add = match dir {
            Dir::UP => (0, -1),
            Dir::DOWN => (0, 1),
            Dir::LEFT => (-1, 0),
            Dir::RIGHT => (1, 0),
            _ => todo!(),
        };

        for _ in 0..amt {
            let next_head = (head.0 + add.0, head.1 + add.1);
            let mut next_tail = tail.clone();

            if !is_adjacent_or_covering(next_head, tail) {
                next_tail = head.clone();
            }

            head = next_head;
            tail = next_tail;
            visited.insert(next_tail.clone());
            // println!("{head:?} {tail:?}");
        }
    }

    visited.len() as i64
}

pub fn solve_b(input: String) -> i64 {
    let mut head = (0, 0);
    let mut tail = VecDeque::from_iter((0..9).map(|_| head.clone()));
    let mut visited = HashSet::<Point>::new();

    visited.insert(tail[0]);

    let moves = input
        .lines()
        .map(|line| {
            line.split_once(" ")
                .map(|(dir, amt)| {
                    (
                        match dir {
                            "U" => Dir::UP,
                            "D" => Dir::DOWN,
                            "L" => Dir::LEFT,
                            "R" => Dir::RIGHT,
                            _ => panic!(),
                        },
                        amt.parse::<i64>().unwrap(),
                    )
                })
                .unwrap()
        })
        .collect::<Vec<_>>();

    for (dir, amt) in moves {
        let add = match dir {
            Dir::UP => (0, -1),
            Dir::DOWN => (0, 1),
            Dir::LEFT => (-1, 0),
            Dir::RIGHT => (1, 0),
            _ => todo!(),
        };

        for _ in 0..amt {
            let next_head = (head.0 + add.0, head.1 + add.1);

            if !is_adjacent_or_covering(next_head, tail[0]) {
                tail.push_front(head.clone());

                if tail.len() > 9 {
                    tail.pop_back();
                }
            }

            head = next_head;
            visited.insert(tail[0].clone());
            println!("{head:?} {tail:?}");
        }
    }

    visited.len() as i64
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[ignore]
    fn a_eg_1() {
        assert_eq!(solve_a(get_sample_input()), 13);
    }

    #[test]
    #[ignore]
    fn a() {
        assert_eq!(solve_a(get_input()), 6486);
    }

    #[test]
    #[ignore]
    fn b_eg_1() {
        assert_eq!(
            solve_b(
                "R 5
U 8
L 8
D 3
R 17
D 10
L 25
U 20"
                    .to_string()
            ),
            36
        );
    }

    #[test]
    // #[ignore]
    fn b_eg_2() {
        assert_eq!(solve_b(get_sample_input()), 36);
    }

    #[test]
    #[ignore]
    fn b() {
        assert_eq!(solve_b(get_input()), 0);
    }

    //
}
