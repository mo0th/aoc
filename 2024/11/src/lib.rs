use std::collections::HashMap;

use rayon::iter::{IntoParallelRefIterator, ParallelIterator};

pub fn get_input() -> String {
    String::from(include_str!("../input"))
}

pub fn get_sample_input() -> String {
    String::from(include_str!("../sample"))
}

fn next_stones_b(stone: i64) -> Vec<i64> {
    if stone == 0 {
        return vec![1];
    } else if stone.to_string().len() % 2 == 0 {
        let str = stone.to_string();

        let (left, right) = str.split_at(str.len() / 2);

        return vec![left.parse().unwrap(), right.parse().unwrap()];
    } else {
        return vec![stone * 2024];
    }
}

fn next_stones_a(stones: &Vec<i64>) -> Vec<i64> {
    stones
        .iter()
        .flat_map(|stone| {
            if stone == &0 {
                return vec![1];
            } else if stone.to_string().len() % 2 == 0 {
                let str = stone.to_string();

                let (left, right) = str.split_at(str.len() / 2);

                return vec![left.parse().unwrap(), right.parse().unwrap()];
            } else {
                return vec![stone * 2024];
            }
        })
        .collect()
}

pub fn solve_a(input: String) -> i64 {
    let stones = input
        .lines()
        .nth(0)
        .unwrap()
        .split_whitespace()
        .map(|s| s.parse::<i64>().unwrap())
        .collect::<Vec<_>>();

    stones
        .par_iter()
        .map(|stone| {
            let mut stones = vec![*stone];

            for _ in 1..=25 {
                stones = next_stones_a(&stones);
            }

            stones.len()
        })
        .sum::<usize>() as i64
}

fn count_stones(
    stone: i64,
    max_depth: i64,
    depth: i64,
    path: Vec<(i64, i64)>,
    cache: &mut HashMap<i64, HashMap<i64, i64>>,
) -> i64 {
    if depth == max_depth {
        return 1;
    }

    if cache.contains_key(&stone) && cache[&stone].contains_key(&depth) {
        return cache[&stone][&depth];
    }

    let next = next_stones_b(stone);

    let mut next_path = path.clone();
    next_path.push((depth, stone));

    let result = next
        .iter()
        .map(|s| count_stones(*s, max_depth, depth + 1, next_path.clone(), cache))
        .sum::<i64>();

    for (depth, stone) in next_path {
        cache.entry(stone).or_default().insert(depth, result);
    }

    result
}

pub fn solve_b(input: String) -> i64 {
    let mut cache = HashMap::new();

    input
        .lines()
        .nth(0)
        .unwrap()
        .split_whitespace()
        .map(|s| s.parse::<i64>().unwrap())
        .map(|s| count_stones(s, 75, 0, vec![], &mut cache))
        .sum::<i64>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_eg_1() {
        assert_eq!(solve_a(get_sample_input()), 55312);
    }

    #[test]
    fn a() {
        assert_eq!(solve_a(get_input()), 191690);
    }

    #[test]
    fn b_eg_1() {
        assert_eq!(solve_b(get_sample_input()), 65601038650482);
    }

    #[test]
    fn b() {
        assert_eq!(solve_b(get_input()), 228651922369703);
    }

    //
}
