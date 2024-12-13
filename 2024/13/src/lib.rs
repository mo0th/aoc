pub fn get_input() -> String {
    String::from(include_str!("../input"))
}

pub fn get_sample_input() -> String {
    String::from(include_str!("../sample"))
}

#[derive(Debug)]
struct Machine {
    a: (f64, f64),
    b: (f64, f64),
    prize: (f64, f64),
}

fn solve_machine(machine: Machine) -> Option<f64> {
    let Machine { a, b, prize } = machine;

    let b_times = (prize.1 * a.0 - prize.0 * a.1) / (b.1 * a.0 - a.1 * b.0);
    let a_times = (prize.0 - b.0 * b_times) / (a.0);

    if a_times.fract() != 0.0 || b_times.fract() != 0.0 {
        None
    } else {
        Some(3.0 * a_times + b_times)
    }
}

fn parse_machine(machine_str: &str) -> Machine {
    let parts = machine_str
        .lines()
        .map(|line| {
            let (_, line) = line.split_once(": ").unwrap();
            let nums = line
                .split(", ")
                .map(|coord| coord[2..].parse::<f64>().unwrap())
                .collect::<Vec<_>>();

            (nums[0], nums[1])
        })
        .collect::<Vec<_>>();

    return Machine {
        a: parts[0],
        b: parts[1],
        prize: parts[2],
    };
}

pub fn solve_a(input: String) -> i64 {
    input
        .split("\n\n")
        .map(parse_machine)
        .filter_map(solve_machine)
        .sum::<f64>() as i64
}

const PART_B_INC: f64 = 10000000000000.0;

pub fn solve_b(input: String) -> i64 {
    input
        .split("\n\n")
        .map(parse_machine)
        .map(|machine| Machine {
            prize: (machine.prize.0 + PART_B_INC, machine.prize.1 + PART_B_INC),
            ..machine
        })
        .filter_map(solve_machine)
        .sum::<f64>() as i64
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_eg_1() {
        assert_eq!(solve_a(get_sample_input()), 480);
    }

    #[test]
    fn a() {
        assert_eq!(solve_a(get_input()), 31897);
    }

    #[test]
    fn b_eg_1() {
        assert_eq!(solve_b(get_sample_input()), 875318608908);
    }

    #[test]
    fn b() {
        assert_eq!(solve_b(get_input()), 87596249540359);
    }

    //
}
