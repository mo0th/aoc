pub fn get_input() -> String {
  String::from(include_str!("../input"))
}

pub fn get_sample_input() -> String {
  String::from(include_str!("../sample"))
}

pub fn solve_a(input: String) -> i64 {
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
    assert_eq!(solve_a(get_sample_input()), 0);
  }

  #[test]
  fn a() {
    assert_eq!(solve_a(get_input()), 0);
  }

  #[test]
  fn b_eg_1() {
    assert_eq!(solve_b(get_sample_input()), 0);
  }

  #[test]
  fn b() {
    assert_eq!(solve_b(get_input()), 0);
  }
}
