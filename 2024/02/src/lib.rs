pub fn get_input() -> String {
    String::from(include_str!("../input"))
}

pub fn get_sample_input() -> String {
    String::from(include_str!("../sample"))
}

fn is_monotonic(nums: &[i64]) -> bool {
    nums.windows(2).all(|w| w[0] < w[1]) || nums.windows(2).all(|w| w[0] > w[1])
}

fn is_safe(nums: &[i64]) -> bool {
    let diffs = nums
        .windows(2)
        .map(|w| w[0].abs_diff(w[1]))
        .all(|d| d >= 1 && d <= 3);
    return is_monotonic(nums) && diffs;
}

pub fn solve_a(input: String) -> i64 {
    input
        .lines()
        .filter(|line| {
            let nums: Vec<_> = line
                .split_whitespace()
                .map(|x| x.parse::<i64>().unwrap())
                .collect();

            return is_safe(&nums);
        })
        .count()
        .try_into()
        .unwrap()
}

pub fn solve_b(input: String) -> i64 {
    input
        .lines()
        .filter(|line| {
            let nums: Vec<_> = line
                .split_whitespace()
                .map(|x| x.parse::<i64>().unwrap())
                .collect();

            let is = nums.iter().enumerate().map(|(i, _)| i);

            for i in is {
                if is_safe(
                    nums.iter()
                        .take(i)
                        .chain(nums.iter().skip(i + 1))
                        .map(|x| *x)
                        .collect::<Vec<_>>()
                        .as_slice(),
                ) {
                    return true;
                }
            }

            return false;
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
        assert_eq!(solve_a(get_input()), 252);
    }

    #[test]
    fn b_eg_1() {
        assert_eq!(solve_b(get_sample_input()), 4);
    }

    #[test]
    fn b() {
        assert_eq!(solve_b(get_input()), 324);
    }

    //
}
