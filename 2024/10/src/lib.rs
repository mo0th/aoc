use std::collections::{HashSet, VecDeque};

pub fn get_input() -> String {
    String::from(include_str!("../input"))
}

pub fn get_sample_input() -> String {
    String::from(include_str!("../sample"))
}

fn find_next_steps(height: u32, pos: (usize, usize), grid: &Vec<Vec<u32>>) -> Vec<(usize, usize)> {
    let limits = (grid.len() - 1, grid[0].len() - 1);

    let mut result = vec![];

    for dr in -1..=1i64 {
        for dc in -1..=1i64 {
            if dr.abs() == dc.abs() {
                continue;
            }

            let r = (pos.0 as i64 + dr) as usize;
            let c = (pos.1 as i64 + dc) as usize;

            if r > limits.0 || c > limits.1 {
                continue;
            }

            if grid[r][c] == height + 1 {
                result.push((r, c));
            }
        }
    }

    result
}

const PEAK: u32 = 9;

fn find_reachable_peaks(start: (usize, usize), grid: &Vec<Vec<u32>>) -> HashSet<(usize, usize)> {
    let mut peaks = HashSet::new();

    let mut queue = VecDeque::new();
    queue.push_back((0u32, vec![start]));

    while !queue.is_empty() {
        let (height, pos) = queue.pop_front().unwrap();

        if height == PEAK {
            peaks.insert(*pos.last().unwrap());
        } else {
            let next = find_next_steps(height, *pos.last().unwrap(), grid);

            queue.extend(next.into_iter().map(|n| {
                let mut pos = pos.clone();
                pos.push(n);
                (height + 1, pos)
            }));
        }
    }

    peaks
}

fn print_grid(grid: &Vec<Vec<u32>>) {
    for row in grid {
        for c in row {
            print!("{}", c);
        }

        println!();
    }
}

pub fn solve_a(input: String) -> i64 {
    let grid = input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    grid.iter()
        .enumerate()
        .flat_map(|(r, row)| {
            row.iter()
                .enumerate()
                .filter_map(move |(c, cell)| if cell == &0 { Some((r, c)) } else { None })
        })
        .map(|(r, c)| find_reachable_peaks((r, c), &grid).len() as i64)
        .sum()
}

fn find_reachable_paths(
    start: (usize, usize),
    grid: &Vec<Vec<u32>>,
) -> HashSet<Vec<(usize, usize)>> {
    let mut paths = HashSet::new();

    let mut queue = VecDeque::new();
    queue.push_back((0u32, vec![start]));

    while !queue.is_empty() {
        let (height, pos) = queue.pop_front().unwrap();

        if height == PEAK {
            paths.insert(pos);
        } else {
            let next = find_next_steps(height, *pos.last().unwrap(), grid);

            queue.extend(next.into_iter().map(|n| {
                let mut pos = pos.clone();
                pos.push(n);
                (height + 1, pos)
            }));
        }
    }

    paths
}

pub fn solve_b(input: String) -> i64 {
    let grid = input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    grid.iter()
        .enumerate()
        .flat_map(|(r, row)| {
            row.iter()
                .enumerate()
                .filter_map(move |(c, cell)| if cell == &0 { Some((r, c)) } else { None })
        })
        .map(|(r, c)| find_reachable_paths((r, c), &grid).len() as i64)
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[ignore]
    fn a_eg_1() {
        assert_eq!(
            solve_a(
                "0123
1234
8765
9876"
                    .to_string()
            ),
            1
        );
    }

    #[test]
    fn a_eg_2() {
        assert_eq!(solve_a(get_sample_input()), 36);
    }

    #[test]
    fn a() {
        assert_eq!(solve_a(get_input()), 796);
    }

    #[test]
    fn b_eg_1() {
        assert_eq!(solve_b(get_sample_input()), 81);
    }

    #[test]
    fn b() {
        assert_eq!(solve_b(get_input()), 1942);
    }

    //
}
