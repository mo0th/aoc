use std::collections::{HashMap, HashSet};

pub fn get_input() -> String {
  String::from(include_str!("../input"))
}

pub fn get_sample_input() -> String {
  String::from(include_str!("../sample"))
}

#[derive(Debug)]
pub enum Cucumber {
  South,
  East,
}

pub fn print_seafloor(seafloor: &HashMap<(usize, usize), Cucumber>, w: usize, h: usize) {
  for i in 0..h {
    for j in 0..w {
      print!(
        "{}",
        if let Some(c) = seafloor.get(&(i, j)) {
          match c {
            Cucumber::East => '>',
            Cucumber::South => 'v',
          }
        } else {
          '.'
        }
      )
    }
    println!();
  }
}

pub fn solve_a(input: String) -> i64 {
  let mut seafloor: HashMap<(usize, usize), Cucumber> = input
    .lines()
    .enumerate()
    .flat_map(|(i, line)| {
      line.chars().enumerate().map(move |(j, c)| {
        (
          (i, j),
          match c {
            '>' => Some(Cucumber::East),
            'v' => Some(Cucumber::South),
            _ => None,
          },
        )
      })
    })
    .filter(|(_, t)| t.is_some())
    .map(|(pos, t)| (pos, t.unwrap()))
    .collect();

  let height = input.lines().count();
  let width = input.lines().next().unwrap().len();

  let mut n_moved = 1;
  let mut step = 0;

  while n_moved > 0 {
    let mut moved = HashSet::new();
    step += 1;

    let mut next_east = HashMap::new();
    let mut next_south = HashMap::new();

    // East-moving cucumbers
    for (&(y, x), _) in seafloor.iter().filter(|(_, c)| matches!(c, Cucumber::East)) {
      let next_x = (x + 1) % width;

      next_east.insert(
        (
          y,
          if seafloor.get(&(y, next_x)).is_some() {
            x
          } else {
            moved.insert((y, x));
            next_x
          },
        ),
        Cucumber::East,
      );
    }

    // East-moving cucumbers
    for (&(y, x), _) in seafloor
      .iter()
      .filter(|(_, c)| matches!(c, Cucumber::South))
    {
      let next_y = (y + 1) % height;

      let cucumber_in_next_pos = next_east.get(&(next_y, x)).is_some()
        || (seafloor.get(&(next_y, x)).is_some() && !moved.contains(&(next_y, x)));

      next_south.insert(
        (
          if cucumber_in_next_pos {
            y
          } else {
            moved.insert((next_y, x));
            next_y
          },
          x,
        ),
        Cucumber::South,
      );
    }

    next_east.extend(next_south);
    seafloor = next_east;
    n_moved = moved.len();
  }

  step
}

pub fn solve_b(_input: String) -> i64 {
  0
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn a_eg_1() {
    assert_eq!(solve_a(get_sample_input()), 58);
  }

  #[test]
  fn a() {
    assert_eq!(solve_a(get_input()), 432);
  }

  //
}
