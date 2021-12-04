use std::error::Error;
use std::fmt::Debug;
use std::fs;

pub fn get_input() -> Result<String, Box<dyn Error>> {
  let input = fs::read_to_string("input")?;

  Ok(input)
}

struct Cell {
  value: i32,
  seen: bool,
}

impl Clone for Cell {
  fn clone(&self) -> Self {
    Cell {
      value: self.value,
      seen: self.seen,
    }
  }
}
impl Copy for Cell {}

impl Debug for Cell {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(
      f,
      "{}{}",
      match self.seen {
        true => "*",
        _ => " ",
      },
      self.value
    )
  }
}

type Grid = Vec<Vec<Cell>>;

fn get_complete_row(grid: &Grid) -> Option<Vec<i32>> {
  for row in grid {
    if row.iter().all(|cell| cell.seen) {
      return Some(row.iter().map(|cell| cell.value).collect());
    }
  }

  for i in 0..GRID_SIZE {
    let col: Vec<_> = grid.iter().map(|row| row[i]).collect();

    if col.iter().all(|cell| cell.seen) {
      return Some(col.iter().map(|cell| cell.value).collect());
    }
  }

  None
}

fn get_complete_grid_from_grid_vec(grids: &Vec<Grid>) -> Option<&Grid> {
  for grid in grids {
    if let Some(_) = get_complete_row(grid) {
      return Some(grid);
    }
  }

  None
}

fn parse_inputs(input: &str) -> Vec<i32> {
  input
    .split(",")
    .map(|s| s.parse::<i32>().unwrap())
    .collect()
}

fn sum_unseen(grid: &Grid) -> i32 {
  grid
    .iter()
    .flat_map(|row| row.iter())
    .filter(|cell| !cell.seen)
    .map(|cell| cell.value)
    .sum()
}

const GRID_SIZE: usize = 5;

fn parse_grid(input: &str) -> Grid {
  let grid_nums = input
    .split_whitespace()
    .map(|num_str| num_str.parse::<i32>().unwrap())
    .collect::<Vec<_>>();

  let mut grid: Grid = vec![];

  for i in 0..GRID_SIZE {
    let mut row: Vec<_> = vec![];

    for j in 0..GRID_SIZE {
      row.push(Cell {
        value: grid_nums[i * GRID_SIZE + j],
        seen: false,
      });
    }

    grid.push(row);
  }

  grid
}

fn mark_seen(grid: &mut Grid, n: i32) {
  for row in grid {
    for cell in row {
      if cell.value == n {
        cell.seen = true;
      }
    }
  }
}

fn get_num_complete(grids: &Vec<Grid>) -> usize {
  grids
    .iter()
    .filter(|grid| get_complete_row(grid).is_some())
    .count()
}

pub fn solve_a(input: String) -> String {
  let mut line_groups = input.split("\n\n");

  let mut inputs = parse_inputs(line_groups.next().unwrap()).into_iter();
  let mut grids: Vec<_> = line_groups.map(|grid_in| parse_grid(grid_in)).collect();

  let mut curr_num = 0;
  let mut complete_grid = None;

  while complete_grid.is_none() {
    curr_num = inputs.next().unwrap();

    grids
      .iter_mut()
      .for_each(|grid| mark_seen(&mut *grid, curr_num));

    complete_grid = get_complete_grid_from_grid_vec(&grids);
  }

  let complete_grid = complete_grid.unwrap();

  let sum = sum_unseen(complete_grid);

  String::from(format!("{}", sum * curr_num))
}

pub fn solve_b(input: String) -> String {
  let mut line_groups = input.split("\n\n");

  let mut inputs = parse_inputs(line_groups.next().unwrap()).into_iter();
  let mut grids: Vec<_> = line_groups.map(|grid_in| parse_grid(grid_in)).collect();

  let mut curr_num = 0;

  while get_num_complete(&grids) + 1 < grids.len() {
    curr_num = inputs.next().unwrap();

    grids
      .iter_mut()
      .for_each(|grid| mark_seen(&mut *grid, curr_num));
  }

  let mut incomplete_grid = grids
    .iter_mut()
    .find(|grid| get_complete_row(grid).is_none())
    .unwrap();

  while get_complete_row(&incomplete_grid).is_none() {
    curr_num = inputs.next().unwrap();

    mark_seen(&mut incomplete_grid, curr_num)
  }

  let sum = sum_unseen(&incomplete_grid);

  String::from(format!("{}", sum * curr_num))
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn a_eg_1() {
    assert_eq!(
      solve_a(String::from(
        "7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1

22 13 17 11  0
8  2 23  4 24
21  9 14 16  7
6 10  3 18  5
1 12 20 15 19

3 15  0  2 22
9 18 13 17  5
19  8  7 25 23
20 11 10 24  4
14 21 16 12  6

14 21 17 24  4
10 16 15  9 19
18  8 23 26 20
22 11 13  6  5
2  0 12  3  7"
      )),
      "4512"
    );
  }

  #[test]
  fn b_eg_1() {
    assert_eq!(
      solve_b(String::from(
        "7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1

22 13 17 11  0
8  2 23  4 24
21  9 14 16  7
6 10  3 18  5
1 12 20 15 19

3 15  0  2 22
9 18 13 17  5
19  8  7 25 23
20 11 10 24  4
14 21 16 12  6

14 21 17 24  4
10 16 15  9 19
18  8 23 26 20
22 11 13  6  5
2  0 12  3  7"
      )),
      "1924"
    );
  }
}
