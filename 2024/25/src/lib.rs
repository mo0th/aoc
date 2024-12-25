pub fn get_input() -> String {
    String::from(include_str!("../input"))
}

pub fn get_sample_input() -> String {
    String::from(include_str!("../sample"))
}

#[derive(Debug, PartialEq)]
enum Item {
    Lock(Vec<usize>),
    Key(Vec<usize>),
}

const LOCK_HEIGHT: usize = 5;
const LOCK_WIDTH: usize = 5;

pub fn solve_a(input: String) -> i64 {
    let input = input
        .split("\n\n")
        .map(|key_or_lock_str| {
            let lines = key_or_lock_str.lines().collect::<Vec<_>>();
            let is_key = lines[0] == ".....";

            let counts = (0..LOCK_WIDTH)
                .map(|i| {
                    (1..=LOCK_HEIGHT)
                        .filter(|j| {
                            let ch = lines[*j].get(i..i + 1);
                            ch == Some("#")
                        })
                        .count()
                })
                .collect::<Vec<_>>();

            if is_key {
                Item::Key(counts)
            } else {
                Item::Lock(counts)
            }
        })
        .collect::<Vec<_>>();

    let locks = input
        .iter()
        .filter_map(|item| match item {
            Item::Lock(counts) => Some(counts),
            _ => None,
        })
        .collect::<Vec<_>>();

    let keys = input
        .iter()
        .filter_map(|item| match item {
            Item::Key(counts) => Some(counts),
            _ => None,
        })
        .collect::<Vec<_>>();

    locks
        .iter()
        .map(|lock| {
            keys.iter()
                .filter(|key| {
                    lock.iter()
                        .zip(key.iter())
                        .all(|(lock_pin, key_pin)| (lock_pin + key_pin) <= LOCK_HEIGHT)
                })
                .count() as i64
        })
        .sum()
}

pub fn solve_b(_input: String) -> i64 {
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_eg_1() {
        assert_eq!(solve_a(get_sample_input()), 3);
    }

    #[test]
    fn a() {
        assert_eq!(solve_a(get_input()), 3356);
    }

    #[test]
    #[ignore]
    fn b_eg_1() {
        assert_eq!(solve_b(get_sample_input()), 1);
    }

    #[test]
    #[ignore]
    fn b() {
        assert_eq!(solve_b(get_input()), 1);
    }

    //
}
