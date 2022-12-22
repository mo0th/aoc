pub fn get_input() -> String {
    String::from(include_str!("../input"))
}

pub fn get_sample_input() -> String {
    String::from(include_str!("../sample"))
}

#[derive(Debug, Clone, Copy)]
enum Cell {
    Wall,
    Open,
    Nothing,
}

#[derive(Debug, Clone, Copy)]
enum Facing {
    Top,
    Right,
    Bottom,
    Left,
}

impl Facing {
    fn get_value(&self) -> i64 {
        match self {
            Facing::Top => 3,
            Facing::Right => 0,
            Facing::Bottom => 1,
            Facing::Left => 2,
        }
    }

    fn get_diff(&self) -> (i64, i64) {
        match self {
            Facing::Top => (-1, 0),
            Facing::Right => (0, 1),
            Facing::Bottom => (1, 0),
            Facing::Left => (0, -1),
        }
    }

    fn rotate(&self, m: &Move) -> Self {
        match m {
            Move::Left => match self {
                Facing::Top => Facing::Left,
                Facing::Right => Facing::Top,
                Facing::Bottom => Facing::Right,
                Facing::Left => Facing::Bottom,
            },
            Move::Right => match self {
                Facing::Top => Facing::Right,
                Facing::Right => Facing::Bottom,
                Facing::Bottom => Facing::Left,
                Facing::Left => Facing::Top,
            },
            Move::Forward(_) => panic!(),
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum Move {
    Left,
    Right,
    Forward(i64),
}

pub fn solve_a(input: String) -> i64 {
    let (board, instructions) = input.split_once("\n\n").unwrap();

    let w = board.lines().map(|l| l.len()).max().unwrap() as i64;
    let h = board.lines().count() as i64;

    let board = board
        .lines()
        .map(|line| {
            line.chars()
                .map(move |c| match c {
                    '.' => Cell::Open,
                    '#' => Cell::Wall,
                    _ => Cell::Nothing,
                })
                .chain((0..w).map(|_| Cell::Nothing))
                .take(w as usize)
                .collect()
        })
        .collect::<Vec<Vec<_>>>();

    let instructions = {
        let numbers = instructions
            .split("L")
            .flat_map(|chunk| chunk.split("R"))
            .map(|s| Move::Forward(s.parse::<i64>().unwrap()))
            .collect::<Vec<_>>();
        let dirs = instructions
            .chars()
            .filter_map(|c| match c {
                'L' => Some(Move::Left),
                'R' => Some(Move::Right),
                _ => None,
            })
            .collect::<Vec<_>>();

        let mut result = vec![];

        for (i, n) in numbers.into_iter().enumerate() {
            result.push(n);
            if let Some(dir) = dirs.get(i) {
                result.push(*dir);
            }
        }

        result
    };

    let mut position = (
        0,
        board[0]
            .iter()
            .enumerate()
            .find(|(_i, c)| !matches!(**c, Cell::Nothing))
            .unwrap()
            .0 as i64,
    );
    let mut facing = Facing::Right;

    for instruction in instructions {
        let num_moves = if let Move::Forward(n) = instruction {
            n
        } else {
            facing = facing.rotate(&instruction);

            continue;
        };

        let (drow, dcol) = facing.get_diff();

        for _ in 0..num_moves {
            let mut row = (((position.0 + drow) + h) % h) as usize;
            let mut col = (((position.1 + dcol) + w) % w) as usize;

            while matches!(board[row][col], Cell::Nothing) {
                row = (((row as i64 + drow) + h) % h) as usize;
                col = (((col as i64 + dcol) + w) % w) as usize;
                println!("{:?}", (row, col));
            }

            let cell = board[row][col];

            match cell {
                Cell::Nothing => panic!("Should not reach here"),
                Cell::Open => {
                    position = (row as i64, col as i64);
                }
                Cell::Wall => {
                    break;
                }
            }
        }
    }

    (1000 * (position.0 + 1)) + (4 * (position.1 + 1)) + facing.get_value()
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
        assert_eq!(solve_a(get_sample_input()), 6032);
    }

    #[test]
    // #[ignore]
    fn a() {
        assert_eq!(solve_a(get_input()), 31568);
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
