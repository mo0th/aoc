pub fn get_input() -> String {
  String::from(include_str!("../input"))
}

pub fn get_sample_input() -> String {
  String::from(include_str!("../sample"))
}

pub fn solve_a(input: String) -> i64 {
  let mut nums = input
    .split(",")
    .map(|s| s.parse::<i64>().unwrap())
    .collect::<Vec<i64>>();

  nums.sort();

  let len = nums.len();

  let target = if len % 2 == 0 {
    (nums[len / 2] + nums[len / 2 - 1]) / 2
  } else {
    nums[len / 2]
  };

  nums.iter().map(|n| (*n - target).abs()).sum()
}

fn sum_to_n(n: i64) -> i64 {
  n * (n + 1) / 2
}

pub fn solve_b(input: String) -> i64 {
  let nums = input
    .split(",")
    .map(|s| s.parse::<i64>().unwrap())
    .collect::<Vec<i64>>();

  let copy: Vec<_> = input
    .split(",")
    .map(|s| s.parse::<i64>().unwrap())
    .collect();

  let max_pos = *nums.iter().max().unwrap();

  let mut min_fuel = i64::MAX;

  for n in 0..=max_pos {
    let fuel_used = copy.iter().map(|p| sum_to_n((p - n).abs())).sum::<i64>();

    if fuel_used < min_fuel {
      min_fuel = fuel_used;
    }
  }

  min_fuel
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn a_eg_1() {
    assert_eq!(solve_a(get_sample_input()), 37);
  }

  #[test]
  fn a() {
    assert_eq!(solve_a(get_input()), 337833);
  }

  #[test]
  fn b_eg_1() {
    assert_eq!(solve_b(get_sample_input()), 168);
  }

  #[test]
  fn b() {
    assert_eq!(solve_b(get_input()), 96678050);
  }
}
