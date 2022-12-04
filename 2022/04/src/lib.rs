use std::convert::TryInto;

type Pair = (i64, i64);

pub fn get_input() -> String {
    String::from(include_str!("../input"))
}

pub fn get_sample_input() -> String {
    String::from(include_str!("../sample"))
}

fn parse_assignment(ass: &str) -> Pair {
    let (start, end) = ass.split_once("-").unwrap();
    (start.parse().unwrap(), end.parse().unwrap())
}

pub fn solve_a(input: String) -> i64 {
    input
        .lines()
        .filter_map(|line| {
            let (first, second) = line.split_once(",").unwrap();
            let first = parse_assignment(first);
            let second = parse_assignment(second);

            if first.0 <= second.0 && second.1 <= first.1 {
                Some(())
            } else if second.0 <= first.0 && first.1 <= second.1 {
                Some(())
            } else {
                None
            }
        })
        .count()
        .try_into()
        .unwrap()
}

pub fn solve_b(input: String) -> i64 {
    input
        .lines()
        .filter_map(|line| {
            let (first, second) = line.split_once(",").unwrap();
            let first = parse_assignment(first);
            let second = parse_assignment(second);

            if (first.0 <= second.0 && second.0 <= first.1)
                || (first.0 <= second.1 && second.1 <= first.1)
            {
                Some(())
            } else if (second.0 <= first.0 && first.0 <= second.1)
                || (second.0 <= first.1 && first.1 <= second.1)
            {
                Some(())
            } else {
                None
            }
        })
        .count()
        .try_into()
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_eg_1() {
        assert_eq!(solve_a(get_sample_input()), 2);
    }

    #[test]
    fn a() {
        assert_eq!(solve_a(get_input()), 456);
    }

    #[test]
    fn b_eg_1() {
        assert_eq!(solve_b(get_sample_input()), 4);
    }

    #[test]
    fn b() {
        assert_eq!(solve_b(get_input()), 808);
    }

    //
}
