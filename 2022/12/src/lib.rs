use std::collections::{HashMap, VecDeque};

pub fn get_input() -> String {
    String::from(include_str!("../input"))
}

pub fn get_sample_input() -> String {
    String::from(include_str!("../sample"))
}

pub fn solve_a(input: String) -> i64 {
    let mut start: (i64, i64) = (0, 0);
    let mut end: (i64, i64) = (0, 0);
    let map: Vec<Vec<i64>> = input
        .lines()
        .enumerate()
        .map(|(row, line)| {
            line.bytes()
                .enumerate()
                .map(|(col, ch)| {
                    let real_ch;
                    if ch == b'S' {
                        start = (col as i64, row as i64);
                        real_ch = b'a'
                    } else if ch == b'E' {
                        end = (col as i64, row as i64);
                        real_ch = b'z'
                    } else {
                        real_ch = ch
                    }
                    (real_ch - b'a') as i64
                })
                .collect()
        })
        .collect();

    let n_rows = map.len() as i64;
    let n_cols = map[0].len() as i64;
    let mut queue = VecDeque::new();
    let mut min_path_len = usize::MAX;

    let mut seen_step = HashMap::<(i64, i64), usize>::new();

    queue.push_back((0, start));

    while let Some((steps, (px, py))) = queue.pop_front() {
        if (px, py) == end {
            if steps > min_path_len {
                continue;
            }

            min_path_len = steps;
            continue;
        }

        if steps >= min_path_len {
            continue;
        }

        let diffs = [(0, 1), (0, -1), (1, 0), (-1, 0)];
        let point_height = map[py as usize][px as usize];

        for (dx, dy) in diffs {
            let new_x = px + dx;
            let new_y = py + dy;
            let new_point = (new_x, new_y);

            if new_x < 0 || new_x >= n_cols || new_y < 0 || new_y >= n_rows {
                continue;
            }

            if seen_step
                .get(&new_point)
                .map(|n| &(steps + 1) >= n)
                .unwrap_or(false)
            {
                continue;
            }

            let new_height = map[new_y as usize][new_x as usize];

            if (new_height - point_height) <= 1 {
                queue.push_back((steps + 1, new_point.clone()));
                seen_step.insert(new_point, steps + 1);
            }
        }
    }

    min_path_len as i64
}

pub fn solve_b(input: String) -> i64 {
    let mut start: (i64, i64) = (0, 0);
    let mut end: (i64, i64) = (0, 0);
    let map: Vec<Vec<i64>> = input
        .lines()
        .enumerate()
        .map(|(row, line)| {
            line.bytes()
                .enumerate()
                .map(|(col, ch)| {
                    let real_ch;
                    if ch == b'S' {
                        start = (col as i64, row as i64);
                        real_ch = b'a'
                    } else if ch == b'E' {
                        end = (col as i64, row as i64);
                        real_ch = b'z'
                    } else {
                        real_ch = ch
                    }
                    (real_ch - b'a') as i64
                })
                .collect()
        })
        .collect();

    let n_rows = map.len() as i64;
    let n_cols = map[0].len() as i64;
    let mut queue = VecDeque::new();
    let mut min_path_len = usize::MAX;

    let mut seen_step = HashMap::<(i64, i64), usize>::new();

    let bottom_points: Vec<_> = map
        .iter()
        .enumerate()
        .flat_map(|(y, row)| {
            row.iter()
                .enumerate()
                .map(move |(x, cell)| (cell, (x as i64, y as i64)))
        })
        .filter_map(|(cell, point)| if *cell == 0 { Some((0, point)) } else { None })
        .collect();

    queue.extend(bottom_points);

    while let Some((steps, (px, py))) = queue.pop_front() {
        if (px, py) == end {
            if steps > min_path_len {
                continue;
            }

            min_path_len = steps;
            continue;
        }

        if steps >= min_path_len {
            continue;
        }

        let diffs = [(0, 1), (0, -1), (1, 0), (-1, 0)];
        let point_height = map[py as usize][px as usize];

        for (dx, dy) in diffs {
            let new_x = px + dx;
            let new_y = py + dy;
            let new_point = (new_x, new_y);

            if new_x < 0 || new_x >= n_cols || new_y < 0 || new_y >= n_rows {
                continue;
            }

            if seen_step
                .get(&new_point)
                .map(|n| &(steps + 1) >= n)
                .unwrap_or(false)
            {
                continue;
            }

            let new_height = map[new_y as usize][new_x as usize];

            if (new_height - point_height) <= 1 {
                queue.push_back((steps + 1, new_point.clone()));
                seen_step.insert(new_point, steps + 1);
            }
        }
    }

    min_path_len as i64
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[ignore]
    fn a_eg_1() {
        assert_eq!(solve_a(get_sample_input()), 31);
    }

    #[test]
    #[ignore]
    fn a() {
        assert_eq!(solve_a(get_input()), 481);
    }

    #[test]
    #[ignore]
    fn b_eg_1() {
        assert_eq!(solve_b(get_sample_input()), 29);
    }

    #[test]
    // #[ignore]
    fn b() {
        assert_eq!(solve_b(get_input()), 480);
    }

    //
}
