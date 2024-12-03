use regex::Regex;

pub fn get_input() -> String {
    String::from(include_str!("../input"))
}

pub fn get_sample_input() -> String {
    String::from(include_str!("../sample"))
}

pub fn solve_a(input: String) -> i64 {
    let re = Regex::new(r"mul\((?<left>\d+),(?<right>\d+)\)").unwrap();

    return re
        .captures_iter(&input)
        .map(|it| {
            let left = it.name("left").unwrap().as_str().parse::<i64>().unwrap();
            let right = it.name("right").unwrap().as_str().parse::<i64>().unwrap();
            return left * right;
        })
        .sum();
}

pub fn solve_b(input: String) -> i64 {
    let re = Regex::new(r"(mul\((?<left>\d+),(?<right>\d+)\))|(do\(\))|(don't\(\))").unwrap();

    let mut can_do = true;

    return re
        .captures_iter(&input)
        .map(|it| {
            let full_match = it.get(0).map(|it| it.as_str()).unwrap();

            if full_match == "do()" {
                can_do = true;
            } else if full_match == "don't()" {
                can_do = false;
            } else if can_do {
                let left = it.name("left").unwrap().as_str().parse::<i64>().unwrap();
                let right = it.name("right").unwrap().as_str().parse::<i64>().unwrap();
                return left * right;
            }

            return 0;
        })
        .sum();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_eg_1() {
        assert_eq!(solve_a(get_sample_input()), 161);
    }

    #[test]
    fn a() {
        assert_eq!(solve_a(get_input()), 168539636);
    }

    #[test]
    fn b_eg_1() {
        assert_eq!(
            solve_b(
                "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))"
                    .to_string()
            ),
            48
        );
    }

    #[test]
    fn b() {
        assert_eq!(solve_b(get_input()), 97529391);
    }

    //
}
