use std::collections::{HashMap, HashSet, VecDeque};

pub fn get_input() -> String {
    String::from(include_str!("../input"))
}

pub fn get_sample_input() -> String {
    String::from(include_str!("../sample"))
}

fn print_map_compact(dim: (i64, i64), walls: &HashSet<(i64, i64)>, path: &[(i64, i64)]) {
    println!();
    for y in 0..dim.1 {
        for x in 0..dim.0 {
            let curr = (x, y);
            let is_wall = walls.contains(&curr);

            print!(
                "{}",
                if is_wall {
                    "#".to_string()
                } else if path.contains(&curr) {
                    "O".to_string()
                } else {
                    ".".to_string()
                }
            )
        }
        println!()
    }
    println!();
}

fn print_map(dim: (i64, i64), walls: &HashSet<(i64, i64)>, path: &[(i64, i64)]) {
    println!();
    for y in 0..dim.1 {
        for x in 0..dim.0 {
            let curr = (x, y);
            let is_wall = walls.contains(&curr);

            print!(
                "{:>3}",
                if is_wall {
                    "#".to_string()
                } else if let Some(i) = path.iter().position(|pos| pos == &curr) {
                    i.to_string()
                } else {
                    ".".to_string()
                }
            )
        }
        println!()
    }
    println!();
}

pub fn solve_a(input: String, dim: (i64, i64), n_bytes: usize) -> i64 {
    let walls = input
        .lines()
        .take(n_bytes)
        .map(|l| {
            let nums: Vec<_> = l.split(',').map(|s| s.parse::<i64>().unwrap()).collect();

            // println!("{nums:?}");

            (nums[0], nums[1])
        })
        .collect::<HashSet<_>>();

    let ok_range = 0..dim.0;

    let end = (dim.0 - 1, dim.1 - 1);

    let mut queue = VecDeque::new();
    queue.push_back(((0, 0), vec![]));
    let mut seen = HashMap::<(i64, i64), Vec<(i64, i64)>>::new();
    let mut visited = HashSet::<(i64, i64)>::new();

    while let Some((curr, steps)) = queue.pop_front() {
        if curr == end {
            // print_map(dim, &walls, &steps);
            print_map_compact(dim, &walls, &steps);

            for s in steps.iter() {
                println!("{s:?}");
            }

            return steps.len() as i64 + 1;
        }

        // if visited.contains(&curr) {
        //     // println!("visited {curr:?} already {}", steps.len());
        //     continue;
        // }

        if let Some(steps_cmp) = seen.get(&curr) {
            println!(
                "Curr {curr:?} at {}, existing at {}",
                steps_cmp.len(),
                steps.len()
            );
            if steps_cmp.len() <= (steps.len() + 1) {
                println!("skipping");
                continue;
            }
        }

        visited.insert(curr);

        seen.insert(curr, steps.clone());

        for (dx, dy) in vec![(1, 0), (0, 1), (-1, 0), (0, -1)] {
            let new = (curr.0 + dx, curr.1 + dy);

            let ok =
                ok_range.contains(&new.0) && ok_range.contains(&new.1) && !walls.contains(&new);

            if !ok {
                continue;
            }

            let mut steps = steps.clone();
            steps.push(curr.clone());
            queue.push_back((new, steps.clone()));
        }
    }

    -1
}

pub fn solve_b(input: String) -> i64 {
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[ignore]
    fn a_eg_1() {
        assert_eq!(solve_a(get_sample_input(), (7, 7), 12), 22);
    }

    #[test]
    fn a() {
        assert_eq!(solve_a(get_input(), (71, 71), 1024), 1);
    }

    #[test]
    #[ignore]
    fn b_eg_1() {
        assert_eq!(solve_b(get_sample_input()), 1);
    }

    #[test]
    #[ignore]
    fn b() {
        assert_eq!(solve_b(get_input()), 1);
    }

    //
}
