use std::collections::HashSet;

pub fn get_input() -> String {
  String::from(include_str!("../input"))
}

pub fn get_sample_input() -> String {
  String::from(include_str!("../sample"))
}

type Point = (i64, i64, i64);

fn parse(input: &str) -> Vec<Vec<Point>> {
  input
    .split("\n\n")
    .map(|scanner_str| {
      scanner_str
        .lines()
        .skip(1)
        .map(|line| {
          let parts: Vec<_> = line.split(",").map(|s| s.parse().unwrap()).collect();
          (parts[0], parts[1], parts[2])
        })
        .collect()
    })
    .collect()
}

fn point_diff(p1: &Point, p2: &Point) -> Point {
  (
    (p1.0 - p2.0).abs(),
    (p1.1 - p2.1).abs(),
    (p1.2 - p2.2).abs(),
  )
}

fn get_diff_map(report: &Vec<Point>) -> Vec<Vec<Point>> {
  report
    .iter()
    .map(|p1| report.iter().map(|p2| point_diff(p1, p2)).collect())
    .collect()
}

fn diff_eq(d1: &Point, d2: &Point) -> bool {
  let mut v1 = vec![d1.0, d1.1, d1.2];
  v1.sort();
  let mut v2 = vec![d2.0, d2.1, d2.2];
  v2.sort();
  v1 == v2
}

fn get_common_indexes(diff_map_1: &Vec<Vec<Point>>, diff_map_2: &Vec<Vec<Point>>) -> Vec<usize> {
  let mut indexes = HashSet::new();

  for (i, diffs1) in diff_map_1.iter().enumerate() {
    let mut n_same = 0;
    for (j, diff1) in diffs1.iter().enumerate() {
      if i == j {
        continue;
      }
      for (k, diffs2) in diff_map_2.iter().enumerate() {
        for (l, diff2) in diffs2.iter().enumerate() {
          if k == l {
            continue;
          }
          if diff_eq(&diff1, &diff2) {
            n_same += 1;
          }
        }
      }
    }

    indexes.insert(i);
  }

  indexes.into_iter().collect()
}

pub fn solve_a(input: String) -> i64 {
  let reports = parse(&input);
  println!("{:?}", reports.iter().map(|s| s.len()).collect::<Vec<_>>());
  let diff_maps = reports.iter().map(|r| get_diff_map(r)).collect::<Vec<_>>();

  let common_indexes_map = (0..reports.len())
    .map(|i| {
      (0..reports.len())
        .map(|j| get_common_indexes(&diff_maps[i], &diff_maps[j]))
        .collect::<Vec<_>>()
    })
    .collect::<Vec<_>>();

  0
}

pub fn solve_b(input: String) -> i64 {
  0
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn a_eg_1() {
    assert_eq!(solve_a(get_sample_input()), 79);
  }

  // #[test]
  // fn a() {
  //   assert_eq!(solve_a(get_input()), 0);
  // }

  // #[test]
  // fn b_eg_1() {
  //   assert_eq!(solve_b(get_sample_input()), 0);
  // }

  // #[test]
  // fn b() {
  //   assert_eq!(solve_b(get_input()), 0);
  // }

  //
}
