use std::collections::{HashMap, HashSet, VecDeque};

pub fn get_input() -> String {
  String::from(include_str!("../input"))
}

pub fn get_sample_input() -> String {
  String::from(include_str!("../sample"))
}

fn is_small_case(cave: &String) -> bool {
  *cave == cave.to_lowercase()
}

pub fn print_graph(graph: &HashMap<String, HashSet<String>>) {
  for (k, v) in graph.iter() {
    println!("{} -> {:?}", k, v);
  }
}

fn parse(input: String) -> HashMap<String, HashSet<String>> {
  let mut graph = HashMap::new();

  for line in input.lines() {
    let parts = line.split('-').collect::<Vec<_>>();
    let src = parts[0].to_string();
    let dst = parts[1].to_string();

    graph
      .entry(src.clone())
      .or_insert(HashSet::new())
      .insert(dst.clone());
    graph.entry(dst).or_insert(HashSet::new()).insert(src);
  }

  graph
}

pub fn solve_a(input: String) -> i64 {
  let graph = parse(input);

  let mut queue = VecDeque::new();
  let mut count = 0;

  let start: String = String::from("start");
  let end: String = String::from("end");

  queue.push_back(vec![start]);

  while !queue.is_empty() {
    let current = queue.pop_front().unwrap();
    let last_in_current = current.last().unwrap();

    if last_in_current == &end {
      count += 1;
      continue;
    }

    let connected_caves = graph.get(last_in_current).unwrap();

    for next in connected_caves.iter() {
      if is_small_case(next) && current.contains(next) {
        continue;
      }

      let mut next_path = current.clone();
      next_path.push(next.clone());
      queue.push_back(next_path);
    }
  }

  count
}

pub fn solve_b(input: String) -> i64 {
  let graph = parse(input);

  let mut queue = VecDeque::new();
  let mut count = 0;

  let start: String = String::from("start");
  let end: String = String::from("end");

  queue.push_back(vec![start]);

  while !queue.is_empty() {
    let current = queue.pop_front().unwrap();
    let last_in_current = current.last().unwrap();

    if last_in_current == &end {
      count += 1;
      continue;
    }

    let connected_caves = graph.get(last_in_current).unwrap();

    for next in connected_caves.iter() {
      if next == &"start".to_string() {
        continue;
      }

      let small_cave_count = current.iter().filter(|c| is_small_case(c)).count();
      let unique_small_cave_count = current
        .iter()
        .filter(|c| is_small_case(c))
        .collect::<HashSet<_>>()
        .len();

      if is_small_case(next) && current.contains(next) && small_cave_count > unique_small_cave_count
      {
        continue;
      }

      let mut next_path = current.clone();
      next_path.push(next.clone());
      queue.push_back(next_path);
    }
  }

  count
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn a_eg_1() {
    assert_eq!(solve_a(get_sample_input()), 10);
  }

  #[test]
  fn a_eg_2() {
    assert_eq!(
      solve_a(String::from(
        "dc-end
HN-start
start-kj
dc-start
dc-HN
LN-dc
HN-end
kj-sa
kj-HN
kj-dc"
      )),
      19
    );
  }

  #[test]
  fn a_eg_3() {
    assert_eq!(
      solve_a(String::from(
        "fs-end
he-DX
fs-he
start-DX
pj-DX
end-zg
zg-sl
zg-pj
pj-he
RW-he
fs-DX
pj-RW
zg-RW
start-pj
he-WI
zg-he
pj-fs
start-RW"
      )),
      226
    );
  }

  #[test]
  fn a() {
    assert_eq!(solve_a(get_input()), 3230);
  }

  #[test]
  fn b_eg_1() {
    assert_eq!(solve_b(get_sample_input()), 36);
  }

  #[test]
  fn b_eg_2() {
    assert_eq!(
      solve_b(String::from(
        "dc-end
HN-start
start-kj
dc-start
dc-HN
LN-dc
HN-end
kj-sa
kj-HN
kj-dc"
      )),
      103
    );
  }

  #[test]
  fn b_eg_3() {
    assert_eq!(
      solve_b(String::from(
        "fs-end
he-DX
fs-he
start-DX
pj-DX
end-zg
zg-sl
zg-pj
pj-he
RW-he
fs-DX
pj-RW
zg-RW
start-pj
he-WI
zg-he
pj-fs
start-RW"
      )),
      3509
    );
  }

  #[test]
  fn b() {
    assert_eq!(solve_b(get_input()), 83475);
  }

  //
}
