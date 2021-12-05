use std::collections::HashMap;
use std::error::Error;
use std::fs;

pub fn get_input() -> Result<String, Box<dyn Error>> {
  let input = fs::read_to_string("input")?;
  Ok(input)
}

fn parse_input(input: &str) -> Vec<((i32, i32), (i32, i32))> {
  input
    .lines()
    .map(|line| {
      let parts: Vec<_> = line
        .split(" -> ")
        .map(|part| {
          let nums: Vec<_> = part.split(",").map(|s| s.parse::<i32>().unwrap()).collect();

          (nums[0], nums[1])
        })
        .collect();

      (parts[0], parts[1])
    })
    .collect()
}

fn min_max(a: i32, b: i32) -> (i32, i32) {
  if a > b {
    (b, a)
  } else {
    (a, b)
  }
}

pub fn solve_a(input: String) -> String {
  // Weird lifetime thing idk
  let lines = parse_input(&input);
  let lines = lines
    .iter()
    .filter(|((x1, y1), (x2, y2))| x1 == x2 || y1 == y2);

  let mut counts = HashMap::new();

  for ((x1, y1), (x2, y2)) in lines {
    let (x1, x2) = min_max(*x1, *x2);
    let (y1, y2) = min_max(*y1, *y2);

    let (start, end) = if x1 == x2 { (y1, y2) } else { (x1, x2) };

    for i in start..=end {
      let key = if x1 == x2 { (x1, i) } else { (i, y1) };

      *counts.entry(key).or_insert(0) += 1;
    }
  }

  String::from(format!("{}", counts.values().filter(|&x| *x >= 2).count()))
}

fn visualize_counts(counts: &HashMap<(i32, i32), i32>) -> String {
  let max_x = counts.keys().map(|(x, _)| *x).max().unwrap();
  let max_y = counts.keys().map(|(y, _)| *y).max().unwrap();

  let max = counts.values().max().unwrap();
  let max_num_len = max.to_string().len();

  let mut output = String::new();

  for i in 0..max_x {
    for j in 0..max_y {
      let key = (i, j);

      let count = counts.get(&key).unwrap_or(&0);

      output.push_str(&format!(" {:>w$}", count, w = max_num_len));
    }
    output.push('\n');
  }

  output
}

pub fn solve_b(input: String) -> String {
  // Weird lifetime thing idk
  let lines = parse_input(&input);
  let lines = lines.iter();

  let mut counts = HashMap::new();

  for &((x1, y1), (x2, y2)) in lines {
    println!("{:?} -> {:?}", (x1, y1), (x2, y2));

    if x1 == x2 || y1 == y2 {
      let (x1, x2) = min_max(x1, x2);
      let (y1, y2) = min_max(y1, y2);

      let (start, end) = if x1 == x2 { min_max(y1, y2) } else { (x1, x2) };

      for i in start..=end {
        let key = if x1 == x2 { (x1, i) } else { (i, y1) };

        *counts.entry(key).or_insert(0) += 1;
      }
    } else {
      let diff = (x1 - x2).abs();
      let x_mult = if x1 > x2 { -1 } else { 1 };
      let y_mult = if y1 > y2 { -1 } else { 1 };

      for i in 0..=diff {
        let x = x1 + i * x_mult;
        let y = y1 + i * y_mult;

        let key = (x, y);

        println!("  {:?}", key);

        *counts.entry(key).or_insert(0) += 1;
      }
    }
  }

  println!("{}", visualize_counts(&counts));

  String::from(format!("{}", counts.values().filter(|&x| *x >= 2).count()))
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn a_eg_1() {
    assert_eq!(
      solve_a(String::from(
        "0,9 -> 5,9
8,0 -> 0,8
9,4 -> 3,4
2,2 -> 2,1
7,0 -> 7,4
6,4 -> 2,0
0,9 -> 2,9
3,4 -> 1,4
0,0 -> 8,8
5,5 -> 8,2"
      )),
      "5"
    );
  }

  #[test]
  fn b_eg_1() {
    assert_eq!(
      solve_b(String::from(
        "0,9 -> 5,9
8,0 -> 0,8
9,4 -> 3,4
2,2 -> 2,1
7,0 -> 7,4
6,4 -> 2,0
0,9 -> 2,9
3,4 -> 1,4
0,0 -> 8,8
5,5 -> 8,2"
      )),
      "12"
    );
  }
}
