use std::collections::HashMap;
use std::error::Error;
use std::fs;

pub fn get_input() -> Result<String, Box<dyn Error>> {
  let input = fs::read_to_string("input")?;

  Ok(input)
}

pub fn solve_a(input: String) -> String {
  let mut map = HashMap::new();
  let n_lines = input.lines().count();
  let half_lines = n_lines / 2;
  let size = String::from(input.lines().take(1).last().unwrap()).len();

  for line in input.lines() {
    for (i, ch) in line.chars().enumerate() {
      if ch == '1' {
        map.insert(i, map.get(&i).unwrap_or(&0) + 1);
      }
    }
  }

  let mut gamma_rate = vec!['0'; size];
  let mut epsilon_rate = vec!['1'; size];

  for (&key, &val) in map.iter() {
    if val > half_lines {
      gamma_rate[key] = '1';
      epsilon_rate[key] = '0';
    }
  }

  let gamma_rate = i32::from_str_radix(&gamma_rate.iter().collect::<String>(), 2).unwrap();
  let epsilon_rate = i32::from_str_radix(&epsilon_rate.iter().collect::<String>(), 2).unwrap();

  String::from(format!("{}", gamma_rate * epsilon_rate))
}

pub fn solve_b(input: String) -> String {
  let size = String::from(input.lines().take(1).last().unwrap()).len();

  let mut oxy_nums: Vec<_> = input.lines().collect();
  let mut co2_nums: Vec<_> = input.lines().collect();

  for i in 0..size {
    if oxy_nums.len() == 1 {
      break;
    }
    let count = oxy_nums
      .iter()
      .filter(|s| s.get(i..i + 1).unwrap() == "1")
      .count();

    let len = oxy_nums.len();
    let ch = if count * 2 >= len { "1" } else { "0" };

    oxy_nums = oxy_nums
      .into_iter()
      .filter(|n| n.get(i..i + 1).unwrap() == ch)
      .collect();
  }

  for i in 0..size {
    if co2_nums.len() == 1 {
      break;
    }
    let count = co2_nums
      .iter()
      .filter(|s| s.get(i..i + 1).unwrap() == "1")
      .count();

    let len = co2_nums.len();
    let ch = if count * 2 >= len { "0" } else { "1" };

    co2_nums = co2_nums
      .into_iter()
      .filter(|n| n.get(i..i + 1).unwrap() == ch)
      .collect();
  }

  let oxy = i32::from_str_radix(oxy_nums.last().unwrap(), 2).unwrap();
  let co2 = i32::from_str_radix(co2_nums.last().unwrap(), 2).unwrap();

  String::from(format!("{}", oxy * co2))
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn a_eg_1() {
    assert_eq!(
      solve_a(String::from(
        "00100
11110
10110
10111
10101
01111
00111
11100
10000
11001
00010
01010"
      )),
      "198"
    );
  }

  #[test]
  fn b_eg_1() {
    assert_eq!(
      solve_b(String::from(
        "00100
11110
10110
10111
10101
01111
00111
11100
10000
11001
00010
01010"
      )),
      "230"
    );
  }
}
