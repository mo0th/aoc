pub fn get_input() -> String {
    String::from(include_str!("../input"))
}

pub fn get_sample_input() -> String {
    String::from(include_str!("../sample"))
}

#[derive(Debug)]
enum Instruction {
    NOOP,
    ADDX(i64),
}

pub fn solve_a(input: String) -> i64 {
    let program = input
        .lines()
        .map(|line| {
            if line == "noop" {
                Instruction::NOOP
            } else {
                line.split_once(" ")
                    .and_then(|(_, n)| n.parse::<i64>().ok())
                    .map(|n| Instruction::ADDX(n))
                    .unwrap()
            }
        })
        .collect::<Vec<_>>();

    let mut signal_strengths: Vec<(i64, i64)> = vec![];
    let mut cycle = 1;
    let mut important_cycles = (0..6i64).map(|n| (n * 40) + 20);
    let mut next_special_cycle = important_cycles.next();

    let mut x = 1;

    for instruction in program.iter().cycle() {
        if next_special_cycle.is_none() {
            break;
        }

        let (cycle_inc, x_inc) = match instruction {
            Instruction::NOOP => (1, 0),
            Instruction::ADDX(n) => (2, *n),
        };

        for i in 1..=cycle_inc {
            cycle += 1;
            if i == cycle_inc {
                x += x_inc;
            }
            match next_special_cycle {
                Some(target) if target == cycle => {
                    signal_strengths.push((cycle, x));
                    next_special_cycle = important_cycles.next();
                }
                Some(_) => (),
                None => {
                    break;
                }
            }
        }
    }

    signal_strengths.iter().map(|(a, b)| a * b).sum()
}

pub fn solve_b(input: String) -> String {
    let program = input
        .lines()
        .map(|line| {
            if line == "noop" {
                Instruction::NOOP
            } else {
                line.split_once(" ")
                    .and_then(|(_, n)| n.parse::<i64>().ok())
                    .map(|n| Instruction::ADDX(n))
                    .unwrap()
            }
        })
        .collect::<Vec<_>>();

    let mut signal_strengths: Vec<(i64, i64)> = vec![];
    let mut cycle = 1i64;
    let mut x = 1i64;
    let mut crt: Vec<Vec<bool>> = (0..6).map(|_| (0..40).map(|_| false).collect()).collect();

    crt[0][0] = true;

    for instruction in program.iter().cycle() {
        if cycle > 240 {
            break;
        }

        let (cycle_inc, x_inc) = match instruction {
            Instruction::NOOP => (1, 0),
            Instruction::ADDX(n) => (2, *n),
        };

        for i in 1..=cycle_inc {
            cycle += 1;

            if cycle > 240 {
                break;
            }

            let row = (cycle - 1) / 40i64;
            let col = (cycle - 1) % 40;

            if i == cycle_inc {
                x += x_inc;
            }

            let min = (x - 1) as i64;
            let max = (x + 1) as i64;
            crt[row as usize][col as usize] = col >= min && col <= max;
        }
    }

    let result: String = crt
        .iter()
        .map(|row| {
            row.iter()
                .map(|c| if *c { '#' } else { '.' })
                .collect::<String>()
                + "\n"
        })
        .collect::<String>()
        .trim()
        .to_string();

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_eg_1() {
        assert_eq!(solve_a(get_sample_input()), 13140);
    }

    #[test]
    fn a() {
        assert_eq!(solve_a(get_input()), 12540);
    }

    #[test]
    fn b_eg_1() {
        assert_eq!(
            solve_b(get_sample_input()),
            "
##..##..##..##..##..##..##..##..##..##..
###...###...###...###...###...###...###.
####....####....####....####....####....
#####.....#####.....#####.....#####.....
######......######......######......####
#######.......#######.......#######.....
"
            .trim()
            .to_string()
        );
    }

    #[test]
    fn b() {
        assert_eq!(
            solve_b(get_input()),
            "
####.####..##..####.####.#....#..#.####.
#....#....#..#....#.#....#....#..#.#....
###..###..#......#..###..#....####.###..
#....#....#.....#...#....#....#..#.#....
#....#....#..#.#....#....#....#..#.#....
#....####..##..####.####.####.#..#.####.
"
            .trim()
            .to_string()
        );
    }

    //
}
