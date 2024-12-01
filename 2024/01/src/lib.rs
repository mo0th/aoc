use std::collections::HashMap;

pub fn get_input() -> String {
    String::from(include_str!("../input"))
}

pub fn get_sample_input() -> String {
    String::from(include_str!("../sample"))
}

pub fn solve_a(input: String) -> i64 {
    let pairs: Vec<_> = input
        .lines()
        .map(|line| {
            let nums: Vec<_> = line
                .split_ascii_whitespace()
                .map(|x| x.parse::<i64>().unwrap())
                .collect();

            (nums[0], nums[1])
        })
        .collect();

    let mut first_col = pairs.iter().map(|(a, _)| *a).collect::<Vec<_>>();
    first_col.sort();
    let mut second_col = pairs.iter().map(|(_, b)| *b).collect::<Vec<_>>();
    second_col.sort();

    let diffs = first_col
        .iter()
        .zip(second_col.iter())
        .map(|(a, b)| a.abs_diff(*b) as i64);

    diffs.sum()
}

pub fn solve_b(input: String) -> i64 {
    let pairs: Vec<_> = input
        .lines()
        .map(|line| {
            let nums: Vec<_> = line
                .split_ascii_whitespace()
                .map(|x| x.parse::<i64>().unwrap())
                .collect();

            (nums[0], nums[1])
        })
        .collect();

    let first_col = pairs.iter().map(|(a, _)| *a).collect::<Vec<_>>();
    let second_col = pairs.iter().map(|(_, b)| *b).collect::<Vec<_>>();
    let mut counts: HashMap<i64, i64> = HashMap::new();
    for n in second_col {
        *counts.entry(n).or_insert(0) += 1;
    }

    return first_col
        .iter()
        .map(|n| {
            return n * counts.get(n).unwrap_or(&0);
        })
        .sum();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_eg_1() {
        assert_eq!(solve_a(get_sample_input()), 11);
    }

    #[test]
    fn a() {
        assert_eq!(solve_a(get_input()), 1222801);
    }

    #[test]
    fn b_eg_1() {
        assert_eq!(solve_b(get_sample_input()), 31);
    }

    #[test]
    fn b() {
        assert_eq!(solve_b(get_input()), 22545250);
    }

    //
}
