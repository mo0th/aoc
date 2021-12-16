use std::error::Error;
use std::fmt::Debug;
use std::process;

pub fn get_input() -> String {
  String::from(include_str!("../input"))
}

pub fn get_sample_input() -> String {
  String::from(include_str!("../sample"))
}

#[derive(Debug)]
struct PacketHeader {
  t: u64,
  v: u64,
}

#[derive(Debug)]
enum Packet {
  Literal(PacketHeader, u64),
  Operator(PacketHeader, Operation, Vec<Packet>),
}

impl Packet {
  fn evaluate(&self) -> u64 {
    match self {
      Packet::Literal(_, v) => *v,
      Packet::Operator(_, op, sub_packets) => match op {
        Operation::Sum => sub_packets.iter().map(|p| p.evaluate()).sum(),
        Operation::Product => sub_packets.iter().map(|p| p.evaluate()).product(),
        Operation::Minimum => sub_packets.iter().map(|p| p.evaluate()).min().unwrap(),
        Operation::Maximum => sub_packets.iter().map(|p| p.evaluate()).max().unwrap(),
        Operation::GreaterThan => {
          if sub_packets[0].evaluate() > sub_packets[1].evaluate() {
            1
          } else {
            0
          }
        }
        Operation::LessThan => {
          if sub_packets[0].evaluate() < sub_packets[1].evaluate() {
            1
          } else {
            0
          }
        }
        Operation::EqualTo => {
          if sub_packets[0].evaluate() == sub_packets[1].evaluate() {
            1
          } else {
            0
          }
        }

        Operation::None => 0,
      },
    }
  }
}

#[derive(Debug)]
enum Operation {
  Sum,
  Product,
  Minimum,
  Maximum,
  GreaterThan,
  LessThan,
  EqualTo,

  None,
}

impl Operation {
  fn from(type_id: u64) -> Operation {
    match type_id {
      0 => Operation::Sum,
      1 => Operation::Product,
      2 => Operation::Minimum,
      3 => Operation::Maximum,
      5 => Operation::GreaterThan,
      6 => Operation::LessThan,
      7 => Operation::EqualTo,
      _ => Operation::None,
    }
  }
}

fn sum_versions(packets: &Vec<Packet>) -> u64 {
  packets
    .iter()
    .map(|p| match p {
      Packet::Literal(h, _) => h.v,
      Packet::Operator(h, _, inner) => h.v + sum_versions(&inner),
    })
    .sum()
}

fn get_version(bits: &Vec<char>, i: &mut usize) -> u64 {
  const SIZE: usize = 3;
  let version_bits = bits.get(*i..*i + SIZE).unwrap().iter().collect::<String>();
  *i += SIZE;
  u64::from_str_radix(version_bits.as_str(), 2).unwrap()
}

fn get_type_id(bits: &Vec<char>, i: &mut usize) -> u64 {
  const SIZE: usize = 3;
  let type_id_bits = bits.get(*i..*i + SIZE).unwrap().iter().collect::<String>();
  *i += SIZE;
  u64::from_str_radix(type_id_bits.as_str(), 2).unwrap()
}

fn get_length_type_id(bits: &Vec<char>, i: &mut usize) -> u64 {
  const SIZE: usize = 1;
  let type_id_bits = bits.get(*i..*i + SIZE).unwrap().iter().collect::<String>();
  *i += SIZE;
  u64::from_str_radix(type_id_bits.as_str(), 2).unwrap()
}

fn get_literal_bits(bits: &Vec<char>, i: &mut usize) -> (bool, Vec<char>) {
  const CHUNK_SIZE: usize = 4;
  let is_end_bit = *bits.get(*i).unwrap();
  *i += 1;
  let literal_bits = bits
    .get(*i..*i + CHUNK_SIZE)
    .unwrap()
    .iter()
    .map(|c| *c)
    .collect::<Vec<_>>();
  *i += CHUNK_SIZE;

  (is_end_bit == '0', literal_bits)
}

fn get_range_value(bits: &Vec<char>, i: &mut usize, n_bits: usize) -> u64 {
  let value = u64::from_str_radix(
    bits
      .get(*i..*i + n_bits)
      .unwrap()
      .iter()
      .collect::<String>()
      .as_str(),
    2,
  )
  .unwrap();

  *i += n_bits;

  value
}

fn get_packet(bits: &Vec<char>, i: &mut usize) -> Result<Packet, Box<dyn Error>> {
  let version = get_version(&bits, i);
  let type_id = get_type_id(&bits, i);

  let header = PacketHeader {
    v: version,
    t: type_id,
  };

  match type_id {
    4 => {
      let (mut is_last_chunk, mut literal_bits) = (false, vec![]);

      while !is_last_chunk {
        let result = get_literal_bits(&bits, i);
        is_last_chunk = result.0;
        literal_bits.extend(result.1)
      }

      Ok(Packet::Literal(
        header,
        u64::from_str_radix(literal_bits.iter().collect::<String>().as_str(), 2).unwrap(),
      ))
    }
    _ => {
      let length_type_id_bit = get_length_type_id(&bits, i);
      let mut sub_packets = vec![];

      match length_type_id_bit {
        0 => {
          let sub_packets_len = get_range_value(&bits, i, 15);

          let result_i = *i + sub_packets_len as usize;

          while *i < result_i {
            if let Ok(packet) = get_packet(&bits, i) {
              sub_packets.push(packet)
            }
          }
        }
        1 => {
          let n_sub_packets = get_range_value(&bits, i, 11);

          for _ in 0..n_sub_packets {
            if let Ok(packet) = get_packet(&bits, i) {
              sub_packets.push(packet)
            }
          }
        }
        _ => process::exit(1),
      }

      Ok(Packet::Operator(
        header,
        Operation::from(type_id),
        sub_packets,
      ))
    }
  }
}

pub fn solve_a(input: String) -> i64 {
  let bits = input
    .chars()
    .flat_map(|c| {
      let as_decimal: u64 = u64::from_str_radix(c.to_string().as_str(), 16).unwrap();
      format!("{:0>4b}", as_decimal).chars().collect::<Vec<_>>()
    })
    .collect::<Vec<char>>();

  let mut packets: Vec<Packet> = vec![];

  let n_bits = bits.len();
  let mut i = 0;

  while i < n_bits - 6 {
    if let Ok(packet) = get_packet(&bits, &mut i) {
      packets.push(packet)
    }
  }

  sum_versions(&packets) as i64
}

pub fn solve_b(input: String) -> i64 {
  let bits = input
    .chars()
    .flat_map(|c| {
      let as_decimal: u64 = u64::from_str_radix(c.to_string().as_str(), 16).unwrap();
      format!("{:0>4b}", as_decimal).chars().collect::<Vec<_>>()
    })
    .collect::<Vec<char>>();

  let mut i = 0;

  if let Ok(packet) = get_packet(&bits, &mut i) {
    packet.evaluate() as i64
  } else {
    0
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn a_eg_1() {
    assert_eq!(solve_a(get_sample_input()), 16);
  }

  #[test]
  fn a_eg_2() {
    assert_eq!(solve_a(String::from("D2FE28")), 6);
  }

  #[test]
  fn a() {
    assert_eq!(solve_a(get_input()), 927);
  }

  #[test]
  fn b() {
    assert_eq!(solve_b(get_input()), 1725277876501);
  }

  //
}
