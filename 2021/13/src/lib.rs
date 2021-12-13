use std::collections::HashSet;

pub fn get_input() -> String {
  String::from(include_str!("../input"))
}

pub fn get_sample_input() -> String {
  String::from(include_str!("../sample"))
}

pub fn solve_a(input: String) -> i64 {
  let parts: Vec<_> = input.split("\n\n").collect();

  let mut points: HashSet<_> = parts[0]
    .lines()
    .map(|line| {
      let parts: Vec<_> = line.split(",").map(|p| p.parse::<i64>().unwrap()).collect();
      (parts[0], parts[1])
    })
    .collect();

  let folds: Vec<_> = parts[1]
    .lines()
    .map(|line| {
      let fold_parts = line.split(" ").collect::<Vec<_>>()[2]
        .split("=")
        .collect::<Vec<_>>();
      (fold_parts[0], fold_parts[1].parse::<i64>().unwrap())
    })
    .collect();

  let (axis, num) = folds[0];

  points = points
    .iter()
    .map(|&(x, y)| match axis {
      "x" => (if x > num { 2 * num - x } else { x }, y),
      "y" => (x, if y > num { 2 * num - y } else { y }),
      _ => panic!("Unknown axis"),
    })
    .collect();

  points.len() as i64
}

fn print_points(points: &HashSet<(i64, i64)>) {
  let max_x = points.iter().map(|&(x, _)| x).max().unwrap();
  let max_y = points.iter().map(|&(_, y)| y).max().unwrap();

  let mut grid = vec![vec![' '; max_x as usize + 1]; max_y as usize + 1];

  for &(x, y) in points {
    grid[y as usize][x as usize] = '#';
  }

  for row in grid {
    println!("{}", row.iter().collect::<String>());
  }
}

pub fn solve_b(input: String) -> i64 {
  let parts: Vec<_> = input.split("\n\n").collect();

  let mut points: HashSet<_> = parts[0]
    .lines()
    .map(|line| {
      let parts: Vec<_> = line.split(",").map(|p| p.parse::<i64>().unwrap()).collect();
      (parts[0], parts[1])
    })
    .collect();

  let folds: Vec<_> = parts[1]
    .lines()
    .map(|line| {
      let fold_parts = line.split(" ").collect::<Vec<_>>()[2]
        .split("=")
        .collect::<Vec<_>>();
      (fold_parts[0], fold_parts[1].parse::<i64>().unwrap())
    })
    .collect();

  for (axis, num) in folds {
    points = points
      .iter()
      .map(|&(x, y)| match axis {
        "x" => (if x > num { 2 * num - x } else { x }, y),
        "y" => (x, if y > num { 2 * num - y } else { y }),
        _ => (x, y),
      })
      .collect();
  }

  print_points(&points);

  0
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn a_eg_1() {
    assert_eq!(solve_a(get_sample_input()), 17);
  }

  #[test]
  fn a() {
    assert_eq!(solve_a(get_input()), 708);
  }

  #[test]
  fn b_eg_1() {
    assert_eq!(solve_b(get_sample_input()), 0);
  }

  #[test]
  fn b() {
    assert_eq!(solve_b(get_input()), 0);
  }

  //
}
