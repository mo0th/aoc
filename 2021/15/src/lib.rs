pub fn get_input() -> String {
  String::from(include_str!("../input"))
}

pub fn get_sample_input() -> String {
  String::from(include_str!("../sample"))
}

pub fn solve_a(input: String) -> i64 {
  let grid = input
    .lines()
    .map(|line| {
      line
        .chars()
        .map(|c| c.to_string().parse::<i64>().unwrap())
        .collect::<Vec<_>>()
    })
    .collect::<Vec<_>>();

  let n_rows = grid.len();
  let n_cols = grid[0].len();
  let n_cells = n_rows * n_cols;

  let mut spt_set = grid
    .iter()
    .map(|row| row.iter().map(|_| false).collect::<Vec<_>>())
    .collect::<Vec<_>>();

  let mut dist = grid
    .iter()
    .map(|row| row.iter().map(|_| i64::MAX).collect::<Vec<_>>())
    .collect::<Vec<_>>();

  dist[0][0] = 0;

  for _ in 0..n_cells {
    let mut min_dist = i64::MAX;
    let mut min_dist_point = (0, 0);

    for (i, row) in dist.iter().enumerate() {
      for (j, &d) in row.iter().enumerate() {
        if d < min_dist && !spt_set[i][j] {
          min_dist = d;
          min_dist_point = (i, j);
        }
      }
    }

    let (r, c) = min_dist_point;

    spt_set[r][c] = true;

    const DIFFS: [(i64, i64); 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];

    for (di, dj) in DIFFS {
      let i = (r as i64 + di) as usize;
      let j = (c as i64 + dj) as usize;

      if i < n_rows && j < n_cols && !spt_set[i][j] && dist[i][j] > dist[r][c] + grid[i][j] {
        dist[i][j] = dist[r][c] + grid[i][j];
      }
    }
  }

  dist[n_rows - 1][n_cols - 1]
}

pub fn print_grid(grid: &Vec<Vec<i64>>) {
  for row in grid {
    println!(
      "{}",
      row.iter().map(|n| format!("{}", n)).collect::<String>()
    )
  }
}

pub fn add_risk(n: i64, inc: i64) -> i64 {
  if n + inc > 9 {
    n + inc - 9
  } else {
    n + inc
  }
}

pub fn solve_b(input: String) -> i64 {
  const N_TILES: i64 = 5;
  let mut grid = input
    .lines()
    .map(|line| {
      let mut line_part = line
        .chars()
        .map(|c| c.to_string().parse::<i64>().unwrap())
        .collect::<Vec<_>>();

      let extensions: Vec<_> = (1..=N_TILES - 1)
        .map(|i| {
          line_part
            .iter()
            .map(|n| add_risk(*n, i))
            .collect::<Vec<_>>()
        })
        .collect();

      for extension in extensions {
        line_part.extend(extension)
      }

      line_part
    })
    .collect::<Vec<_>>();

  let extensions: Vec<_> = (1..=N_TILES - 1)
    .map(|i| {
      grid
        .iter()
        .map(|row| row.iter().map(|n| add_risk(*n, i)).collect())
        .collect::<Vec<_>>()
    })
    .collect();

  for extension in extensions {
    grid.extend(extension);
  }

  let n_rows = grid.len();
  let n_cols = grid[0].len();
  let n_cells = n_rows * n_cols;

  let mut spt_set = grid
    .iter()
    .map(|row| row.iter().map(|_| false).collect::<Vec<_>>())
    .collect::<Vec<_>>();

  let mut dist = grid
    .iter()
    .map(|row| row.iter().map(|_| i64::MAX).collect::<Vec<_>>())
    .collect::<Vec<_>>();

  dist[0][0] = 0;

  for _ in 0..n_cells {
    let mut min_dist = i64::MAX;
    let mut min_dist_point = (0, 0);

    for (i, row) in dist.iter().enumerate() {
      for (j, &d) in row.iter().enumerate() {
        if d < min_dist && !spt_set[i][j] {
          min_dist = d;
          min_dist_point = (i, j);
        }
      }
    }

    let (r, c) = min_dist_point;

    spt_set[r][c] = true;

    const DIFFS: [(i64, i64); 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];

    for (di, dj) in DIFFS {
      let i = (r as i64 + di) as usize;
      let j = (c as i64 + dj) as usize;

      if i < n_rows && j < n_cols && !spt_set[i][j] && dist[i][j] > dist[r][c] + grid[i][j] {
        dist[i][j] = dist[r][c] + grid[i][j];
      }
    }
  }

  // print_grid(&grid);
  // println!("{}", grid.len());

  dist[n_rows - 1][n_cols - 1]
}

#[cfg(test)]
mod tests {
  use super::*;

  // #[test]
  // fn a_eg_1() {
  //   assert_eq!(solve_a(get_sample_input()), 40);
  // }

  // #[test]
  // fn a() {
  //   assert_eq!(solve_a(get_input()), 619);
  // }

  #[test]
  fn b_eg_1() {
    assert_eq!(solve_b(get_sample_input()), 315);
  }

  #[test]
  fn b() {
    assert_eq!(solve_b(get_input()), 0);
  }

  //
}
