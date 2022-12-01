pub fn get_input() -> String {
    String::from(include_str!("../input"))
}

pub fn get_sample_input() -> String {
    String::from(include_str!("../sample"))
}

pub fn solve_a(input: String) -> i64 {
    let elves = input
        .split("\n\n")
        .map(|chunk| {
            chunk
                .lines()
                .map(|l| l.parse::<i64>().unwrap())
                .sum::<i64>()
        })
        .max()
        .unwrap();

    elves
}

pub fn solve_b(input: String) -> i64 {
    let mut elves = input
        .split("\n\n")
        .map(|chunk| {
            chunk
                .lines()
                .map(|l| l.parse::<i64>().unwrap())
                .sum::<i64>()
        })
        .collect::<Vec<_>>();

    elves.sort();

    let result = elves.pop().unwrap() + elves.pop().unwrap() + elves.pop().unwrap();

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_eg_1() {
        assert_eq!(solve_a(get_sample_input()), 24000);
    }

    #[test]
    fn a() {
        assert_eq!(solve_a(get_input()), 68467);
    }

    #[test]
    fn b_eg_1() {
        assert_eq!(solve_b(get_sample_input()), 45000);
    }

    #[test]
    fn b() {
        assert_eq!(solve_b(get_input()), 203420);
    }

    //
}
