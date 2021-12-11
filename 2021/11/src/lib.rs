pub fn get_input() -> String {
  String::from(include_str!("../input"))
}

pub fn get_sample_input() -> String {
  String::from(include_str!("../sample"))
}

pub fn print_flashed(energy_levels: &Vec<Vec<bool>>) {
  for row in energy_levels {
    for cell in row {
      print!(
        "{}",
        match cell {
          true => '#',
          false => '.',
        }
      );
    }
    println!();
  }
}

pub fn print_energy_levels(energy_levels: &Vec<Vec<i64>>) {
  for row in energy_levels {
    for cell in row {
      print!("{}", cell);
    }
    println!();
  }
}

fn any_can_flash(energy_levels: &Vec<Vec<i64>>, has_flashed: &Vec<Vec<bool>>) -> bool {
  for (i, row) in energy_levels.iter().enumerate() {
    for (j, cell) in row.iter().enumerate() {
      if *cell > 9 && !has_flashed[i][j] {
        return true;
      }
    }
  }

  false
}

fn get_adjacent(i: usize, j: usize) -> Vec<(usize, usize)> {
  const DIFFS: [(i64, i64); 8] = [
    (0, -1),
    (1, 0),
    (0, 1),
    (-1, 0),
    (1, 1),
    (1, -1),
    (-1, 1),
    (-1, -1),
  ];

  let mut adjacent = vec![];

  for (di, dj) in DIFFS.iter() {
    let i = (i as i64 + di) as usize;
    let j = (j as i64 + dj) as usize;

    if i < 10 && j < 10 {
      adjacent.push((i, j));
    }
  }

  adjacent
}

pub fn solve_a(input: String) -> i64 {
  let mut energy_levels = input
    .lines()
    .map(|line| {
      line
        .chars()
        .map(|ch| ch.to_string().parse::<i64>().unwrap())
        .collect::<Vec<_>>()
    })
    .collect::<Vec<_>>();

  let mut flashes = 0;

  for _ in 0..100 {
    let mut has_flashed: Vec<Vec<bool>> =
      (0..10).map(|_| (0..10).map(|_| false).collect()).collect();

    energy_levels = energy_levels
      .iter()
      .map(|row| row.iter().map(|e| e + 1).collect())
      .collect();

    while any_can_flash(&energy_levels, &has_flashed) {
      let mut to_increment = vec![];

      for (i, row) in energy_levels.iter_mut().enumerate() {
        for (j, cell) in row.iter_mut().enumerate() {
          if *cell > 9 && !has_flashed[i][j] {
            has_flashed[i][j] = true;
            let mut adj = get_adjacent(i, j);
            to_increment.append(&mut adj);
            flashes += 1;
          }
        }
      }

      for &(i, j) in to_increment.iter() {
        energy_levels[i][j] += 1;
      }

      for (i, row) in has_flashed.iter().enumerate() {
        for (j, flashed) in row.iter().enumerate() {
          if *flashed {
            energy_levels[i][j] = 0;
          }
        }
      }
    }
  }

  flashes
}

pub fn solve_b(input: String) -> i64 {
  let mut energy_levels = input
    .lines()
    .map(|line| {
      line
        .chars()
        .map(|ch| ch.to_string().parse::<i64>().unwrap())
        .collect::<Vec<_>>()
    })
    .collect::<Vec<_>>();

  let mut step = 0;

  loop {
    step += 1;

    let mut has_flashed: Vec<Vec<bool>> =
      (0..10).map(|_| (0..10).map(|_| false).collect()).collect();

    energy_levels = energy_levels
      .iter()
      .map(|row| row.iter().map(|e| e + 1).collect())
      .collect();

    while any_can_flash(&energy_levels, &has_flashed) {
      let mut to_increment = vec![];

      for (i, row) in energy_levels.iter_mut().enumerate() {
        for (j, cell) in row.iter_mut().enumerate() {
          if *cell > 9 && !has_flashed[i][j] {
            has_flashed[i][j] = true;
            let mut adj = get_adjacent(i, j);
            to_increment.append(&mut adj);
          }
        }
      }

      for &(i, j) in to_increment.iter() {
        energy_levels[i][j] += 1;
      }

      for (i, row) in has_flashed.iter().enumerate() {
        for (j, flashed) in row.iter().enumerate() {
          if *flashed {
            energy_levels[i][j] = 0;
          }
        }
      }
    }

    if has_flashed.iter().all(|row| row.iter().all(|cell| *cell)) {
      break;
    }
  }

  step
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn a_eg_1() {
    assert_eq!(solve_a(get_sample_input()), 1656);
  }

  #[test]
  fn a() {
    assert_eq!(solve_a(get_input()), 1739);
  }

  #[test]
  fn b_eg_1() {
    assert_eq!(solve_b(get_sample_input()), 195);
  }

  #[test]
  fn b() {
    assert_eq!(solve_b(get_input()), 324);
  }

  //
}
