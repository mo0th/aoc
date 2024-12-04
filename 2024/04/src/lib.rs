use std::vec;

pub fn get_input() -> String {
    String::from(include_str!("../input"))
}

pub fn get_sample_input() -> String {
    String::from(include_str!("../sample"))
}

pub fn transpose(input: Vec<Vec<char>>) -> Vec<Vec<char>> {
    let mut output = vec![];
    for row in input {
        output.push(row);
    }
    output
}

trait ToUsize {
    fn to_usize(&self) -> Option<usize>;
}

impl ToUsize for i64 {
    fn to_usize(&self) -> Option<usize> {
        if self < &0 {
            None
        } else {
            Some(*self as usize)
        }
    }
}

fn get_options_a(i: usize, j: usize) -> Vec<Vec<(usize, usize)>> {
    let options = vec![
        vec![(0, 0), (1, 0), (2, 0), (3, 0)],
        vec![(0, 0), (-1, 0), (-2, 0), (-3, 0)],
        vec![(0, 0), (0, 1), (0, 2), (0, 3)],
        vec![(0, 0), (0, -1), (0, -2), (0, -3)],
        vec![(0, 0), (1, 1), (2, 2), (3, 3)],
        vec![(0, 0), (-1, 1), (-2, 2), (-3, 3)],
        vec![(0, 0), (1, -1), (2, -2), (3, -3)],
        vec![(0, 0), (-1, -1), (-2, -2), (-3, -3)],
    ];

    return options
        .iter()
        .map(|opt| {
            opt.iter()
                .filter_map(|(di, dj)| {
                    (i as i64 + di)
                        .to_usize()
                        .and_then(|i| (j as i64 + dj).to_usize().map(|j| (i, j)))
                })
                .collect()
        })
        .collect();
}

pub fn solve_a(input: String) -> i64 {
    let grid_as_rows = input
        .lines()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let mut count = 0;

    for (i, row) in grid_as_rows.iter().enumerate() {
        for (j, cell) in row.iter().enumerate() {
            if cell == &'X' {
                for opt_list in get_options_a(i, j) {
                    let joined: String = opt_list
                        .iter()
                        .filter_map(|(i, j)| {
                            return grid_as_rows.get(*i).and_then(|row| row.get(*j)).cloned();
                        })
                        .collect();

                    if joined == "XMAS" || joined == "SAMX" {
                        count += 1;
                    }
                }
            }
        }
    }

    return count;
}

fn get_options_b(i: usize, j: usize) -> Vec<Vec<(usize, usize)>> {
    let options = vec![
        vec![(-1, -1), (0, 0), (1, 1)],
        vec![(1, -1), (0, 0), (-1, 1)],
    ];

    return options
        .iter()
        .map(|opt| {
            opt.iter()
                .filter_map(|(di, dj)| {
                    (i as i64 + di)
                        .to_usize()
                        .and_then(|i| (j as i64 + dj).to_usize().map(|j| (i, j)))
                })
                .collect()
        })
        .collect();
}

pub fn solve_b(input: String) -> i64 {
    let grid_as_rows = input
        .lines()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    return grid_as_rows
        .iter()
        .enumerate()
        .map(|(i, row)| {
            row.iter()
                .enumerate()
                .filter(|(j, cell)| {
                    if cell != &&'A' {
                        return false;
                    }

                    let words = get_options_b(i, *j)
                        .iter()
                        .map(|opt| {
                            opt.iter()
                                .filter_map(|(i, j)| {
                                    return grid_as_rows.get(*i).and_then(|row| row.get(*j));
                                })
                                .collect::<String>()
                        })
                        .filter(|word| word == &"MAS" || word == &"SAM")
                        .count();

                    return words == 2;
                })
                .count() as i64
        })
        .sum();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_eg_1() {
        assert_eq!(solve_a(get_sample_input()), 18);
    }

    #[test]
    fn a() {
        assert_eq!(solve_a(get_input()), 2517);
    }

    #[test]
    fn b_eg_1() {
        assert_eq!(solve_b(get_sample_input()), 9);
    }

    #[test]
    fn b() {
        assert_eq!(solve_b(get_input()), 1960);
    }

    //
}
