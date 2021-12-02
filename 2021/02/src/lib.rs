use std::error::Error;
use std::fs;

pub fn get_input() -> Result<String, Box<dyn Error>> {
  let input = fs::read_to_string("input")?;

  Ok(input)
}

pub fn solve_a(input: String) -> String {
  let mut depth = 0;
  let mut position = 0;

  for line in input.lines() {
    let parts: Vec<String> = line.split(" ").map(|s| String::from(s)).collect();

    let dir = &parts[0];
    let amount = (&parts[1]).parse::<i32>().unwrap();

    match dir.as_str() {
      "forward" => position += amount,
      // "forward" => {
      //   position -= amount
      // },
      "up" => depth -= amount,
      "down" => depth += amount,
      _ => (),
    }
  }

  String::from(format!("{}", depth * position))
}

pub fn solve_b(input: String) -> String {
  let mut depth = 0;
  let mut position = 0;
  let mut aim = 0;

  for line in input.lines() {
    let parts: Vec<String> = line.split(" ").map(|s| String::from(s)).collect();

    let dir = &parts[0];
    let amount = (&parts[1]).parse::<i32>().unwrap();

    match dir.as_str() {
      "forward" => {
        position += amount;
        depth += aim * amount;
      }
      "up" => aim -= amount,
      "down" => aim += amount,
      _ => (),
    }
  }

  String::from(format!("{}", depth * position))
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn a_eg_1() {
    assert_eq!(
      solve_a(String::from(
        "forward 5
down 5
forward 8
up 3
down 8
forward 2"
      )),
      "150"
    );
  }

  #[test]
  fn b_eg_1() {
    assert_eq!(
      solve_b(String::from(
        "forward 5
down 5
forward 8
up 3
down 8
forward 2"
      )),
      "900"
    );
  }
}
