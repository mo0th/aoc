pub fn get_input() -> String {
  String::from(include_str!("../input"))
}

pub fn get_sample_input() -> String {
  String::from(include_str!("../sample"))
}

pub fn solve_a(input: String) -> i64 {
  input
    .lines()
    .map(|line| {
      let parts: Vec<_> = line.split(" | ").collect();

      parts[1].split(" ").map(|segments| segments.len())
    })
    .flatten()
    .filter(|&x| x == 2 || x == 4 || x == 3 || x == 7)
    .count() as i64
}

type Section = [i64; 7];
type SignalTranslation = [i64; 7];

fn parse_section(input: &str) -> Section {
  let mut output = [0; 7];
  for ch in input.chars() {
    output[ch as usize - 'a' as usize] += 1;
  }

  output
}

fn num_on_segments(segments: &Section) -> i64 {
  segments.iter().filter(|&&x| x > 0).count() as i64
}

fn sum_sections(sections: &[Section]) -> Section {
  let mut output = [0; 7];
  for section in sections {
    for i in 0..7 {
      output[i] += section[i];
    }
  }

  output
}

fn mark_for_solution(options: &mut [Section; 7], in_bit: i64, out_bit: i64) {
  for (curr_in_bit, out_options) in options.iter_mut().enumerate() {
    if curr_in_bit as i64 != in_bit {
      out_options[out_bit as usize] = 0;
    } else {
      for (curr_out_bit, is_possible) in out_options.iter_mut().enumerate() {
        if curr_out_bit as i64 != out_bit {
          *is_possible = 0;
        }
      }
    }
  }
}

fn mark_possible(options: &mut [Section; 7], possible_in: &[usize], possible_out: &[usize]) {
  for (in_bit, out_options) in options.iter_mut().enumerate() {
    if !possible_in.contains(&in_bit) {
      for i in possible_out {
        out_options[*i] = 0;
      }
    } else {
      for (i, out_bit) in out_options.iter_mut().enumerate() {
        if !possible_out.contains(&i) {
          *out_bit = 0;
        }
      }
    }
  }
}

fn get_translation_array(signals: &Vec<Section>) -> SignalTranslation {
  let mut options: [Section; 7] = [[1; 7], [1; 7], [1; 7], [1; 7], [1; 7], [1; 7], [1; 7]];

  let signal_2 = signals[0];
  let signal_3 = signals[1];
  let signal_4 = signals[2];

  // These segements could be 2/c or 5/d
  let in_bits_for_2_5: Vec<_> = signal_2
    .iter()
    .enumerate()
    .filter(|(_, &b)| b != 0)
    .map(|(i, _)| i)
    .collect();

  mark_possible(&mut options, &in_bits_for_2_5, &[2, 5]);

  // This is the 0/a segment in the output
  let diff_2_3 = signal_2
    .iter()
    .zip(signal_3.iter())
    .enumerate()
    .filter(|&(_, (a, b))| a != b)
    .last()
    .unwrap()
    .0;

  mark_for_solution(&mut options, diff_2_3 as i64, 0);

  // These segments could be 1/b or 3/d
  let diff_2_4: Vec<_> = signal_2
    .iter()
    .zip(signal_4.iter())
    .enumerate()
    .filter(|&(_, (a, b))| a != b)
    .map(|(i, _)| i)
    .collect();

  mark_possible(&mut options, &diff_2_4, &[1, 3]);

  let in_bit_for_1 = sum_sections(signals)
    .iter()
    .enumerate()
    .find(|(_, &b)| b == 6)
    .unwrap()
    .0;

  mark_for_solution(&mut options, in_bit_for_1 as i64, 1);

  let in_bit_for_4 = sum_sections(signals)
    .iter()
    .enumerate()
    .find(|(_, &b)| b == 4)
    .unwrap()
    .0;

  mark_for_solution(&mut options, in_bit_for_4 as i64, 4);

  let in_bit_for_5 = sum_sections(signals)
    .iter()
    .enumerate()
    .find(|(_, &b)| b == 9)
    .unwrap()
    .0;

  mark_for_solution(&mut options, in_bit_for_5 as i64, 5);

  let mut output = [0; 7];

  for (i, row) in options.iter().enumerate() {
    output[i] = row.iter().enumerate().find(|(_, &b)| b == 1).unwrap().0 as i64;
  }

  output
}

fn apply_translation(translation: SignalTranslation, signal: Section) -> Section {
  let mut output: Section = [0; 7];

  for (i, &on) in signal.iter().enumerate() {
    output[translation[i] as usize] = on;
  }

  output
}

fn get_num(signals: &Vec<Section>) -> i64 {
  let mut num = 0;
  for signal in signals {
    num *= 10;
    num += match signal {
      [1, 1, 1, 0, 1, 1, 1] => 0, // 0
      [0, 0, 1, 0, 0, 1, 0] => 1, // 1
      [1, 0, 1, 1, 1, 0, 1] => 2, // 2
      [1, 0, 1, 1, 0, 1, 1] => 3, // 3
      [0, 1, 1, 1, 0, 1, 0] => 4, // 4
      [1, 1, 0, 1, 0, 1, 1] => 5, // 5
      [1, 1, 0, 1, 1, 1, 1] => 6, // 6
      [1, 0, 1, 0, 0, 1, 0] => 7, // 7
      [1, 1, 1, 1, 1, 1, 1] => 8, // 8
      [1, 1, 1, 1, 0, 1, 1] => 9, // 9
      _ => 0,
    }
  }

  num
}

pub fn solve_b(input: String) -> i64 {
  let input = input
    .lines()
    .map(|line| {
      let parts = line.split(" | ").collect::<Vec<_>>();

      let mut signals = parts[0]
        .split(" ")
        .map(|s| parse_section(s))
        .collect::<Vec<_>>();

      signals.sort_by_key(|s| s.iter().filter(|&e| e > &0).count());

      let lit_segments = parts[1]
        .split(" ")
        .map(|s| parse_section(s))
        .collect::<Vec<_>>();

      (signals, lit_segments)
    })
    .collect::<Vec<_>>();

  let mut result = 0;

  for (signals, lit_segments) in input {
    result += get_num(
      &lit_segments
        .iter()
        .map(|s| apply_translation(get_translation_array(&signals), *s))
        .collect::<Vec<_>>(),
    );
  }

  result
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn a_eg_1() {
    assert_eq!(solve_a(get_sample_input()), 26);
  }

  #[test]
  fn a() {
    assert_eq!(solve_a(get_input()), 367);
  }

  #[test]
  fn b_eg_1() {
    assert_eq!(solve_b(get_sample_input()), 61229);
  }

  #[test]
  fn b_eg_2() {
    assert_eq!(
      solve_b(String::from(
        "acedgfb cdfbe gcdfa fbcad dab cefabd cdfgeb eafb cagedb ab | cdfeb fcadb cdfeb cdbaf"
      )),
      5353
    );
  }

  #[test]
  fn b() {
    assert_eq!(solve_b(get_input()), 974512);
  }
}
