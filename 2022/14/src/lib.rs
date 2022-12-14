use std::{collections::HashSet, ops::RangeInclusive};

pub fn get_input() -> String {
    String::from(include_str!("../input"))
}

pub fn get_sample_input() -> String {
    String::from(include_str!("../sample"))
}

pub fn print_paths_a(points: &HashSet<(usize, usize)>) {
    let x_min = points.iter().map(|point| point.0).min().unwrap() - 1;
    let x_max = points.iter().map(|point| point.0).max().unwrap() + 1;

    let y_min = points.iter().map(|point| point.1).min().unwrap().min(0);
    let y_max = points.iter().map(|point| point.1).max().unwrap() + 1;

    let pad_start_len = format!("{y_max:w$}  ", w = 2).len();
    for x in x_min..=x_max {
        print!(
            "{}{}",
            " ".repeat(pad_start_len),
            if x == 500 { "V" } else { " " }
        )
    }
    println!();

    for y in y_min..=y_max {
        print!("{y:w$}  ", w = 2);
        for x in x_min..=x_max {
            print!("{}", if points.contains(&(x, y)) { "#" } else { "." })
        }
        println!()
    }
}

pub fn print_paths_b(points: &HashSet<(usize, usize)>) {
    let x_min = points.iter().map(|point| point.0).min().unwrap() - 1;
    let x_max = points.iter().map(|point| point.0).max().unwrap() + 1;

    let y_min = points.iter().map(|point| point.1).min().unwrap().min(0);
    let y_max = points.iter().map(|point| point.1).max().unwrap() + 2;

    let pad_start_len = format!("{y_max:w$}  ", w = 2).len();
    print!("{}", " ".repeat(pad_start_len));

    for x in x_min..=x_max {
        print!("{}", if x == 500 { "V" } else { " " })
    }
    println!();

    for y in y_min..=y_max {
        print!("{y:w$}  ", w = 2);
        for x in x_min..=x_max {
            print!(
                "{}",
                if points.contains(&(x, y)) || y == y_max {
                    "#"
                } else {
                    "."
                }
            )
        }
        println!()
    }
}

pub fn solve_a(input: String) -> i64 {
    let paths: Vec<Vec<(usize, usize)>> = input
        .lines()
        .map(|line| {
            line.split(" -> ")
                .map(|point| {
                    let (x, y) = point.split_once(",").unwrap();

                    (x.parse().unwrap(), y.parse().unwrap())
                })
                .collect()
        })
        .collect();

    let mut rock_points = paths
        .iter()
        .flat_map(|path| {
            path.windows(2).flat_map(|window| {
                let from = window[0];
                let to = window[1];
                let x_are_same = from.0 == to.0;

                let range = if x_are_same {
                    RangeInclusive::new(from.1.min(to.1), from.1.max(to.1))
                } else {
                    RangeInclusive::new(from.0.min(to.0), from.0.max(to.0))
                };

                range.map(move |i| if x_are_same { (from.0, i) } else { (i, from.1) })
            })
        })
        .collect::<HashSet<_>>();

    let y_max = rock_points.iter().map(|point| point.1).max().unwrap() + 1;

    let mut num_sand = 1;

    'outer: loop {
        let mut x = 500;
        let mut y = 0;

        loop {
            // falling off the path
            if y >= y_max {
                break 'outer;
            }

            let next_y = y + 1;
            let next_x = if !rock_points.contains(&(x, next_y)) {
                x
            } else if !rock_points.contains(&(x - 1, next_y)) {
                x - 1
            } else if !rock_points.contains(&(x + 1, next_y)) {
                x + 1
            } else {
                break;
            };

            y = next_y;
            x = next_x;
        }

        rock_points.insert((x, y));

        num_sand += 1;
    }

    num_sand - 1
}

pub fn solve_b(input: String) -> i64 {
    let paths: Vec<Vec<(usize, usize)>> = input
        .lines()
        .map(|line| {
            line.split(" -> ")
                .map(|point| {
                    let (x, y) = point.split_once(",").unwrap();

                    (x.parse().unwrap(), y.parse().unwrap())
                })
                .collect()
        })
        .collect();

    let mut rock_points = paths
        .iter()
        .flat_map(|path| {
            path.windows(2).flat_map(|window| {
                let from = window[0];
                let to = window[1];
                let x_are_same = from.0 == to.0;

                let range = if x_are_same {
                    RangeInclusive::new(from.1.min(to.1), from.1.max(to.1))
                } else {
                    RangeInclusive::new(from.0.min(to.0), from.0.max(to.0))
                };

                range.map(move |i| if x_are_same { (from.0, i) } else { (i, from.1) })
            })
        })
        .collect::<HashSet<_>>();

    let y_max = rock_points.iter().map(|point| point.1).max().unwrap() + 2;

    let mut num_sand = 1;

    'outer: loop {
        let mut x = 500;
        let mut y = 0;

        loop {
            let next_y = y + 1;

            if next_y == y_max {
                break;
            }

            let next_x = if !rock_points.contains(&(x, next_y)) {
                x
            } else if !rock_points.contains(&(x - 1, next_y)) {
                x - 1
            } else if !rock_points.contains(&(x + 1, next_y)) {
                x + 1
            } else {
                break;
            };

            y = next_y;
            x = next_x;
        }

        if (x, y) == (500, 0) {
            break;
        }

        rock_points.insert((x, y));

        num_sand += 1;
    }

    num_sand
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[ignore]
    fn a_eg_1() {
        assert_eq!(solve_a(get_sample_input()), 24);
    }

    #[test]
    #[ignore]
    fn a() {
        assert_eq!(solve_a(get_input()), 825);
    }

    #[test]
    #[ignore]
    fn b_eg_1() {
        assert_eq!(solve_b(get_sample_input()), 93);
    }

    #[test]
    // #[ignore]
    fn b() {
        assert_eq!(solve_b(get_input()), 26729);
    }

    //
}
