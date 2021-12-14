use std::collections::HashMap;

pub fn get_input() -> String {
  String::from(include_str!("../input"))
}

pub fn get_sample_input() -> String {
  String::from(include_str!("../sample"))
}

pub fn solve_a(input: String) -> i64 {
  let parts: Vec<_> = input.split("\n\n").collect();

  let mut polymer: Vec<_> = parts[0].chars().collect();

  let rules: HashMap<_, _> = parts[1]
    .split("\n")
    .map(|line| {
      let parts: Vec<_> = line.split(" -> ").collect();
      let left: String = parts[0].chars().collect();
      let right: Vec<_> = parts[1].chars().collect();
      (left, right[0])
    })
    .collect();

  for _ in 0..10 {
    let mut next_polymer: Vec<_> = polymer
      .windows(2)
      .flat_map(|window| {
        let pattern: String = window.iter().collect();
        return [window[0], *rules.get(&pattern).unwrap()];
      })
      .collect();
    next_polymer.push(*polymer.last().unwrap());
    polymer = next_polymer
  }

  let counts = polymer.iter().fold(HashMap::new(), |mut acc, c| {
    *acc.entry(c).or_insert(0) += 1;
    acc
  });

  let min_count = counts.values().min().unwrap();
  let max_count = counts.values().max().unwrap();

  max_count - min_count
}

pub fn solve_b(input: String) -> i64 {
  let parts: Vec<_> = input.split("\n\n").collect();

  let mut polymer: Vec<_> = parts[0].chars().collect();

  let rules: HashMap<_, _> = parts[1]
    .split("\n")
    .map(|line| {
      let parts: Vec<_> = line.split(" -> ").collect();
      let left: String = parts[0].chars().collect();
      let right: Vec<_> = parts[1].chars().collect();
      (left, right[0])
    })
    .collect();

  for _ in 0..40 {
    let mut next_polymer: Vec<_> = polymer
      .windows(2)
      .flat_map(|window| {
        let pattern: String = window.iter().collect();
        return [window[0], *rules.get(&pattern).unwrap()];
      })
      .collect();
    next_polymer.push(*polymer.last().unwrap());
    polymer = next_polymer;
  }

  let counts = polymer.iter().fold(HashMap::new(), |mut acc, c| {
    *acc.entry(c).or_insert(0) += 1;
    acc
  });

  let min_count = counts.values().min().unwrap();
  let max_count = counts.values().max().unwrap();

  max_count - min_count
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn a_eg_1() {
    assert_eq!(solve_a(get_sample_input()), 1588);
  }

  #[test]
  fn a() {
    assert_eq!(solve_a(get_input()), 2874);
  }

  #[test]
  fn b_eg_1() {
    assert_eq!(solve_b(get_sample_input()), 2188189693529);
  }

  #[test]
  fn b() {
    assert_eq!(solve_b(get_input()), 0);
  }

  //
}
