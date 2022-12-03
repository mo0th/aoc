use std::{collections::HashSet, hash::Hash};

pub fn get_input() -> String {
    String::from(include_str!("../input"))
}

pub fn get_sample_input() -> String {
    String::from(include_str!("../sample"))
}

pub fn solve_a(input: String) -> i64 {
    input
        .lines()
        .map(|line| {
            let start = line.get(0..line.len() / 2).unwrap();
            let end = line.get(line.len() / 2..line.len()).unwrap();
            let start_set = start.bytes().collect::<HashSet<_>>();

            let most_common = end.bytes().find(|ch| start_set.contains(ch)).unwrap();

            let priority = (if most_common < 96 {
                most_common - 64 + 26
            } else {
                most_common - 96
            }) as i64;

            priority
        })
        .sum::<i64>()
}

pub fn solve_b(input: String) -> i64 {
    input
        .lines()
        .collect::<Vec<_>>()
        .chunks(3)
        .map(|chunk| {
            let first_set = chunk[0].bytes().collect::<HashSet<_>>();
            let second_set: HashSet<u8> = chunk[1].bytes().collect::<HashSet<_>>();

            let common = chunk[2]
                .bytes()
                .find(|ch| first_set.contains(ch) && second_set.contains(ch))
                .unwrap();

            let priority = (if common < 96 {
                common - 64 + 26
            } else {
                common - 96
            }) as i64;

            priority
        })
        .sum::<i64>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_eg_1() {
        assert_eq!(solve_a(get_sample_input()), 157);
    }

    #[test]
    fn a() {
        assert_eq!(solve_a(get_input()), 8240);
    }

    #[test]
    fn b_eg_1() {
        assert_eq!(solve_b(get_sample_input()), 70);
    }

    #[test]
    fn b() {
        assert_eq!(solve_b(get_input()), 2587);
    }

    //
}
