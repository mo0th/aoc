use std::{collections::HashMap, str::FromStr};

pub fn get_input() -> String {
    String::from(include_str!("../input"))
}

pub fn get_sample_input() -> String {
    String::from(include_str!("../sample"))
}

#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy)]
enum Color {
    Red,
    Green,
    Blue,
}

impl FromStr for Color {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use Color::*;
        Ok(match s {
            "red" => Red,
            "green" => Green,
            "blue" => Blue,
            _ => {
                return Err("Parsing failed");
            }
        })
    }
}

type CubeSet = HashMap<Color, usize>;

#[derive(Debug, PartialEq, Eq)]
struct Game {
    record: Vec<CubeSet>,
}

fn parse_input(input: String) -> Vec<Game> {
    return input
        .lines()
        .map(|line| {
            let sets_start_idx = line.find(":").unwrap() + 1;
            let sets_str = (&line[sets_start_idx..]).trim();
            let sets: Vec<CubeSet> = sets_str
                .split(";")
                .map(|set_str| {
                    set_str
                        .trim()
                        .split(",")
                        .map(|count_str| count_str.trim().split_once(" ").unwrap())
                        .map(|(n, color)| {
                            (color.parse::<Color>().unwrap(), n.parse::<usize>().unwrap())
                        })
                        .collect()
                })
                .collect();
            return Game { record: sets };
        })
        .collect();
}

pub fn solve_a(input: String) -> i64 {
    let available_cubes = HashMap::from([(Color::Red, 12), (Color::Green, 13), (Color::Blue, 14)]);

    return parse_input(input)
        .iter()
        .enumerate()
        .filter(|(_, game)| {
            return game.record.iter().all(|set| {
                set.iter()
                    .all(|(color, count)| (available_cubes.get(color).unwrap()) >= (count))
            });
        })
        .fold(0, |a, (i, _)| a + i + 1) as i64;
}

pub fn solve_b(input: String) -> i64 {
    return parse_input(input)
        .iter()
        .map(|game| {
            let mut min_set = HashMap::<Color, usize>::new();

            game.record.iter().for_each(|set| {
                set.into_iter().for_each(|(&color, &n)| {
                    let &updated = min_set.get(&color).unwrap_or(&0).max(&n);
                    min_set.insert(color, updated);
                })
            });

            min_set.iter().map(|(_, n)| n).product::<usize>()
        })
        .sum::<usize>() as i64;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_eg_1() {
        assert_eq!(solve_a(get_sample_input()), 8);
    }

    #[test]
    fn a() {
        assert_eq!(solve_a(get_input()), 2169);
    }

    #[test]
    fn b_eg_1() {
        assert_eq!(solve_b(get_sample_input()), 2286);
    }

    #[test]
    fn b() {
        assert_eq!(solve_b(get_input()), 60948);
    }

    //
}
