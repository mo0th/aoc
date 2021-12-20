use std::collections::HashMap;

pub fn get_input() -> String {
  String::from(include_str!("../input"))
}

pub fn get_sample_input() -> String {
  String::from(include_str!("../sample"))
}

type ImageData = HashMap<(i64, i64), bool>;
type Algorithm = Vec<bool>;

#[derive(Debug)]
struct Image {
  data: ImageData,
  inf_pixel_light: bool,
}

impl Image {
  fn new(data: ImageData) -> Image {
    Image {
      data,
      inf_pixel_light: false,
    }
  }

  fn enhance(&self, algorithm: &Algorithm) -> Image {
    const BUFFER: i64 = 1;
    let mut result_data = ImageData::new();

    let data = self.data.clone();

    let min_x = data.iter().map(|((x, _), _)| x).min().unwrap() - BUFFER;
    let max_x = data.iter().map(|((x, _), _)| x).max().unwrap() + BUFFER;
    let min_y = data.iter().map(|((_, y), _)| y).min().unwrap() - BUFFER;
    let max_y = data.iter().map(|((_, y), _)| y).max().unwrap() + BUFFER;

    for i in min_x..=max_x {
      for j in min_y..=max_y {
        let mut bit_string = String::new();

        for di in -1..=1 {
          for dj in -1..=1 {
            let pos = (i + di, j + dj);

            let is_light = match data.get(&pos) {
              Some(b) => *b,
              None => self.inf_pixel_light,
            };

            // println!("{:?}", (pos, is_light));

            bit_string.push(if is_light { '1' } else { '0' })
          }
        }

        let n = usize::from_str_radix(&bit_string, 2).unwrap();

        // println!("{:?}", ((i, j), bit_string, n, algorithm[n]));

        result_data.insert((i, j), algorithm[n]);
      }
    }

    let mut result = Image::new(result_data);
    if self.inf_pixel_light && !algorithm[511] {
      result.inf_pixel_light = false;
    } else if !self.inf_pixel_light && algorithm[0] {
      result.inf_pixel_light = true;
    };

    result
  }

  fn get_n_light(&self) -> i64 {
    self.data.iter().filter(|(_, l)| **l).count() as i64
  }
}

fn parse(input: &str) -> (Algorithm, Image) {
  let input_parts = input.split("\n\n").collect::<Vec<_>>();
  let algorithm = input_parts[0]
    .chars()
    .map(|c| match c {
      '#' => true,
      '.' => false,
      c => panic!("invalid algorithm char '{}'", c),
    })
    .collect();

  let image_data = input_parts[1]
    .lines()
    .enumerate()
    .flat_map(|(i, line)| {
      line.chars().enumerate().map(move |(j, c)| match c {
        '#' => ((i as i64, j as i64), true),
        '.' => ((i as i64, j as i64), false),
        c => panic!("invalid image char '{}'", c),
      })
    })
    .collect();

  (algorithm, Image::new(image_data))
}

pub fn solve_a(input: String) -> i64 {
  let (algorithm, initial_image) = parse(&input);
  let enhanced_1 = initial_image.enhance(&algorithm);
  let enhanced_2 = enhanced_1.enhance(&algorithm);
  return enhanced_2.get_n_light();
}

pub fn solve_b(input: String) -> i64 {
  let (algorithm, mut image) = parse(&input);

  for _ in 0..50 {
    image = image.enhance(&algorithm);
  }

  image.get_n_light()
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn a_eg_1() {
    assert_eq!(solve_a(get_sample_input()), 35);
  }

  #[test]
  fn a() {
    assert_eq!(solve_a(get_input()), 5571);
  }

  #[test]
  fn b_eg_1() {
    assert_eq!(solve_b(get_sample_input()), 3351);
  }

  #[test]
  fn b() {
    assert_eq!(solve_b(get_input()), 17965);
  }

  //
}
