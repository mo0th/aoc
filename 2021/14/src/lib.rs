use std::collections::HashMap;

pub fn get_input() -> String {
  String::from(include_str!("../input"))
}

pub fn get_sample_input() -> String {
  String::from(include_str!("../sample"))
}

pub fn solve_a(input: String) -> i64 {
  let parts: Vec<_> = input.split("\n\n").collect();

  let polymer: Vec<_> = parts[0].chars().collect();

  let rules: HashMap<_, _> = parts[1]
    .split("\n")
    .map(|line| {
      let parts: Vec<_> = line.split(" -> ").collect();
      let left: String = parts[0].chars().collect();
      let right: Vec<_> = parts[1].chars().collect();
      (left, right[0])
    })
    .collect();

  let mut counts = parts[0]
    .chars()
    .collect::<Vec<_>>()
    .windows(2)
    .map(|window| window.iter().collect::<String>())
    .into_iter()
    .fold(HashMap::new(), |mut acc, c| {
      *acc.entry(c).or_insert(0) += 1;
      acc
    });

  for _ in 0..10 {
    let new_counts = counts
      .iter()
      .fold(HashMap::new(), |mut acc, (pair, count)| {
        let to_insert = rules.get(pair).unwrap();
        let chars: Vec<_> = pair.chars().collect();
        let new_pair = chars[0].to_string() + &to_insert.to_string();
        *acc.entry(new_pair).or_insert(0) += count;
        let new_pair = to_insert.to_string() + &chars[1].to_string();
        *acc.entry(new_pair).or_insert(0) += count;

        acc
      });

    counts = new_counts;
  }

  let mut char_counts = counts
    .iter()
    .fold(HashMap::new(), |mut acc, (pair, count)| {
      for c in pair.chars() {
        *acc.entry(c).or_insert(0) += count;
      }

      acc
    });

  let first_char = polymer.first().unwrap();
  let last_char = polymer.last().unwrap();
  for c in [first_char, last_char] {
    *char_counts.entry(*c).or_insert(0) += 1;
  }

  char_counts = char_counts
    .into_iter()
    .map(|(pair, count)| (pair, count / 2))
    .collect();

  let min_count = char_counts.values().min().unwrap();
  let max_count = char_counts.values().max().unwrap();

  max_count - min_count
}

pub fn solve_b(input: String) -> i64 {
  let parts: Vec<_> = input.split("\n\n").collect();

  let polymer: Vec<_> = parts[0].chars().collect();

  let rules: HashMap<_, _> = parts[1]
    .split("\n")
    .map(|line| {
      let parts: Vec<_> = line.split(" -> ").collect();
      let left: String = parts[0].chars().collect();
      let right: Vec<_> = parts[1].chars().collect();
      (left, right[0])
    })
    .collect();

  let mut counts = parts[0]
    .chars()
    .collect::<Vec<_>>()
    .windows(2)
    .map(|window| window.iter().collect::<String>())
    .into_iter()
    .fold(HashMap::new(), |mut acc, c| {
      *acc.entry(c).or_insert(0) += 1;
      acc
    });

  for _ in 0..40 {
    let new_counts = counts
      .iter()
      .fold(HashMap::new(), |mut acc, (pair, count)| {
        let to_insert = rules.get(pair).unwrap();
        let chars: Vec<_> = pair.chars().collect();
        let new_pair = chars[0].to_string() + &to_insert.to_string();
        *acc.entry(new_pair).or_insert(0) += count;
        let new_pair = to_insert.to_string() + &chars[1].to_string();
        *acc.entry(new_pair).or_insert(0) += count;

        acc
      });

    counts = new_counts;
  }

  let mut char_counts = counts
    .iter()
    .fold(HashMap::new(), |mut acc, (pair, count)| {
      for c in pair.chars() {
        *acc.entry(c).or_insert(0) += count;
      }

      acc
    });

  let first_char = polymer.first().unwrap();
  let last_char = polymer.last().unwrap();
  for c in [first_char, last_char] {
    *char_counts.entry(*c).or_insert(0) += 1;
  }

  char_counts = char_counts
    .into_iter()
    .map(|(pair, count)| (pair, count / 2))
    .collect();

  let min_count = char_counts.values().min().unwrap();
  let max_count = char_counts.values().max().unwrap();

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
    assert_eq!(solve_b(get_input()), 5208377027195);
  }

  //
}
