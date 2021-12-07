use std::collections::HashMap;

pub fn get_input() -> String {
  String::from(include_str!("../input"))
}

pub fn get_sample_input() -> String {
  String::from(include_str!("../sample"))
}

const SPAWN_TIME: i64 = 6;
const INITIAL_SPAWN_TIME: i64 = SPAWN_TIME + 2;

pub fn solve_a(input: String) -> i64 {
  let mut counts: HashMap<i64, i64> = HashMap::new();

  let initial_lanternfish: Vec<_> = input
    .split(",")
    .map(|x| x.parse::<i64>().unwrap())
    .collect();

  for fish in initial_lanternfish {
    *counts.entry(fish).or_insert(0) += 1;
  }

  for _ in 0..80 {
    let mut next_counts = HashMap::new();

    if let Some(&count) = counts.get(&0) {
      next_counts.insert(SPAWN_TIME, count);
      next_counts.insert(INITIAL_SPAWN_TIME, count);
    }

    for (&time_left, &count) in counts.iter() {
      if time_left > 0 {
        *next_counts.entry(time_left - 1).or_insert(0) += count;
      }
    }

    counts = next_counts;
  }

  counts.values().sum()
}

pub fn solve_b(input: String) -> i64 {
  let mut counts: HashMap<i64, i64> = HashMap::new();

  let initial_lanternfish: Vec<_> = input
    .split(",")
    .map(|x| x.parse::<i64>().unwrap())
    .collect();

  for fish in initial_lanternfish {
    *counts.entry(fish).or_insert(0) += 1;
  }

  for _ in 0..256 {
    let mut next_counts = HashMap::new();

    if let Some(&count) = counts.get(&0) {
      next_counts.insert(SPAWN_TIME, count);
      next_counts.insert(INITIAL_SPAWN_TIME, count);
    }

    for (&time_left, &count) in counts.iter() {
      if time_left > 0 {
        *next_counts.entry(time_left - 1).or_insert(0) += count;
      }
    }

    counts = next_counts;
  }

  counts.values().sum()
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn a_eg_1() {
    assert_eq!(solve_a(get_sample_input()), 5934);
  }

  #[test]
  fn a_eg_2() {
    assert_eq!(solve_a("1".to_string()), 1401);
  }

  #[test]
  fn a() {
    assert_eq!(solve_a(get_input()), 365862);
  }

  #[test]
  fn b_eg_1() {
    assert_eq!(solve_b(get_sample_input()), 26984457539);
  }

  #[test]
  fn b() {
    assert_eq!(solve_b(get_input()), 1653250886439);
  }
}
