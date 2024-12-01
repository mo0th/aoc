use std::{
    fmt::{Debug, Display, Write},
    fs::{self, OpenOptions},
    io::Write as IOWrite,
    ops::RangeInclusive,
};

use itertools::Itertools;

pub fn get_input() -> String {
    String::from(include_str!("../input"))
}

pub fn get_sample_input() -> String {
    String::from(include_str!("../sample"))
}

#[derive(Debug, Clone, Copy)]
pub enum Cell {
    Digit(u8),
    Empty,
    Symbol(char),
}

impl From<char> for Cell {
    fn from(ch: char) -> Self {
        match ch {
            '.' => Self::Empty,
            ch if ch.is_digit(10) => Self::Digit(ch.to_digit(10).unwrap() as u8),
            ch => Self::Symbol(ch),
        }
    }
}

impl Display for Cell {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Cell::Digit(d) => f.write_str(&format!("{d}")),
            Cell::Empty => f.write_char('.'),
            &Cell::Symbol(s) => f.write_char(s),
        }
    }
}

pub fn print_grid(grid: &Vec<Vec<Cell>>) {
    for row in grid {
        for cell in row {
            print!("{cell}");
        }
        print!("\n");
    }
}

pub fn solve_a(input: String) -> i64 {
    let grid: Vec<_> = input
        .lines()
        .map(|line| line.chars().map(|ch| ch.into()).collect::<Vec<Cell>>())
        .collect();

    let part_numbers = grid
        .iter()
        .enumerate()
        .flat_map(|(y, row)| {
            let mut num: u64 = 0;
            let mut is_part_number = false;
            let mut nums = vec![];

            for (x, cell) in row.iter().enumerate() {
                if let &Cell::Digit(d) = cell {
                    num = num * 10 + (d as u64);

                    if is_part_number {
                        continue;
                    }

                    for dy in 0..=2 {
                        for dx in 0..=2 {
                            let yy = (y + dy).wrapping_sub(1);
                            let xx = (x + dx).wrapping_sub(1);

                            let c = grid.get(yy).and_then(|row| row.get(xx));

                            if let Some(Cell::Symbol(_)) = c {
                                is_part_number = true;
                                break;
                            }
                        }
                    }
                } else {
                    if is_part_number && num != 0 {
                        nums.push(num)
                    }

                    is_part_number = false;
                    num = 0;
                }
            }

            if is_part_number && num != 0 {
                nums.push(num)
            }

            return nums.into_iter();
        })
        .collect::<Vec<_>>();

    return part_numbers.iter().sum::<u64>() as i64;
}

pub fn solve_b(input: String) -> i64 {
    let grid: Vec<_> = input
        .lines()
        .map(|line| line.chars().map(|ch| ch.into()).collect::<Vec<Cell>>())
        .collect();

    let part_numbers = grid
        .iter()
        .enumerate()
        .flat_map(|(y, row)| {
            let mut num: u64 = 0;
            let mut start_x: usize = 0;
            let mut is_part_number = false;
            let mut nums = vec![];

            for (x, cell) in row.iter().enumerate() {
                if let &Cell::Digit(d) = cell {
                    if num == 0 {
                        start_x = x;
                    }

                    num = num * 10 + (d as u64);

                    if is_part_number {
                        continue;
                    }

                    for dy in 0..=2 {
                        for dx in 0..=2 {
                            let yy = (y + dy).wrapping_sub(1);
                            let xx = (x + dx).wrapping_sub(1);

                            let c = grid.get(yy).and_then(|row| row.get(xx));

                            if let Some(Cell::Symbol(s)) = c {
                                if s == &'*' || true {
                                    is_part_number = true;
                                    break;
                                }
                            }
                        }
                    }
                } else {
                    if is_part_number && num != 0 {
                        nums.push((start_x, x, y, num))
                    }

                    is_part_number = false;
                    num = 0;
                    start_x = 0;
                }
            }

            if is_part_number && num != 0 {
                nums.push((start_x, row.len(), y, num))
            }

            return nums.into_iter();
        })
        .into_group_map_by(|n| n.1);

    dbg!(&part_numbers);

    let gears = grid
        .iter()
        .enumerate()
        .flat_map(|(y, row)| return row.iter().enumerate().map(move |(x, cell)| (x, y, cell)))
        .filter_map(|(x, y, cell)| {
            return if matches!(cell, Cell::Symbol('*')) {
                Some((x, y))
            } else {
                None
            };
        })
        .collect::<Vec<_>>();

    let empty_vec = vec![];

    let gear_ratios: u64 = gears
        .iter()
        .filter_map(|&(gx, gy)| {
            let gys = [gy - 1, gy, gy + 1];
            let adjacent_parts = gys
                .iter()
                .flat_map(|y| {
                    return if let Some(v) = part_numbers.get(y) {
                        v
                    } else {
                        &empty_vec
                    };
                })
                .filter_map(|&(x1, x2, _, n)| {
                    let x1 = if x1 == 0 { 0 } else { x1 - 1 };
                    dbg!((x1, x2, n, gx, gy));
                    return dbg!(if gx >= x1 && gx <= x2 { Some(n) } else { None });
                })
                .collect::<Vec<_>>();

            dbg!((gx, gy, &adjacent_parts, gys));

            if adjacent_parts.len() != 2 {
                return None;
            }

            return Some(adjacent_parts.iter().product::<u64>());
        })
        .sum();

    return gear_ratios as i64;
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    #[ignore]
    fn a_eg_1() {
        assert_eq!(solve_a(get_sample_input()), 4361);
    }

    #[test]
    #[ignore]
    fn a_eg_2() {
        assert_eq!(
            solve_a(
                "
467..114.
...*.....
..35..633
......#..
617*.....
.....+.58
..592....
......755
...$.*...
.664.598.
"
                .trim()
                .to_string()
            ),
            4361
        );
    }

    #[test]
    #[ignore]
    fn a() {
        assert_eq!(solve_a(get_input()), 530849);
    }

    #[test]
    fn b_eg_1() {
        assert_eq!(solve_b(get_sample_input()), 467835);
    }

    #[test]
    #[ignore]
    fn b() {
        assert_eq!(solve_b(get_input()), 0);
    }

    //
}
