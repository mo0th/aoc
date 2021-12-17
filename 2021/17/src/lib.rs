pub fn get_input() -> String {
  String::from(include_str!("../input"))
}

pub fn get_sample_input() -> String {
  String::from(include_str!("../sample"))
}

fn step(vel: &Vector) -> Vector {
  let (x, y) = *vel;

  let next_x = if x == 0 {
    0
  } else if x > 0 {
    x - 1
  } else {
    x + 1
  };

  (next_x, y - 1)
}

fn add_vel(pos: &Vector, vel: &Vector) -> Vector {
  let (x, y) = *pos;
  let (v_x, v_y) = *vel;

  (x + v_x, y + v_y)
}

type Range = ((i64, i64), (i64, i64));
type Vector = (i64, i64);

fn get_range(input: String) -> Range {
  let s = input
    .as_str()
    .replace("target area: x=", "")
    .replace("y=", "");

  let pairs = s
    .split(", ")
    .map(|part| {
      let range_parts = part
        .split("..")
        .map(|n| n.parse::<i64>().unwrap())
        .collect::<Vec<_>>();

      (range_parts[0], range_parts[1])
    })
    .collect::<Vec<_>>();

  (pairs[0], pairs[1])
}

fn in_range(range: &Range, pos: &Vector) -> bool {
  let (x, y) = *pos;

  let (x_range, y_range) = *range;

  x >= x_range.0 && x <= x_range.1 && y >= y_range.0 && y <= y_range.1
}

fn is_past_range(range: &Range, pos: &Vector) -> bool {
  let (x, y) = *pos;

  let (x_range, y_range) = *range;

  x > x_range.1 || y < y_range.0
}

pub fn solve_a(input: String) -> i64 {
  let (x_range, y_range) = get_range(input);

  let mut valid_start_velocities = vec![];

  for x in 0..=x_range.1 {
    for y in 0..=y_range.0.abs() {
      let mut p = (0, 0);
      let mut v = (x, y);

      let range = (x_range, y_range);

      while !is_past_range(&range, &p) {
        p = add_vel(&p, &v);
        v = step(&v);

        if in_range(&range, &p) {
          valid_start_velocities.push((x, y));
          break;
        }
      }
    }
  }

  valid_start_velocities
    .iter()
    .map(|(_, v_y)| v_y * (v_y + 1) / 2)
    .max()
    .unwrap()
}

pub fn solve_b(input: String) -> i64 {
  let (x_range, y_range) = get_range(input);

  let mut valid_start_velocities = vec![];

  for x in 0..=x_range.1 {
    for y in y_range.0..=y_range.0.abs() {
      let mut p = (0, 0);
      let mut v = (x, y);

      let range = (x_range, y_range);

      while !is_past_range(&range, &p) {
        p = add_vel(&p, &v);
        v = step(&v);

        if in_range(&range, &p) {
          valid_start_velocities.push((x, y));
          break;
        }
      }
    }
  }

  valid_start_velocities.len() as i64
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn a_eg_1() {
    assert_eq!(solve_a(get_sample_input()), 45);
  }

  #[test]
  fn a() {
    assert_eq!(solve_a(get_input()), 10296);
  }

  #[test]
  fn b_eg_1() {
    assert_eq!(solve_b(get_sample_input()), 112);
  }

  #[test]
  fn b() {
    assert_eq!(solve_b(get_input()), 2371);
  }

  //
}
