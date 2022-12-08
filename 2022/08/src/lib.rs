pub fn get_input() -> String {
    String::from(include_str!("../input"))
}

pub fn get_sample_input() -> String {
    String::from(include_str!("../sample"))
}

fn print_grid(grid: &Vec<Vec<i64>>) {
    grid.iter().for_each(|row| {
        row.iter().for_each(|&b| print!("{b}"));
        print!("\n");
    })
}
fn print_visible(visible: &Vec<Vec<bool>>) {
    visible.iter().for_each(|row| {
        row.iter()
            .for_each(|&b| print!("{}", if b { "X" } else { " " }));
        print!("\n");
    })
}

pub fn solve_a(input: String) -> i64 {
    let grid = input
        .lines()
        .map(|line| {
            line.chars()
                .map(|ch| ch.to_string().parse::<i64>().unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();
    let mut visible = grid
        .iter()
        .map(|row| row.iter().map(|_| false).collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let n_rows = grid.len();
    let n_cols = grid[0].len();

    for i in 0..n_rows {
        visible[i][0] = true;
        visible[i][n_cols - 1] = true;
    }
    for i in 0..n_cols {
        visible[0][i] = true;
        visible[n_rows - 1][i] = true;
    }

    for (i, row) in grid
        .iter()
        .enumerate()
        .filter(|(i, _)| i > &0 && i < &(n_rows - 1))
    {
        let mut max_so_far = row[0];

        for (j, &curr) in row
            .iter()
            .enumerate()
            .filter(|(j, _)| j > &0 && j < &(n_cols - 1))
        {
            if curr > max_so_far {
                visible[i][j] = true;
                max_so_far = curr;
            }
        }

        let mut max_so_far = row[row.len() - 1];

        for (j, &curr) in row
            .iter()
            .enumerate()
            .rev()
            .filter(|(j, _)| j > &0 && j < &(n_cols - 1))
        {
            if curr > max_so_far {
                visible[i][j] = true;
                max_so_far = curr;
            }
        }
    }

    for j in 1..(n_cols - 1) {
        let mut max_so_far = grid[0][j];

        for i in 1..(n_rows - 1) {
            let curr = grid[i][j];
            if curr > max_so_far {
                visible[i][j] = true;
                max_so_far = curr;
            }
        }

        let mut max_so_far = grid[n_rows - 1][j];

        for i in (1..(n_rows - 1)).rev() {
            let curr = grid[i][j];
            if curr > max_so_far {
                visible[i][j] = true;
                max_so_far = curr;
            }
        }
    }

    visible
        .iter()
        .map(|row| row.iter().filter(|&&b| b).count() as i64)
        .sum::<i64>()
}

pub fn solve_b(input: String) -> i64 {
    let grid = input
        .lines()
        .map(|line| {
            line.chars()
                .map(|ch| ch.to_string().parse::<i64>().unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let n_rows = grid.len();
    let n_cols = grid[0].len();

    let mut max_score = 1usize;

    for i in 1..(n_rows - 1) {
        for j in 1..(n_cols - 1) {
            let curr_height = grid[i][j];

            let right = grid[i]
                .iter()
                .skip(j + 1)
                .take_while(|&&t| t < curr_height)
                .count();
            let right = if right < (n_cols - (j + 1)) {
                right + 1
            } else {
                right
            };

            let left = grid[i]
                .iter()
                .take(j)
                .rev()
                .take_while(|&&t| t < curr_height)
                .count();
            let left = if left < j { left + 1 } else { left };

            let top = grid
                .iter()
                .map(|row| row[j])
                .take(i)
                .rev()
                .take_while(|&t| t < curr_height)
                .count();

            let top = if top < i { top + 1 } else { top };

            let bottom = grid
                .iter()
                .map(|row| row[j])
                .skip(i + 1)
                .take_while(|&t| t < curr_height)
                .count();
            let bottom = if bottom < (n_rows - (i + 1)) {
                bottom + 1
            } else {
                bottom
            };

            let score = left * right * top * bottom;

            if score > max_score {
                max_score = score
            }
        }
    }

    max_score as i64
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_eg_1() {
        assert_eq!(solve_a(get_sample_input()), 21);
    }

    #[test]
    fn a() {
        assert_eq!(solve_a(get_input()), 1832);
    }

    #[test]
    fn b_eg_1() {
        assert_eq!(solve_b(get_sample_input()), 8);
    }

    #[test]
    fn b() {
        assert_eq!(solve_b(get_input()), 157320);
    }

    //
}
