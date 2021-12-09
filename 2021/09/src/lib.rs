pub fn get_input() -> String {
  String::from(include_str!("../input"))
}

pub fn get_sample_input() -> String {
  String::from(include_str!("../sample"))
}

pub fn solve_a(input: String) -> i64 {
  let grid: Vec<Vec<_>> = input
    .split("\n")
    .map(|line| {
      line
        .chars()
        .map(|s| s.to_string().parse::<i64>().unwrap())
        .collect()
    })
    .collect();

  const DIFFS: [(i64, i64); 4] = [(0, 1), (1, 0), (0, -1), (-1, 0)];

  let mut low_points = vec![];

  for (i, row) in grid.iter().enumerate() {
    for (j, point) in row.iter().enumerate() {
      let mut points_to_compare = vec![];

      for &(di, dj) in DIFFS.iter() {
        match grid.get((i as i64 + di) as usize) {
          Some(row) => match row.get((j as i64 + dj) as usize) {
            Some(point) => points_to_compare.push(*point),
            None => continue,
          },
          None => continue,
        }
      }

      if points_to_compare.iter().all(|p| p > point) {
        low_points.push(point);
      }
    }
  }

  low_points.into_iter().map(|p| p + 1).sum()
}

const DIFFS: [(i64, i64); 4] = [(0, 1), (1, 0), (0, -1), (-1, 0)];

fn dfs(grid: &Vec<Vec<i64>>, i: usize, j: usize, seen: &mut Vec<Vec<bool>>) -> i64 {
  seen[i][j] = true;
  let mut output = 1;

  for (di, dj) in DIFFS {
    let i = i as i64 + di;
    let j = j as i64 + dj;

    if i >= 0 && i < grid.len() as i64 && j >= 0 && j < grid[0].len() as i64 {
      let i = i as usize;
      let j = j as usize;

      if !seen[i][j] && grid[i][j] != 9 {
        output += dfs(grid, i, j, seen);
      }
    }
  }

  output
}

pub fn solve_b(input: String) -> i64 {
  let grid: Vec<Vec<_>> = input
    .split("\n")
    .map(|line| {
      line
        .chars()
        .map(|s| s.to_string().parse::<i64>().unwrap())
        .collect()
    })
    .collect();

  let mut seen: Vec<Vec<bool>> = (0..grid.len())
    .map(|_| (0..grid[0].len()).map(|_| false).collect())
    .collect::<Vec<_>>();

  let mut island_sizes = vec![];

  for (i, row) in grid.iter().enumerate() {
    for (j, point) in row.iter().enumerate() {
      if !seen[i][j] && point != &9 {
        island_sizes.push(dfs(&grid, i, j, &mut seen));
      }
    }
  }

  island_sizes.sort();

  island_sizes
    .get(island_sizes.len() - 3..)
    .unwrap()
    .iter()
    .product()
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn a_eg_1() {
    assert_eq!(solve_a(get_sample_input()), 15);
  }

  #[test]
  fn a() {
    assert_eq!(solve_a(get_input()), 468);
  }

  #[test]
  fn b_eg_1() {
    assert_eq!(solve_b(get_sample_input()), 1134);
  }

  #[test]
  fn b() {
    assert_eq!(solve_b(get_input()), 1280496);
  }
}
