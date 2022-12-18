use std::collections::HashSet;

pub fn get_input() -> String {
    String::from(include_str!("../input"))
}

pub fn get_sample_input() -> String {
    String::from(include_str!("../sample"))
}

type Point = (i64, i64);

#[derive(Debug)]
struct Rock {
    /// Positions relative to bottom left
    /// ####
    ///
    /// .#.
    /// ###
    /// .#.
    ///
    /// ..#
    /// ..#
    /// ###
    ///
    /// #
    /// #
    /// #
    /// #
    ///
    /// ##
    /// ##
    points: Vec<Point>,

    width: i64,
    min_x: i64,
    max_x: i64,
}

impl Rock {
    fn new(points: Vec<Point>) -> Self {
        let min_x = points.iter().map(|p| p.0).min().unwrap();
        let max_x = points.iter().map(|p| p.0).max().unwrap();
        Rock {
            points,
            width: ((min_x - max_x).abs() + 1),
            min_x,
            max_x,
        }
    }

    fn can_move(&self, x: i64, y: i64, filled: &HashSet<Point>) -> bool {
        if self.min_x + x < 0 || self.max_x + x >= 7 || y < 0 {
            return false;
        }

        self.points
            .iter()
            .map(|p| (p.0 + x, p.1 + y))
            .all(|p| !filled.contains(&p))
    }

    fn fill(&self, x: i64, y: i64, filled: &mut HashSet<Point>) {
        filled.extend(self.points.iter().map(|p| (p.0 + x, p.1 + y)))
    }
}

fn get_rocks() -> Vec<Rock> {
    vec![
        Rock::new(vec![(0, 0), (1, 0), (2, 0), (3, 0)]),
        Rock::new(vec![(1, 0), (0, 1), (1, 1), (2, 1), (1, 2)]),
        Rock::new(vec![(0, 0), (1, 0), (2, 0), (2, 1), (2, 2)]),
        Rock::new(vec![(0, 0), (0, 1), (0, 2), (0, 3)]),
        Rock::new(vec![(0, 0), (1, 0), (0, 1), (1, 1)]),
    ]
}

fn print_filled(filled: &HashSet<Point>) {
    let max_y = filled.iter().map(|p| p.1).max().unwrap_or(0);

    println!("|.......|");
    for y in (0..=max_y).rev() {
        print!("|");
        for x in 0..7 {
            print!("{}", if filled.contains(&(x, y)) { "#" } else { "." })
        }
        println!("|")
    }
    println!("+-------+");
}

#[derive(Debug)]
enum Dir {
    Left,
    Right,
}

pub fn solve_a(input: String) -> i64 {
    let rocks = get_rocks();
    let mut rock_it = rocks.iter().cycle();
    let mut jets = input
        .chars()
        .map(|c| match c {
            '<' => Dir::Left,
            '>' => Dir::Right,
            _ => panic!(),
        })
        .cycle();

    let mut filled = HashSet::<Point>::new();

    for _ in 0..2022 {
        let rock = rock_it.next().unwrap();
        let mut x = 2i64;
        let mut y = filled.iter().map(|p| p.1).max().unwrap_or(-1) + 4;
        // println!("{filled:?}");
        loop {
            let dx = match jets.next().unwrap() {
                Dir::Left => -1,
                Dir::Right => 1,
            };
            let next_x = x + dx;
            let next_y = y - 1;

            // print!("next: {:?}", (next_x, next_y));

            if rock.can_move(next_x, y, &filled) {
                x = next_x;
                // print!("; took x")
            }

            if !rock.can_move(x, next_y, &filled) {
                // println!("; ending on {:?}", (x, y));
                rock.fill(x, y, &mut filled);
                break;
            }

            y = next_y;
            // println!("; took y");
        }
        // print_filled(&filled);
        // std::thread::sleep(std::time::Duration::from_millis(1000))
    }

    filled.iter().map(|p| p.1).max().unwrap_or(0) + 1
}

pub fn solve_b(input: String) -> i64 {
    let rocks = get_rocks();
    let mut rock_it = rocks.iter().enumerate().cycle();
    let mut jets = input
        .chars()
        .map(|c| match c {
            '<' => Dir::Left,
            '>' => Dir::Right,
            _ => panic!(),
        })
        .enumerate()
        .cycle();

    let mut filled = HashSet::<Point>::new();

    let n_sim = 1_000_000_000_000u64;
    // let n_sim = 2022;

    for sim in 0..n_sim {
        if sim % 10_000 == 0 && sim > 0 {
            let curr_max_y = filled.iter().map(|p| p.1).max().unwrap_or(0);
            println!("sim {sim} - {curr_max_y}");
            let min_y_for_subset = curr_max_y - 20;
            filled = filled
                .into_iter()
                .filter(|p| p.1 >= min_y_for_subset)
                .collect();
        }
        let (i, rock) = rock_it.next().unwrap();
        let mut x = 2i64;
        let mut y = filled.iter().map(|p| p.1).max().unwrap_or(-1) + 4;
        // println!("{filled:?}");
        loop {
            let (j, jet) = jets.next().unwrap();
            let dx = match jet {
                Dir::Left => -1,
                Dir::Right => 1,
            };
            let next_x = x + dx;
            let next_y = y - 1;

            // if i == 0 || j == 0 {
            //     println!(
            //         "curr max: {:?} {}",
            //         (i, j),
            //         filled.iter().map(|p| p.1).max().unwrap_or(0) + 1
            //     )
            // }

            // print!("next: {:?}", (next_x, next_y));

            if rock.can_move(next_x, y, &filled) {
                x = next_x;
                // print!("; took x")
            }

            if !rock.can_move(x, next_y, &filled) {
                // println!("; ending on {:?}", (x, y));
                rock.fill(x, y, &mut filled);
                break;
            }

            y = next_y;
            // println!("; took y");
        }
        // print_filled(&filled);
        // std::thread::sleep(std::time::Duration::from_millis(1000))
    }

    filled.iter().map(|p| p.1).max().unwrap_or(0) + 1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[ignore]
    fn a_eg_1() {
        assert_eq!(solve_a(get_sample_input()), 3068);
    }

    #[test]
    #[ignore]
    fn a() {
        assert_eq!(solve_a(get_input()), 3163);
    }

    #[test]
    // #[ignore]
    fn b_eg_1() {
        assert_eq!(solve_b(get_sample_input()), 1514285714288);
    }

    #[test]
    #[ignore]
    fn b() {
        assert_eq!(solve_b(get_input()), 0);
    }

    //
}
