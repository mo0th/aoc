use std::collections::{HashMap, HashSet};

pub fn get_input() -> String {
  String::from(include_str!("../input"))
}

pub fn get_sample_input() -> String {
  String::from(include_str!("../sample"))
}

pub fn solve_a(input: String) -> i64 {
  let opening: HashSet<char> = "([{<".chars().collect();
  let opening_to_closing: HashMap<char, char> = [('(', ')'), ('{', '}'), ('[', ']'), ('<', '>')]
    .iter()
    .cloned()
    .collect();

  input
    .lines()
    .map(|line| {
      let mut stack = vec![];

      for ch in line.chars() {
        if opening.contains(&ch) {
          stack.push(ch);
        } else if let Some(last) = stack.pop() {
          if opening_to_closing[&last] != ch {
            return Some(ch);
          }
        }
      }

      return None;
    })
    .map(|maybe_ch| match maybe_ch {
      Some(')') => 3,
      Some(']') => 57,
      Some('>') => 25137,
      Some('}') => 1197,
      _ => 0,
    })
    .sum()
}

pub fn solve_b(input: String) -> i64 {
  let opening: HashSet<char> = "([{<".chars().collect();
  let opening_to_closing: HashMap<char, char> = [('(', ')'), ('{', '}'), ('[', ']'), ('<', '>')]
    .iter()
    .cloned()
    .collect();

  let mut scores = input
    .lines()
    .map(|line| {
      let mut stack = vec![];

      for ch in line.chars() {
        if opening.contains(&ch) {
          stack.push(ch);
        } else if let Some(last) = stack.pop() {
          if opening_to_closing[&last] != ch {
            return None;
          }
        }
      }

      return Some(
        stack
          .into_iter()
          .map(|ch| opening_to_closing[&ch])
          .rev()
          .fold(0, |acc, ch| {
            acc * 5
              + (match ch {
                ')' => 1,
                ']' => 2,
                '}' => 3,
                '>' => 4,
                _ => 0,
              })
          }),
      );
    })
    .filter(|score| score.is_some())
    .collect::<Vec<_>>();

  scores.sort();

  scores[scores.len() / 2].unwrap()
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn a_eg_1() {
    assert_eq!(solve_a(get_sample_input()), 26397);
  }

  #[test]
  fn a() {
    assert_eq!(solve_a(get_input()), 392139);
  }

  #[test]
  fn b_eg_1() {
    assert_eq!(solve_b(get_sample_input()), 288957);
  }

  #[test]
  fn b() {
    assert_eq!(solve_b(get_input()), 4001832844);
  }
}
