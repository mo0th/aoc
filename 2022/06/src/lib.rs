use std::collections::HashSet;

pub fn get_input() -> String {
    String::from(include_str!("../input"))
}

pub fn get_sample_input() -> String {
    String::from(include_str!("../sample"))
}

pub fn solve_a(input: String) -> i64 {
    input
        .chars()
        .collect::<Vec<_>>()
        .windows(4)
        .enumerate()
        .find(|(i, window)| {
            if window.iter().collect::<HashSet<_>>().len() == window.len() {
                true
            } else {
                false
            }
        })
        .unwrap()
        .0 as i64
        + 4
}

pub fn solve_b(input: String) -> i64 {
    input
        .chars()
        .collect::<Vec<_>>()
        .windows(14)
        .enumerate()
        .find(|(i, window)| {
            if window.iter().collect::<HashSet<_>>().len() == window.len() {
                true
            } else {
                false
            }
        })
        .unwrap()
        .0 as i64
        + 14
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_eg_1() {
        assert_eq!(solve_a(get_sample_input()), 7);
    }

    #[test]
    fn a() {
        assert_eq!(solve_a(get_input()), 1876);
    }

    #[test]
    fn b_eg_1() {
        assert_eq!(solve_b(get_sample_input()), 19);
    }

    #[test]
    fn b() {
        assert_eq!(solve_b(get_input()), 2202);
    }

    //
}
