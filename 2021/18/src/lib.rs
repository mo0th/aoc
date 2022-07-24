pub fn get_input() -> String {
  String::from(include_str!("../input"))
}

pub fn get_sample_input() -> String {
  String::from(include_str!("../sample"))
}

#[derive(Debug, Clone)]
enum Number {
  Literal(u64),
  Pair(Box<Number>, Box<Number>),
}

impl Number {
  fn parse(input: &Vec<char>, idx: &mut usize) -> Number {
    match input[*idx] {
      '[' => {
        *idx += 1;
        let left = Number::parse(input, idx);
        *idx += 1;
        let right = Number::parse(input, idx);
        *idx += 1;
        Number::Pair(Box::new(left), Box::new(right))
      }
      d => {
        *idx += 1;
        Number::Literal(d.to_digit(10).unwrap().into())
      }
    }
  }

  fn magnitude(&self) -> i64 {
    match self {
      Number::Literal(n) => *n as i64,
      Number::Pair(a, b) => a.magnitude() * 3 + b.magnitude() * 2,
    }
  }

  fn to_val(&self) -> u64 {
    match self {
      Number::Literal(n) => *n,
      _ => panic!(),
    }
  }

  fn reduce(self) -> Number {
    let mut result = self;

    result
  }

  fn split(&self, has_split: &mut bool) -> Number {
    match self {
      Number::Literal(n) if *n > 9 => {
        *has_split = true;

        Number::Pair(
          Box::new(Number::Literal(n / 2)),
          Box::new(Number::Literal(((*n as f64) / 2.0).ceil() as u64)),
        )
      }
      Number::Pair(left, right) if !*has_split => {
        let maybe_new_left = left.split(has_split);

        if *has_split {
          Number::Pair(Box::new(maybe_new_left), *right)
        } else {
          Number::Pair(*left, Box::new(right.split(has_split)))
        }
      }
      n => *n,
    }
  }
}

pub fn solve_a(input: String) -> i64 {
  let numbers = input
    .trim()
    .lines()
    .map(|line| Number::parse(&line.chars().collect(), &mut 0).reduce())
    .collect::<Vec<_>>();

  for n in numbers {
    println!("{:?}", n);
  }

  0
}

pub fn solve_b(input: String) -> i64 {
  0
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn a_eg_1() {
    assert_eq!(solve_a(get_sample_input()), 4140);
  }

  // #[test]
  // fn a() {
  //   assert_eq!(solve_a(get_input()), 0);
  // }

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
