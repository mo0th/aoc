use std::collections::HashMap;
use std::num::ParseIntError;
use std::str::FromStr;

pub fn get_input() -> String {
  String::from(include_str!("../input"))
}

pub fn get_sample_input() -> String {
  String::from(include_str!("../sample"))
}

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
enum Operand {
  W,
  X,
  Y,
  Z,
  Literal(i64),
}

impl FromStr for Operand {
  type Err = ParseIntError;

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    match s {
      "w" => Ok(Self::W),
      "x" => Ok(Self::X),
      "y" => Ok(Self::Y),
      "z" => Ok(Self::Z),
      n => Ok(Self::Literal(n.parse::<i64>()?)),
    }
  }
}

impl Operand {
  fn to_reg_idx(&self) -> usize {
    match self {
      Self::W => 0,
      Self::X => 1,
      Self::Y => 2,
      Self::Z => 3,
      _ => panic!(),
    }
  }

  fn to_val(&self, regs: &Registers) -> i64 {
    match self {
      Operand::Literal(v) => *v,
      op => regs[op.to_reg_idx()],
    }
  }
}

#[derive(Debug, Eq, PartialEq)]
enum Instruction {
  Inp(Operand),
  Add(Operand, Operand),
  Mul(Operand, Operand),
  Div(Operand, Operand),
  Mod(Operand, Operand),
  Eql(Operand, Operand),
}

impl Instruction {
  fn eval(&self, regs: &mut Registers, input: Option<i64>) {
    match self {
      Instruction::Inp(o) => regs[o.to_reg_idx()] = input.unwrap(),
      Instruction::Add(a, b) => regs[a.to_reg_idx()] += b.to_val(regs),
      Instruction::Mul(a, b) => regs[a.to_reg_idx()] *= b.to_val(regs),
      Instruction::Div(a, b) => regs[a.to_reg_idx()] /= b.to_val(regs),
      Instruction::Mod(a, b) => regs[a.to_reg_idx()] %= b.to_val(regs),
      Instruction::Eql(a, b) => {
        regs[a.to_reg_idx()] = if a.to_val(regs) == b.to_val(regs) {
          1
        } else {
          0
        }
      }
    }
  }
}

type Registers = [i64; 4];

fn parse(input: &str) -> Vec<Instruction> {
  input
    .lines()
    .map(|line| {
      let parts = line.split(" ").collect::<Vec<_>>();

      if parts.len() == 2 {
        Instruction::Inp(parts[1].parse::<Operand>().unwrap())
      } else {
        let operands = parts
          .iter()
          .skip(1)
          .map(|p| p.parse::<Operand>().unwrap())
          .collect::<Vec<_>>();

        match parts[0] {
          "add" => Instruction::Add(operands[0], operands[1]),
          "mul" => Instruction::Mul(operands[0], operands[1]),
          "div" => Instruction::Div(operands[0], operands[1]),
          "mod" => Instruction::Mod(operands[0], operands[1]),
          "eql" => Instruction::Eql(operands[0], operands[1]),
          _ => panic!("input borked"),
        }
      }
    })
    .collect()
}

fn dfs(
  instructions: &Vec<Instruction>,
  i: usize,
  regs: Registers,
  cache: &mut HashMap<(Registers, usize), Option<i64>>,
  smallest: bool,
) -> Option<i64> {
  // println!("{:?}", (&instructions[i], i, regs, cache.len(), smallest));
  if let Some(ans) = cache.get(&(regs, i)) {
    return *ans;
  }

  let inputs: Vec<_> = if smallest {
    (1..=9).collect()
  } else {
    (1..=9).rev().collect()
  };

  'input_loop: for input in inputs {
    let mut regs = regs;
    let mut i = i;
    instructions[i].eval(&mut regs, Some(input));
    i += 1;

    while let Some(instruction) = instructions.get(i) {
      match instruction {
        Instruction::Inp(_) => {
          if let Some(num) = dfs(instructions, i, regs, cache, smallest) {
            let result = Some(num * 10 + input);
            cache.insert((regs, i), result);
            return result;
          } else {
            continue 'input_loop;
          }
        }
        instruction => {
          instruction.eval(&mut regs, None);
          i += 1;
        }
      }
    }

    if regs[3] == 0 {
      let result = Some(input);
      cache.insert((regs, i), result);
      return result;
    }
  }

  cache.insert((regs, i), None);
  None
}

pub fn solve_a(input: String) -> i64 {
  let instructions = parse(&input);

  let cache = &mut HashMap::new();

  format!("{}", dfs(&instructions, 0, [0; 4], cache, false).unwrap())
    .chars()
    .rev()
    .collect::<String>()
    .parse::<i64>()
    .unwrap()
}

pub fn solve_b(input: String) -> i64 {
  let instructions = parse(&input);

  let cache = &mut HashMap::new();

  format!("{}", dfs(&instructions, 0, [0; 4], cache, true).unwrap())
    .chars()
    .rev()
    .collect::<String>()
    .parse::<i64>()
    .unwrap()
}

#[cfg(test)]
mod tests {
  use super::*;

  // #[test]
  // fn a() {
  //   assert_eq!(solve_a(get_input()), 93997999296912);
  // }

  #[test]
  fn b() {
    assert_eq!(solve_b(get_input()), 0);
  }

  //
}
