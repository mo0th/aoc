use std::collections::HashSet;

pub fn get_input() -> String {
  String::from(include_str!("../input"))
}

pub fn get_sample_input() -> String {
  String::from(include_str!("../sample"))
}

pub fn solve_a(input: String) -> i64 {
  let steps = input
    .as_str()
    .replace("x=", "")
    .replace("y=", "")
    .replace("z=", "")
    .lines()
    .map(|line| {
      let parts = line.split(" ").collect::<Vec<_>>();
      let on = match parts[0] {
        "on" => true,
        _ => false,
      };

      let ranges = parts[1]
        .split(",")
        .map(|range_str| {
          let range_vec = range_str
            .split("..")
            .map(|p| p.parse::<i64>().unwrap())
            .collect::<Vec<_>>();

          (range_vec[0], range_vec[1])
        })
        .collect::<Vec<_>>();

      (on, ranges[0], ranges[1], ranges[2])
    })
    .collect::<Vec<_>>();

  let mut on_cubes: HashSet<(i64, i64, i64)> = HashSet::new();

  for (on, (x_min, x_max), (y_min, y_max), (z_min, z_max)) in steps {
    if x_min < -50 || x_max > 50 || y_min < -50 || y_max > 50 || z_min < -50 || z_max > 50 {
      continue;
    }

    for x in x_min..=x_max {
      for y in y_min..=y_max {
        for z in z_min..=z_max {
          let point = (x, y, z);

          // println!("{:?}", &point);

          if on {
            on_cubes.insert(point);
          } else {
            if on_cubes.contains(&point) {
              on_cubes.remove(&point);
            }
          }
        }
      }
    }
  }

  on_cubes.len() as i64
}

#[derive(Debug)]
struct Cube {
  x: (i64, i64),
  y: (i64, i64),
  z: (i64, i64),
}

pub fn solve_b(input: String) -> i64 {
  0
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn a_eg_1() {
    assert_eq!(solve_a(get_sample_input()), 590784);
  }

  #[test]
  fn a() {
    assert_eq!(solve_a(get_input()), 581108);
  }

  // #[test]
  // fn b_eg_1() {
  //   assert_eq!(solve_b(get_sample_input()), 0);
  // }

  // #[test]
  // fn b() {
  //   assert_eq!(solve_b(get_input()), 0);
  // }

  //
}
