pub fn get_input() -> String {
    String::from(include_str!("../input"))
}

pub fn get_sample_input() -> String {
    String::from(include_str!("../sample"))
}

pub fn solve_a(input: String) -> i64 {
    let mut numbers = input
        .lines()
        .map(|l| l.parse::<i64>().unwrap())
        .collect::<Vec<_>>();
    let original = numbers.clone();
    let len = numbers.len() as i64;

    for n in original {
        println!("before {n} {numbers:?}");
        let mut diff = if n >= 0 {
            n % len
        } else {
            let mut n = n;
            while n < 0 {
                n += len;
            }
            n
        };

        let start_idx = numbers
            .iter()
            .enumerate()
            .find_map(|(i, num)| if num == &n { Some(i) } else { None })
            .unwrap() as i64;
        println!("found {n} at {start_idx}");

        for d in 0..diff.abs() {
            let idx = {
                let mut idx = start_idx + d;
                while idx < 0 {
                    idx += len;
                }
                idx % len
            };
            let next_idx = (idx + 1) % len;
            println!("swap {idx} ({start_idx} + {d}) with {next_idx}");
            // println!("{:?}", (start_idx, d, min, max, idx, next_idx));
            numbers.swap(idx as usize, next_idx as usize);
        }

        println!("after  {n} {numbers:?}\n{}\n", "=".repeat(10));
    }

    let zero_idx = numbers
        .iter()
        .enumerate()
        .find_map(|(i, num)| if num == &0 { Some(i) } else { None })
        .unwrap();

    let coords = [1000, 2000, 3000]
        .iter()
        .map(|n| (n + zero_idx) % (len as usize))
        .map(|i| numbers[i])
        .collect::<Vec<_>>();

    println!("{coords:?}");

    coords.iter().sum::<i64>()
}

pub fn solve_b(input: String) -> i64 {
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    // #[ignore]
    fn a_eg_1() {
        assert_eq!(solve_a(get_sample_input()), 3);
    }

    #[test]
    #[ignore]
    fn a() {
        assert_eq!(solve_a(get_input()), 0);
    }

    #[test]
    #[ignore]
    fn b_eg_1() {
        assert_eq!(solve_b(get_sample_input()), 0);
    }

    #[test]
    #[ignore]
    fn b() {
        assert_eq!(solve_b(get_input()), 0);
    }

    //
}
