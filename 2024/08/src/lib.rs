use std::collections::{HashMap, HashSet};

use itertools::Itertools;

pub fn get_input() -> String {
    String::from(include_str!("../input"))
}

pub fn get_sample_input() -> String {
    String::from(include_str!("../sample"))
}

type Pos = (i64, i64);

fn pos_diff(p1: &Pos, p2: &Pos) -> Pos {
    return (p1.0 - p2.0, p1.1 - p2.1);
}

fn pos_add(p1: &Pos, p2: &Pos) -> Pos {
    return (p1.0 + p2.0, p1.1 + p2.1);
}

fn is_in_limits(p: &Pos, limits: &Pos) -> bool {
    return p.0 >= 0 && p.0 <= limits.0 && p.1 >= 0 && p.1 <= limits.1;
}

fn get_antinodes_a(nodes: &Vec<Pos>, limits: &Pos) -> Vec<Pos> {
    let mut antinodes: HashSet<Pos> = HashSet::new();

    for pos1 in nodes.iter() {
        for pos2 in nodes.iter() {
            if pos1 == pos2 {
                continue;
            }

            let diff = pos_diff(pos1, pos2);
            let maybe_antinode = pos_add(pos1, &diff);

            if !is_in_limits(&maybe_antinode, limits) {
                continue;
            }

            antinodes.insert(maybe_antinode);
        }
    }

    return antinodes
        .into_iter()
        .filter_map(|p| return if nodes.contains(&p) { None } else { Some(p) })
        .collect();
}

pub fn solve_a(input: String) -> i64 {
    let limits = (
        input.lines().count() as i64 - 1,
        input.lines().next().unwrap().len() as i64 - 1,
    );

    let node_sets: Vec<(char, Pos)> = input
        .lines()
        .enumerate()
        .flat_map(|(row, line)| {
            return line.char_indices().filter_map(move |(col, ch)| {
                if ch == '.' {
                    None
                } else {
                    Some((ch, (row as i64, col as i64)))
                }
            });
        })
        .collect();

    let mut map: HashMap<char, Vec<Pos>> = HashMap::new();
    for (ch, pos) in node_sets {
        map.entry(ch).or_default().push(pos);
    }

    let antinodes_set = map
        .iter()
        .flat_map(|(ch, v)| {
            return get_antinodes_a(v, &limits);
        })
        .collect::<HashSet<_>>();

    return antinodes_set.len() as i64;
}

fn get_antinodes_b(nodes: &Vec<Pos>, limits: &Pos) -> HashSet<Pos> {
    return nodes
        .iter()
        .flat_map(|pos1| {
            return nodes
                .iter()
                .filter_map(move |pos2| {
                    if pos1 == pos2 {
                        return None;
                    }

                    let diff = pos_diff(pos1, pos2);

                    let mut maybe_antinode = *pos2;
                    let mut antinodes = vec![];

                    loop {
                        maybe_antinode = pos_add(&maybe_antinode, &diff);
                        if is_in_limits(&maybe_antinode, limits) {
                            antinodes.push(maybe_antinode);
                        } else {
                            break;
                        }
                    }

                    return Some(antinodes);
                })
                .flatten();
        })
        .collect();
}

pub fn solve_b(input: String) -> i64 {
    let limits = (
        input.lines().count() as i64 - 1,
        input.lines().next().unwrap().len() as i64 - 1,
    );

    let node_sets: Vec<(char, Pos)> = input
        .lines()
        .enumerate()
        .flat_map(|(row, line)| {
            return line.char_indices().filter_map(move |(col, ch)| {
                if ch == '.' {
                    None
                } else {
                    Some((ch, (row as i64, col as i64)))
                }
            });
        })
        .collect();

    let mut map: HashMap<char, Vec<Pos>> = HashMap::new();
    for (ch, pos) in node_sets {
        map.entry(ch).or_default().push(pos);
    }

    return map
        .iter()
        .flat_map(|(_, v)| {
            return get_antinodes_b(v, &limits);
        })
        .unique()
        .count() as i64;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_eg_1() {
        assert_eq!(solve_a(get_sample_input()), 14);
    }

    #[test]
    fn a() {
        assert_eq!(solve_a(get_input()), 394);
    }

    #[test]
    fn b_eg_1() {
        assert_eq!(solve_b(get_sample_input()), 34);
    }

    #[test]
    fn b() {
        assert_eq!(solve_b(get_input()), 1277);
    }

    //
}
