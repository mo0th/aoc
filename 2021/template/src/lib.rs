use std::error::Error;
use std::fs;

pub fn get_input() -> Result<String, Box<dyn Error>> {
  let input = fs::read_to_string("input")?;

  Ok(input)
}

pub fn solve_a(input: String) -> String {
  input
}

pub fn solve_b(input: String) -> String {
  input
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
