use std::{
    collections::{HashSet, VecDeque},
    iter::once,
};

pub fn get_input() -> String {
    String::from(include_str!("../input"))
}

pub fn get_sample_input() -> String {
    String::from(include_str!("../sample"))
}

#[derive(Debug)]
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
        }
    }

    visited.len() as i64
}

pub fn solve_b(input: String) -> i64 {
    let mut rope = vec![(0, 0); 10];
    let rope_len = rope.len();
    let mut visited = HashSet::<Point>::new();

    visited.insert(*rope.last().unwrap());

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

    for (dir, amt) in moves.into_iter() {
        let add = match dir {
            Dir::UP => (0, -1),
            Dir::DOWN => (0, 1),
            Dir::LEFT => (-1, 0),
            Dir::RIGHT => (1, 0),
        };

        for _ in 0..amt {
            let new_head = (rope[0].0 + add.0, rope[0].1 + add.1);
            rope[0] = new_head;

            for i in 1..rope_len {
                let curr = rope[i];
                let prev = rope[i - 1];
                let (mut curr_x, mut curr_y) = curr;

                if !is_adjacent_or_covering(prev, curr) {
                    let (prev_x, prev_y) = prev;

                    if prev_x > curr_x {
                        curr_x += 1;
                    } else if prev_x < curr_x {
                        curr_x -= 1;
                    }

                    if prev_y > curr_y {
                        curr_y += 1;
                    } else if prev_y < curr_y {
                        curr_y -= 1;
                    }
                }

                rope[i] = (curr_x, curr_y);
            }

            visited.insert(*rope.last().unwrap());
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
    // #[ignore]
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
    fn b() {
        assert_eq!(solve_b(get_input()), 2678);
    }

    //
}
