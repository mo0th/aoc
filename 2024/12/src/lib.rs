use std::collections::{HashMap, HashSet, VecDeque};

pub fn get_input() -> String {
    String::from(include_str!("../input"))
}

pub fn get_sample_input() -> String {
    String::from(include_str!("../sample"))
}

fn find_contiguous(set: &HashSet<(i64, i64)>) -> Vec<HashSet<(i64, i64)>> {
    let mut seen = HashSet::new();

    let mut result = vec![];

    for pos in set.iter() {
        if seen.contains(pos) {
            continue;
        }

        seen.insert(*pos);

        let mut next_section = HashSet::new();
        let mut queue = VecDeque::new();
        queue.push_back(*pos);

        while !queue.is_empty() {
            let pos = queue.pop_front().unwrap();
            seen.insert(pos);
            next_section.insert(pos);

            for (dx, dy) in [(1, 0), (0, 1), (-1, 0), (0, -1)] {
                let connected_pos = (pos.0 + dx, pos.1 + dy);

                if seen.contains(&connected_pos) {
                    continue;
                }

                seen.insert(connected_pos.clone());

                if set.contains(&connected_pos) {
                    queue.push_back(connected_pos);
                }
            }
        }

        result.push(next_section);
    }

    result
}

fn measure(set: &HashSet<(i64, i64)>) -> (i64, i64) {
    let area = set.len() as i64;

    let perimeter = set
        .iter()
        .map(|(x, y)| {
            4i64 - [(1, 0), (0, 1), (-1, 0), (0, -1)]
                .iter()
                .filter(|(dx, dy)| {
                    let connected_pos = (x + dx, y + dy);

                    set.contains(&connected_pos)
                })
                .count() as i64
        })
        .sum::<i64>();

    (area, perimeter)
}

pub fn solve_a(input: String) -> i64 {
    let mut areas: HashMap<char, HashSet<(i64, i64)>> = HashMap::new();

    input
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.char_indices()
                .map(move |(x, ch)| (ch, (x as i64, y as i64)))
        })
        .for_each(|(ch, pos)| {
            areas.entry(ch).or_default().insert(pos);
        });

    areas
        .iter()
        .map(|(_, poses)| {
            find_contiguous(poses)
                .iter()
                .map(measure)
                .map(|(area, perimeter)| area * perimeter)
                .sum::<i64>()
        })
        .sum()
}

pub fn solve_b(input: String) -> i64 {
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_eg_1() {
        assert_eq!(solve_a(get_sample_input()), 140);
    }

    #[test]
    fn a_eg_2() {
        assert_eq!(
            solve_a(
                "RRRRIICCFF
RRRRIICCCF
VVRRRCCFFF
VVRCCCJFFF
VVVVCJJCFE
VVIVCCJJEE
VVIIICJJEE
MIIIIIJJEE
MIIISIJEEE
MMMISSJEEE"
                    .to_string()
            ),
            1930
        )
    }

    #[test]
    fn a_eg_3() {
        assert_eq!(
            solve_a(
                "OOOOO
OXOXO
OOOOO
OXOXO
OOOOO"
                    .to_string()
            ),
            772
        )
    }

    #[test]
    fn a() {
        assert_eq!(solve_a(get_input()), 1533024);
    }

    #[test]
    #[ignore]
    fn b_eg_1() {
        assert_eq!(solve_b(get_sample_input()), 80);
    }

    #[test]
    #[ignore]
    fn b() {
        assert_eq!(solve_b(get_input()), 1);
    }

    //
}
