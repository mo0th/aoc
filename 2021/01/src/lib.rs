use std::error::Error;
use std::fs;

pub fn get_input() -> Result<String, Box<dyn Error>> {
  let input = fs::read_to_string("input")?;

  Ok(input)
}

pub fn solve_a(input: String) -> String {
  let numbers: Vec<i32> = input
    .split_whitespace()
    .map(|s| s.parse().unwrap())
    .collect();

  let mut count = 0;

  for window in numbers.windows(2) {
    if let &[n1, n2] = window {
      if n2 > n1 {
        count += 1
      }
    }
  }

  String::from(format!("{}", count))
}

pub fn solve_b(input: String) -> String {
  let numbers: Vec<i32> = input
    .split_whitespace()
    .map(|s| s.parse().unwrap())
    .collect();

  let mut count = 0;

  for window in numbers.windows(4) {
    if let &[n1, n2, n3, n4] = window {
      let s1 = n1 + n2 + n3;
      let s2 = n2 + n3 + n4;
      if s2 > s1 {
        count += 1
      }
    }
  }

  String::from(format!("{}", count))
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn a_eg_1() {
    assert_eq!(solve_a(String::from("sample input")), "sample input");
  }

  #[test]
  fn b_eg_1() {
    assert_eq!(solve_b(String::from("sample input")), "sample input");
  }
}
