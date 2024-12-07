use std::{collections::HashSet, fmt::Display, fs::write, io};

pub fn get_input() -> String {
    String::from(include_str!("../input"))
}

pub fn get_sample_input() -> String {
    String::from(include_str!("../sample"))
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Dir {
    Up,
    Down,
    Left,
    Right,
}

impl Display for Dir {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Dir::Up => write!(f, "^"),
            Dir::Down => write!(f, "v"),
            Dir::Left => write!(f, "<"),
            Dir::Right => write!(f, ">"),
        }
    }
}

impl Dir {
    fn next_dir(&self) -> Dir {
        match self {
            Dir::Up => Dir::Right,
            Dir::Down => Dir::Left,
            Dir::Left => Dir::Up,
            Dir::Right => Dir::Down,
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
enum Cell {
    Empty,
    Thing,
}

type Guard = (Dir, usize, usize);

fn is_in_grid(grid: &Vec<Vec<Cell>>, col: usize, row: usize) -> bool {
    row < grid.len() && col < grid[row].len()
}

fn next_guard(grid: &Vec<Vec<Cell>>, guard: Guard) -> Guard {
    let maybe_next = match guard.0 {
        Dir::Up => (guard.0, guard.1, guard.2.wrapping_sub(1)),
        Dir::Down => (guard.0, guard.1, guard.2 + 1),
        Dir::Left => (guard.0, guard.1.wrapping_sub(1), guard.2),
        Dir::Right => (guard.0, guard.1 + 1, guard.2),
    };

    let (_, col, row) = maybe_next;

    match grid.get(row).and_then(|row| row.get(col)) {
        Some(Cell::Thing) => (guard.0.next_dir(), guard.1, guard.2),
        Some(Cell::Empty) | None => maybe_next,
    }
}

fn print_grid(grid: &Vec<Vec<Cell>>, seen: &HashSet<(usize, usize)>, guard: &Guard) {
    let mut str = String::new();

    for (y, row) in grid.iter().enumerate() {
        for (x, cell) in row.iter().enumerate() {
            if x == guard.1 && y == guard.2 {
                str.push_str(&format!("{}", guard.0));
                continue;
            }

            match cell {
                Cell::Thing => str.push('#'),
                Cell::Empty => {
                    if seen.contains(&(x, y)) {
                        str.push('x');
                    } else {
                        str.push('.');
                    }
                }
            }
        }

        str.push('\n')
    }

    write("../grid.ignored", str).unwrap();
}

fn confirm() -> () {
    let mut answer = String::new();

    io::stdin()
        .read_line(&mut answer)
        .ok()
        .expect("Failed to read line");
}

pub fn solve_a(input: String) -> i64 {
    let mut guard = (Dir::Up, 0, 0);

    let grid = input
        .lines()
        .enumerate()
        .map(|(y, line)| {
            line.char_indices()
                .filter_map(|(x, c)| match c {
                    '.' => Cell::Empty.into(),
                    '#' => Cell::Thing.into(),
                    '^' => {
                        guard = (Dir::Up, x, y);

                        None
                    }
                    _ => None,
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let mut seen = HashSet::new();

    let mut dr: i64 = -1;
    let mut dc: i64 = 0;

    loop {
        let (_, col, row) = guard;

        // print_grid(&grid, &seen, &guard);
        // confirm();

        println!("{} {}", col, row);

        if (row as i64 + dr) as usize >= grid.len() || (col as i64 + dc) as usize >= grid[row].len()
        {
            break;
        }

        seen.insert((col, row));

        if grid[(row as i64 + dr) as usize][(col as i64 + dc) as usize] == Cell::Thing {
            println!("pre-turn {} {}", dr, dc);
            (dc, dr) = (-dr, dc);
            println!("postturn {} {}", dr, dc);
        } else {
            guard.1 = (guard.1 as i64 + dc) as usize;
            guard.2 = (guard.2 as i64 + dr) as usize;
        }
    }

    // while is_in_grid(&grid, guard.1, guard.2) {
    //     let (_, col, row) = guard;

    //     seen.insert((col, row));

    //     // print_grid(&grid, &seen, &guard);
    //     // println!("{}", seen.len());
    //     // confirm();

    //     guard = next_guard(&grid, guard);
    // }

    seen.len() as i64
}

pub fn solve_b(input: String) -> i64 {
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[ignore]
    fn a_eg_1() {
        assert_eq!(solve_a(get_sample_input()), 41);
    }

    #[test]
    fn a() {
        assert_eq!(solve_a(get_input()), 1);
    }

    #[test]
    #[ignore]
    fn b_eg_1() {
        assert_eq!(solve_b(get_sample_input()), 1);
    }

    #[test]
    #[ignore]
    fn b() {
        assert_eq!(solve_b(get_input()), 1);
    }

    //
}
