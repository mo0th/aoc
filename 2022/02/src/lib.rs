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
            let (elf, me) = line.split_once(" ").unwrap();
            let did_win =
                (elf == "A" && me == "Y") || (elf == "B" && me == "Z") || (elf == "C" && me == "X");
            let did_draw = match elf {
                "A" => "X",
                "B" => "Y",
                "C" => "Z",
                _ => "",
            } == me;
            let me = match me {
                "X" => 1,
                "Y" => 2,
                "Z" => 3,
                _ => 0,
            };

            return me
                + if did_win {
                    6
                } else if did_draw {
                    3
                } else {
                    0
                };
        })
        .sum::<i64>()
}

pub fn solve_b(input: String) -> i64 {
    input
        .lines()
        .map(|line| {
            let (elf, should_win) = line.split_once(" ").unwrap();

            let me = match should_win {
                "X" => match elf {
                    "A" => "C",
                    "B" => "A",
                    "C" => "B",
                    _ => panic!(),
                },
                "Y" => elf,
                "Z" => match elf {
                    "A" => "B",
                    "B" => "C",
                    "C" => "A",
                    _ => panic!(),
                },
                _ => panic!(),
            };
            let me = match me {
                "A" => 1,
                "B" => 2,
                "C" => 3,
                _ => panic!(),
            };

            return me
                + match should_win {
                    "X" => 0,
                    "Y" => 3,
                    "Z" => 6,
                    _ => panic!(),
                };
        })
        .sum::<i64>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_eg_1() {
        assert_eq!(solve_a(get_sample_input()), 15);
    }

    #[test]
    fn a() {
        assert_eq!(solve_a(get_input()), 9651);
    }

    #[test]
    fn b_eg_1() {
        assert_eq!(solve_b(get_sample_input()), 12);
    }

    #[test]
    fn b() {
        assert_eq!(solve_b(get_input()), 10560);
    }

    //
}
