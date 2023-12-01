use std::collections::{HashMap, HashSet};

pub fn get_input() -> String {
    String::from(include_str!("../input"))
}

pub fn get_sample_input() -> String {
    String::from(include_str!("../sample"))
}

#[derive(Debug, Clone, Copy)]
enum Dir {
    North,
    South,
    West,
    East,
}

impl Dir {
    fn get_diffs(&self) -> Vec<(i64, i64)> {
        (-1..=1)
            .map(|i| match self {
                Dir::North => (i, -1),
                Dir::South => (i, 1),
                Dir::West => (-1, i),
                Dir::East => (1, i),
            })
            .collect()
    }
}

trait AddExt {
    fn add(&self, rhs: &Self) -> Self;
}

impl AddExt for (i64, i64) {
    fn add(&self, rhs: &Self) -> Self {
        (self.0 + rhs.0, self.1 + rhs.1)
    }
}

fn print_elves(elves: &HashSet<(i64, i64)>) {
    let min_x = elves.iter().map(|e| e.0).min().unwrap();
    let max_x = elves.iter().map(|e| e.0).max().unwrap();
    let min_y = elves.iter().map(|e| e.1).min().unwrap();
    let max_y = elves.iter().map(|e| e.1).max().unwrap();

    for x in min_x..=max_x {
        for y in min_y..=max_y {
            print!("{}", if elves.contains(&(x, y)) { "#" } else { "." })
        }
        println!()
    }
}

pub fn solve_a(input: String) -> i64 {
    let mut elves = input
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.char_indices().filter_map(move |(x, c)| {
                if c == '#' {
                    Some((x as i64, y as i64))
                } else {
                    None
                }
            })
        })
        .collect::<HashSet<_>>();

    let dir_test_order = vec![Dir::North, Dir::South, Dir::West, Dir::East];

    print_elves(&elves);
    println!("{}", "=".repeat(20));

    for i in 0..10 {
        let mut moves: HashMap<(i64, i64), Vec<(i64, i64)>> = HashMap::new();

        for elf in elves.iter() {
            let mut order_it = dir_test_order.iter().cycle().skip(i).take(4);

            if let Some(dir_to_move) = order_it.find(|d| {
                d.get_diffs()
                    .iter()
                    .map(|diff| elf.add(&diff))
                    .all(|p| !elves.contains(&p))
            }) {
                let moved = elf.add(&dir_to_move.get_diffs()[0]);
                println!("{elf:?} {moved:?}");
                moves.entry(moved).or_default().push(elf.clone())
            }
        }

        println!("{moves:?}");

        let allowed = moves
            .iter()
            .filter(|(_dest, elves)| elves.len() == 1)
            .map(|(dest, elves)| (elves[0], *dest))
            .collect::<HashMap<_, _>>();

        println!("{allowed:?}");

        elves = elves
            .iter()
            .map(|elf| *allowed.get(elf).unwrap_or(&elf))
            .collect();

        println!("{}", "=".repeat(20));
        print_elves(&elves);
    }

    0
}

pub fn solve_b(input: String) -> i64 {
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    // #[ignore]
    fn a_eg_1() {
        assert_eq!(solve_a(get_sample_input()), 0);
    }

    #[test]
    #[ignore]
    fn a() {
        assert_eq!(solve_a(get_input()), 0);
    }

    #[test]
    #[ignore]
    fn b_eg_1() {
        assert_eq!(solve_b(get_sample_input()), 0);
    }

    #[test]
    #[ignore]
    fn b() {
        assert_eq!(solve_b(get_input()), 0);
    }

    //
}
