use std::collections::{HashMap, HashSet, VecDeque};

pub fn get_input() -> String {
    String::from(include_str!("../input"))
}

pub fn get_sample_input() -> String {
    String::from(include_str!("../sample"))
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
enum Facing {
    North,
    East,
    South,
    West,
}

fn get_diff(facing: Facing) -> (i64, i64) {
    match facing {
        Facing::North => (0, -1),
        Facing::East => (1, 0),
        Facing::South => (0, 1),
        Facing::West => (-1, 0),
    }
}

impl Facing {
    fn adjacent(&self) -> Vec<Facing> {
        match self {
            Facing::North | Facing::South => vec![Facing::East, Facing::West],
            Facing::East | Facing::West => vec![Facing::North, Facing::South],
        }
    }
}

#[allow(unused)]
fn print_grid(grid: &Vec<Vec<char>>) {
    for row in grid {
        for c in row {
            print!("{}", c);
        }
        println!();
    }
}

#[allow(unused)]
fn print_grid_with_seats(grid: &Vec<Vec<char>>, seats: &HashSet<(usize, usize)>) {
    for row in grid {
        for (x, c) in row.iter().enumerate() {
            if seats.contains(&(x, row.len() - 1)) {
                print!("O");
            } else {
                print!("{}", c);
            }
        }
        println!();
    }
}

pub fn solve_a(input: String) -> i64 {
    let grid = input
        .lines()
        .map(|l| l.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();

    let start = grid
        .iter()
        .enumerate()
        .find_map(|(y, row)| {
            row.iter()
                .enumerate()
                .find_map(|(x, c)| if *c == 'S' { Some((x, y)) } else { None })
        })
        .unwrap();

    let end = grid
        .iter()
        .enumerate()
        .find_map(|(y, row)| {
            row.iter()
                .enumerate()
                .find_map(|(x, c)| if *c == 'E' { Some((x, y)) } else { None })
        })
        .unwrap();

    let mut seen = HashMap::<(usize, usize, Facing), i64>::new();
    let mut queue = VecDeque::new();

    queue.push_back((start, Facing::East, 0));

    while let Some((pos, facing, score)) = queue.pop_front() {
        if let Some(seen_score) = seen.get(&(pos.0, pos.1, facing)) {
            if *seen_score < score {
                continue;
            }
        }

        seen.insert((pos.0, pos.1, facing), score);

        if pos == end {
            continue;
        }

        let facing_diff = get_diff(facing);
        let forward_pos = (
            (pos.0 as i64 + facing_diff.0) as usize,
            (pos.1 as i64 + facing_diff.1) as usize,
        );

        if (0..grid.len()).contains(&forward_pos.0)
            && (0..grid[0].len()).contains(&forward_pos.1)
            && (grid[forward_pos.1][forward_pos.0] == '.'
                || grid[forward_pos.1][forward_pos.0] == 'E')
        {
            queue.push_back((forward_pos, facing, score + 1));
        }

        for next_facing in facing.adjacent() {
            queue.push_back((pos, next_facing, score + 1000));
        }
    }

    *seen
        .iter()
        .filter_map(|(&(x, y, _), score)| if (x, y) == end { Some(score) } else { None })
        .min()
        .unwrap()
}

pub fn solve_b(input: String) -> i64 {
    let grid = input
        .lines()
        .map(|l| l.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();

    let start = grid
        .iter()
        .enumerate()
        .find_map(|(y, row)| {
            row.iter()
                .enumerate()
                .find_map(|(x, c)| if *c == 'S' { Some((x, y)) } else { None })
        })
        .unwrap();

    let end = grid
        .iter()
        .enumerate()
        .find_map(|(y, row)| {
            row.iter()
                .enumerate()
                .find_map(|(x, c)| if *c == 'E' { Some((x, y)) } else { None })
        })
        .unwrap();

    let mut seen = HashMap::<(usize, usize, Facing), i64>::new();
    let mut queue = VecDeque::new();

    let mut paths = vec![];

    queue.push_back((start, Facing::East, 0, vec![(start.clone(), Facing::East)]));

    while let Some((pos, facing, score, path)) = queue.pop_front() {
        if let Some(seen_score) = seen.get(&(pos.0, pos.1, facing)) {
            if *seen_score < score {
                continue;
            }
        }

        seen.insert((pos.0, pos.1, facing), score);

        if pos == end {
            paths.push((path, score));
            continue;
        }

        let facing_diff = get_diff(facing);
        let forward_pos = (
            (pos.0 as i64 + facing_diff.0) as usize,
            (pos.1 as i64 + facing_diff.1) as usize,
        );

        if (0..grid.len()).contains(&forward_pos.0)
            && (0..grid[0].len()).contains(&forward_pos.1)
            && (grid[forward_pos.1][forward_pos.0] == '.'
                || grid[forward_pos.1][forward_pos.0] == 'E')
        {
            let mut path = path.clone();
            path.push((forward_pos, facing));
            queue.push_back((forward_pos, facing, score + 1, path));
        }

        for next_facing in facing.adjacent() {
            let mut path = path.clone();
            path.push((path.last().unwrap().0.clone(), next_facing));
            queue.push_back((pos, next_facing, score + 1000, path));
        }
    }

    let min = seen
        .iter()
        .filter_map(|(&(x, y, _), score)| if (x, y) == end { Some(score) } else { None })
        .min()
        .unwrap();

    let seats = paths
        .into_iter()
        .filter_map(|(path, score)| if score == *min { Some(path) } else { None })
        .flatten()
        .map(|(p, _)| p)
        .collect::<HashSet<_>>();

    seats.len() as i64
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_eg_1() {
        assert_eq!(solve_a(get_sample_input()), 7036);
    }

    #[test]
    fn a() {
        assert_eq!(solve_a(get_input()), 135512);
    }

    #[test]
    fn b_eg_1() {
        assert_eq!(solve_b(get_sample_input()), 45);
    }

    #[test]
    fn b() {
        assert_eq!(solve_b(get_input()), 541);
    }

    //
}
