use std::collections::HashMap;

pub fn get_input() -> String {
  String::from(include_str!("../input"))
}

pub fn get_sample_input() -> String {
  String::from(include_str!("../sample"))
}

#[derive(Debug)]
struct DeterministicDie {
  next_num: i64,
  rolls: i64,
}

impl DeterministicDie {
  fn new() -> Self {
    Self {
      next_num: 1,
      rolls: 0,
    }
  }

  fn roll(&mut self) -> i64 {
    let ret = self.next_num;

    self.next_num += 1;

    if self.next_num > 100 {
      self.next_num -= 100;
    }

    self.rolls += 1;

    ret
  }

  fn roll_n(&mut self, n: usize) -> i64 {
    (0..n).map(|_| self.roll()).sum()
  }
}

#[derive(Debug)]
struct DeterministicPlayer {
  pos: i64,
  score: i64,
}

impl DeterministicPlayer {
  fn new(starting_pos: i64) -> Self {
    Self {
      pos: starting_pos,
      score: 0,
    }
  }

  fn handle_roll(&mut self, roll: i64) {
    self.pos += roll % 10;

    if self.pos > 10 {
      self.pos -= 10;
    }

    self.score += self.pos;
  }

  fn has_won(&self) -> bool {
    self.score >= 1000
  }
}

pub fn solve_a(input: String) -> i64 {
  let mut die = DeterministicDie::new();

  let mut players = input
    .lines()
    .map(|line| {
      DeterministicPlayer::new(
        line
          .chars()
          .last()
          .unwrap()
          .to_string()
          .parse::<i64>()
          .unwrap(),
      )
    })
    .collect::<Vec<_>>();

  let mut player_idx = 0;

  while !players.iter().any(|p| p.has_won()) {
    players[player_idx].handle_roll(die.roll_n(3));

    player_idx = (player_idx + 1) % 2;
  }

  let loser = players.iter().filter(|p| !p.has_won()).last().unwrap();

  loser.score * die.rolls
}

#[derive(Debug, Eq, PartialEq, Hash)]
struct GameState {
  p1: (u8, u8),
  p2: (u8, u8),
}

fn weird_mod(n: u8, m: u8) -> u8 {
  if n > m {
    n - m
  } else {
    n
  }
}

impl GameState {
  fn get_winner(&self) -> Option<u8> {
    let GameState {
      p1: (_, s1),
      p2: (_, s2),
    } = self;

    if *s1 > 20 {
      Some(0)
    } else if *s2 > 20 {
      Some(1)
    } else {
      None
    }
  }

  fn split(&self, is_p1: bool) -> Vec<GameState> {
    if is_p1 {
      let (pos, score) = self.p1;

      (1..=3)
        .map(|i| GameState {
          p1: (pos + i, score),
          ..*self
        })
        .collect()
    } else {
      let (pos, score) = self.p2;

      (1..=3)
        .map(|i| GameState {
          p2: (pos + i, score),
          ..*self
        })
        .collect()
    }
  }

  fn resolve_turn(&self, is_p1: bool) -> Vec<GameState> {
    self
      .split(is_p1)
      .iter()
      .flat_map(|s| s.split(is_p1))
      .flat_map(|s| s.split(is_p1))
      .map(|s| {
        if is_p1 {
          let (pos, score) = s.p1;
          let modded = weird_mod(pos, 10);
          GameState {
            p1: (modded, score + modded),
            ..s
          }
        } else {
          let (pos, score) = s.p2;
          let modded = weird_mod(pos, 10);
          GameState {
            p2: (modded, score + modded),
            ..s
          }
        }
      })
      .collect::<Vec<_>>()
  }
}

pub fn solve_b(input: String) -> u64 {
  let start_positions = input
    .lines()
    .map(|line| {
      line
        .chars()
        .last()
        .unwrap()
        .to_string()
        .parse::<u8>()
        .unwrap()
    })
    .collect::<Vec<_>>();

  let mut states = HashMap::<GameState, u64>::new();

  states.insert(
    GameState {
      p1: (start_positions[0], 0),
      p2: (start_positions[1], 0),
    },
    1,
  );

  let mut is_p1 = true;
  let mut wins = [0, 0];

  while !states.is_empty() {
    let mut next_states = HashMap::new();

    for (state, count) in states.iter() {
      let new_states = state.resolve_turn(is_p1);

      for s in new_states {
        match s.get_winner() {
          Some(winner) => wins[winner as usize] += count,
          None => *next_states.entry(s).or_insert(0) += count,
        }
      }
    }

    states = next_states;
    is_p1 = !is_p1;
  }

  *wins.iter().max().unwrap()
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn a_eg_1() {
    assert_eq!(solve_a(get_sample_input()), 739785);
  }

  #[test]
  fn a() {
    assert_eq!(solve_a(get_input()), 1004670);
  }

  #[test]
  fn b_eg_1() {
    assert_eq!(solve_b(get_sample_input()), 444356092776315);
  }

  #[test]
  fn b() {
    assert_eq!(solve_b(get_input()), 492043106122795);
  }

  //
}
