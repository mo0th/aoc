pub fn get_input() -> String {
    String::from(include_str!("../input"))
}

pub fn get_sample_input() -> String {
    String::from(include_str!("../sample"))
}

pub fn solve_a(input: String) -> i64 {
    return input
        .lines()
        .map(|line| {
            let digits = line
                .chars()
                .filter(|ch| ch.is_digit(10))
                .collect::<Vec<_>>();
            let mut first = digits.first().unwrap().to_string();
            let last = digits.last().unwrap().to_string();
            first.push_str(&last);
            return first.parse::<i64>().unwrap();
        })
        .sum();
}

pub fn solve_b(input: String) -> i64 {
    let numbers = [
        ("one", "1"),
        ("two", "2"),
        ("three", "3"),
        ("four", "4"),
        ("five", "5"),
        ("six", "6"),
        ("seven", "7"),
        ("eight", "8"),
        ("nine", "9"),
    ];

    let mut sum = 0;

    for line in input.lines().map(|l| l.to_string()) {
        let mut first: Option<String> = None;

        for i in 0..line.len() {
            let ch = line.chars().nth(i).unwrap();
            if ch.is_digit(10) {
                first = Some(ch.into())
            }

            if let Some((_, n)) = numbers.iter().find(|(s, _)| line[i..].starts_with(s)) {
                first = n.to_string().into()
            }

            if first.is_some() {
                break;
            }
        }

        let mut last: Option<String> = None;

        for i in (0..line.len()).rev() {
            let ch = line.chars().nth(i).unwrap();
            if ch.is_digit(10) {
                last = Some(ch.into())
            }

            let slice = &line[..=i];

            if let Some((_, n)) = numbers.iter().find(|(s, _)| slice.ends_with(s)) {
                last = n.to_string().into()
            }

            if last.is_some() {
                break;
            }
        }

        match (first, last) {
            (Some(first), Some(last)) => {
                let num = first
                    .chars()
                    .chain(last.chars())
                    .collect::<String>()
                    .parse::<i64>()
                    .unwrap();
                sum += num;
            }
            _ => (),
        };
    }

    return sum;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_eg_1() {
        assert_eq!(solve_a(get_sample_input()), 142);
    }

    #[test]
    fn a() {
        assert_eq!(solve_a(get_input()), 54388);
    }

    #[test]
    fn b_eg_1() {
        assert_eq!(
            solve_b(
                "two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen"
                    .to_string()
            ),
            281
        );
    }

    #[test]
    fn b_eg_2() {
        assert_eq!(solve_b("seven2tjf".into()), 72)
    }

    #[test]
    fn b() {
        assert_eq!(solve_b(get_input()), 53515);
    }

    //
}
