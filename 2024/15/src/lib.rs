use std::{collections::HashSet, vec};

pub fn get_input() -> String {
    String::from(include_str!("../input"))
}

pub fn get_sample_input() -> String {
    String::from(include_str!("../sample"))
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum CellA {
    Wall,
    Empty,
    Robot,
    Box,
}

fn get_a(grid: &Vec<Vec<CellA>>, x: i64, y: i64) -> Option<CellA> {
    Some(*grid.get(y as usize)?.get(x as usize)?)
}

fn set_a(grid: &mut Vec<Vec<CellA>>, x: i64, y: i64, cell: CellA) {
    grid[y as usize][x as usize] = cell;
}

#[allow(unused)]
fn print_grid_a(grid: &Vec<Vec<CellA>>) {
    for row in grid {
        for cell in row {
            print!(
                "{}",
                match cell {
                    CellA::Wall => '#',
                    CellA::Empty => '.',
                    CellA::Robot => '@',
                    CellA::Box => 'O',
                }
            );
        }
        println!();
    }
}

fn move_cells_a(mut grid: &mut Vec<Vec<CellA>>, dx: i64, dy: i64, to_move: &Vec<(i64, i64)>) {
    for pos in to_move.iter().rev() {
        let x = pos.0 + dx;
        let y = pos.1 + dy;

        let cell = get_a(&grid, pos.0, pos.1).unwrap();
        set_a(&mut grid, x, y, cell);
    }

    if to_move.len() > 0 {
        set_a(&mut grid, to_move[0].0, to_move[0].1, CellA::Empty);
    }
}

pub fn solve_a(input: String) -> i64 {
    let (grid, moves) = input.split_once("\n\n").unwrap();

    let mut grid = grid
        .lines()
        .map(|line| {
            line.chars()
                .map(|ch| match ch {
                    '#' => CellA::Wall,
                    '.' => CellA::Empty,
                    'O' => CellA::Box,
                    '@' => CellA::Robot,
                    _ => unreachable!(),
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    for m in moves.chars() {
        let (dx, dy) = match m {
            '>' => (1, 0),
            '<' => (-1, 0),
            '^' => (0, -1),
            'v' => (0, 1),
            _ => continue,
        };

        let robot_pos = grid
            .iter()
            .enumerate()
            .find_map(|(y, row)| {
                row.iter().enumerate().find_map(|(x, cell)| match cell {
                    CellA::Robot => Some((x as i64, y as i64)),
                    _ => None,
                })
            })
            .unwrap();

        let in_the_way = (1..)
            .map(|i| {
                let pos = (robot_pos.0 + dx * i, robot_pos.1 + dy * i);
                get_a(&grid, pos.0, pos.1).map(|cell| (pos, cell))
            })
            .take_while(|cell| cell.is_some_and(|(_, cell)| cell != CellA::Empty))
            .collect::<Option<Vec<_>>>()
            .unwrap();

        if in_the_way.iter().any(|(_, cell)| cell == &CellA::Wall) {
            continue;
        }

        let mut to_move = vec![robot_pos];
        to_move.extend(in_the_way.iter().map(|(pos, _)| *pos));

        move_cells_a(&mut grid, dx, dy, &to_move);
    }

    grid.iter()
        .enumerate()
        .flat_map(|(y, row)| {
            row.iter()
                .enumerate()
                .filter_map(move |(x, cell)| match cell {
                    CellA::Box => (y * 100 + x).into(),
                    _ => None,
                })
        })
        .sum::<usize>() as i64
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum CellB {
    Wall,
    Empty,
    BoxLeft,
    BoxRight,
    Robot,
}

#[allow(unused)]
fn print_grid_b(grid: &Vec<Vec<CellB>>) {
    for row in grid {
        for cell in row {
            print!(
                "{}",
                match cell {
                    CellB::Wall => '#',
                    CellB::Empty => '.',
                    CellB::BoxLeft => '[',
                    CellB::BoxRight => ']',
                    CellB::Robot => '@',
                }
            );
        }
        println!();
    }
}

fn get_b(grid: &Vec<Vec<CellB>>, x: i64, y: i64) -> Option<CellB> {
    Some(*grid.get(y as usize)?.get(x as usize)?)
}

fn set_b(grid: &mut Vec<Vec<CellB>>, x: i64, y: i64, cell: CellB) {
    grid[y as usize][x as usize] = cell;
}

fn move_cells_b(mut grid: &mut Vec<Vec<CellB>>, dx: i64, dy: i64, to_move: &Vec<(i64, i64)>) {
    let mut moved: HashSet<_> = HashSet::new();

    for pos in to_move.iter().rev() {
        let x = pos.0 + dx;
        let y = pos.1 + dy;

        let cell = get_b(&grid, pos.0, pos.1).unwrap();
        set_b(&mut grid, x, y, cell);
        moved.insert((x, y));
    }

    for pos in to_move {
        if moved.contains(&pos) {
            continue;
        }

        set_b(&mut grid, pos.0, pos.1, CellB::Empty);
    }
}

fn find_to_move_b(
    grid: &Vec<Vec<CellB>>,
    robot_pos: (i64, i64),
    dx: i64,
    dy: i64,
) -> Option<Vec<(i64, i64)>> {
    if dy == 0 {
        let in_the_way = (1..)
            .map(|i| {
                let pos = (robot_pos.0 + dx * i, robot_pos.1 + dy * i);
                get_b(&grid, pos.0, pos.1).map(|cell| (pos, cell))
            })
            .take_while(|cell| cell.is_some_and(|(_, cell)| cell != CellB::Empty))
            .collect::<Option<Vec<_>>>()
            .unwrap_or(vec![]);

        if in_the_way.iter().any(|(_, cell)| cell == &CellB::Wall) {
            return None;
        }

        let mut to_move = vec![robot_pos];
        to_move.extend(in_the_way.iter().map(|(pos, _)| *pos));
        Some(to_move)
    } else {
        let mut rows_of_cells = vec![vec![robot_pos]];

        loop {
            let mut next_row = vec![];

            for pos in rows_of_cells.last().unwrap() {
                let cell = get_b(&grid, pos.0, pos.1).unwrap();

                let next_x = pos.0 + dx;
                let next_y = pos.1 + dy;

                let next_cell = if let Some(next) = get_b(&grid, next_x, next_y) {
                    next
                } else {
                    continue;
                };

                match cell {
                    CellB::Robot => {
                        match next_cell {
                            CellB::BoxLeft => {
                                // box left
                                next_row.push((next_x, next_y));
                                // box right
                                next_row.push((next_x + 1, next_y));
                            }
                            CellB::BoxRight => {
                                // box right
                                next_row.push((next_x, next_y));
                                // box left
                                next_row.push((next_x - 1, next_y));
                            }
                            CellB::Wall => return None,
                            CellB::Empty => continue,
                            CellB::Robot => unreachable!(),
                        }
                    }
                    CellB::BoxLeft => {
                        match next_cell {
                            CellB::BoxRight => {
                                // box right
                                next_row.push((next_x, next_y));
                                // box left
                                next_row.push((next_x - 1, next_y));
                            }
                            CellB::BoxLeft => {
                                // box right
                                next_row.push((next_x, next_y));
                            }
                            CellB::Wall => return None,
                            CellB::Empty => continue,
                            CellB::Robot => unreachable!(),
                        }
                    }
                    CellB::BoxRight => match next_cell {
                        CellB::BoxLeft => {
                            // box left
                            next_row.push((next_x, next_y));
                            // box right
                            next_row.push((next_x + 1, next_y));
                        }
                        CellB::BoxRight => {
                            // box right
                            next_row.push((next_x, next_y));
                        }
                        CellB::Wall => return None,
                        CellB::Empty => continue,
                        CellB::Robot => unreachable!(),
                    },
                    CellB::Wall => unreachable!(),
                    CellB::Empty => unreachable!(),
                }
            }

            if next_row.len() == 0 {
                break;
            }

            rows_of_cells.push(next_row);
        }

        Some(rows_of_cells.into_iter().flatten().collect())
    }
}

pub fn solve_b(input: String) -> i64 {
    let (grid, moves) = input.split_once("\n\n").unwrap();

    let mut grid = grid
        .lines()
        .map(|line| {
            line.chars()
                .flat_map(|ch| match ch {
                    '#' => vec![CellB::Wall, CellB::Wall],
                    '.' => vec![CellB::Empty, CellB::Empty],
                    'O' => vec![CellB::BoxLeft, CellB::BoxRight],
                    '@' => vec![CellB::Robot, CellB::Empty],
                    _ => unreachable!(),
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    for m in moves.chars() {
        let (dx, dy) = match m {
            '>' => (1, 0),
            '<' => (-1, 0),
            '^' => (0, -1),
            'v' => (0, 1),
            _ => continue,
        };

        let robot_pos = grid
            .iter()
            .enumerate()
            .find_map(|(y, row)| {
                row.iter().enumerate().find_map(|(x, cell)| match cell {
                    CellB::Robot => Some((x as i64, y as i64)),
                    _ => None,
                })
            })
            .unwrap();

        if let Some(to_move) = find_to_move_b(&grid, robot_pos, dx, dy) {
            move_cells_b(&mut grid, dx, dy, &to_move);
        } else {
            continue;
        }
    }

    grid.iter()
        .enumerate()
        .flat_map(|(y, row)| {
            row.iter()
                .enumerate()
                .filter_map(move |(x, cell)| match cell {
                    CellB::BoxLeft => (y * 100 + x).into(),
                    _ => None,
                })
        })
        .sum::<usize>() as i64
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_eg_1() {
        assert_eq!(solve_a(get_sample_input()), 2028);
    }

    #[test]
    fn a() {
        assert_eq!(solve_a(get_input()), 1526673);
    }

    #[test]
    fn b_eg_1() {
        assert_eq!(
            solve_b(
                "
##########
#..O..O.O#
#......O.#
#.OO..O.O#
#..O@..O.#
#O#..O...#
#O..O..O.#
#.OO.O.OO#
#....O...#
##########

<vv>^<v^>v>^vv^v>v<>v^v<v<^vv<<<^><<><>>v<vvv<>^v^>^<<<><<v<<<v^vv^v>^
vvv<<^>^v^^><<>>><>^<<><^vv^^<>vvv<>><^^v>^>vv<>v<<<<v<^v>^<^^>>>^<v<v
><>vv>v^v^<>><>>>><^^>vv>v<^^^>>v^v^<^^>v^^>v^<^v>v<>>v^v^<v>v^^<^^vv<
<<v<^>>^^^^>>>v^<>vvv^><v<<<>^^^vv^<vvv>^>v<^^^^v<>^>vvvv><>>v^<<^^^^^
^><^><>>><>^^<<^^v>>><^<v>^<vv>>v>>>^v><>^v><<<<v>>v<v<v>vvv>^<><<>^><
^>><>^v<><^vvv<^^<><v<<<<<><^v<<<><<<^^<v<^^^><^>>^<v^><<<^>>^v<v^v<v^
>^>>^v>vv>^<<^v<>><<><<v<<v><>v<^vv<<<>^^v^>^^>>><<^v>>v^v><^^>>^<>vv^
<><^^>^^^<><vvvvv^v<v<<>^v<v>v<<^><<><<><<<^^<<<^<<>><<><^^^>^^<>^>v<>
^^>vv<^v^v<vv>^<><v<^v>^^^>>>^^vvv^>vvv<>>>^<^>>>>>^<<^v>^vvv<>^<><<v>
v^^>>><<^^<>>^v^<v^vv<>v^<<>^<^v^v><^<<<><<^<v><v<>vv>>v><v^<vv<>v^<<^
"
                .trim()
                .to_string()
            ),
            9021
        );
    }

    #[test]
    fn b() {
        assert_eq!(solve_b(get_input()), 1535509);
    }

    //
}
