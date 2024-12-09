use std::{
    ascii::AsciiExt,
    collections::{HashSet, VecDeque},
    iter::Zip,
    string::ParseError,
};

pub fn get_input() -> String {
    String::from(include_str!("../input"))
}

pub fn get_sample_input() -> String {
    String::from(include_str!("../sample"))
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Block {
    Empty,
    File(i64),
}

fn print_blocks(blocks: &Vec<&Block>) {
    let str = blocks
        .iter()
        .map(|b| match b {
            Block::Empty => ".".to_string(),
            Block::File(i) => i.to_string(),
        })
        .collect::<String>();

    println!("{}", str);
}

pub fn solve_a(input: String) -> i64 {
    let mut blocks = input
        .char_indices()
        .flat_map(|(i, c)| {
            if i % 2 == 0 {
                return (0..c.to_digit(10).unwrap())
                    .map(|_| Block::File(i as i64 / 2))
                    .collect::<Vec<_>>();
            }

            return (0..c.to_digit(10).unwrap())
                .map(|_| Block::Empty)
                .collect::<Vec<_>>();
        })
        .collect::<Vec<_>>();

    let mut non_empty_from_end = blocks
        .iter()
        .enumerate()
        .rev()
        .filter(|(_, block)| **block != Block::Empty)
        .peekable();

    let mut result = vec![];

    for (i, block) in blocks.iter().enumerate() {
        let (j, next_non_empty_from_end) = non_empty_from_end.peek().unwrap();

        if j < &i {
            break;
        }

        match block {
            Block::File(_) => {
                result.push(block);
            }
            Block::Empty => {
                result.push(next_non_empty_from_end);
                let _ = non_empty_from_end.next();
            }
        }
    }

    result
        .iter()
        .enumerate()
        .map(|(i, block)| match block {
            Block::File(f) => i as i64 * *f,
            Block::Empty => 0,
        })
        .sum::<i64>()
}

fn print_sections(sections: &VecDeque<(i64, i64, Block)>, moved_ids: &Vec<i64>) {
    let mut seen = HashSet::new();

    let str = sections
        .iter()
        .flat_map(|(_, len, block)| {
            let ch = match block.clone() {
                Block::Empty => ".".to_string(),
                Block::File(i) => {
                    if seen.contains(&i) && moved_ids.contains(&i) {
                        ".".to_string()
                    } else {
                        seen.insert(i);
                        i.to_string()
                    }
                }
            };

            (0..*len).map(move |_| ch.clone())
        })
        .collect::<String>();

    println!("{str}")
}

pub fn solve_b(input: String) -> i64 {
    let mut parsed_input = input
        .char_indices()
        .map(|(i, c)| {
            (
                c.to_digit(10).unwrap() as i64,
                if i % 2 == 0 {
                    Block::File(i as i64 / 2)
                } else {
                    Block::Empty
                },
            )
        })
        .collect::<VecDeque<_>>();

    let mut sections = VecDeque::new();

    let mut i = 0;
    for (len, block) in parsed_input.iter() {
        sections.push_back((i, *len, block.clone()));
        i += len;
    }

    let sections_to_move = sections.clone();
    let sections_to_move = sections_to_move
        .iter()
        .filter(|s| matches!(s.2, Block::File(_)))
        .rev()
        .collect::<Vec<_>>();

    let mut moved_ids = vec![];

    for section_to_move in sections_to_move {
        for i in 0..sections.len() {
            if i as i64 > section_to_move.0 {
                break;
            }

            let section = sections[i];

            if let Block::File(_) = section.2 {
                continue;
            }

            if section.1 < section_to_move.1 {
                continue;
            } else {
                if section.1 == section_to_move.1 {
                    sections[i].2 = section_to_move.2;
                } else {
                    sections[i] = (section.0, section_to_move.1, section_to_move.2);
                    sections.insert(
                        i + 1,
                        (
                            section.0 + section_to_move.1,
                            (section.1 - section_to_move.1),
                            Block::Empty,
                        ),
                    );
                };

                if let Block::File(id) = section_to_move.2 {
                    moved_ids.push(id);
                }

                break;
            }
        }
    }

    let mut seen: HashSet<i64> = HashSet::new();

    sections
        .iter()
        .flat_map(|(_, len, block)| {
            let mut block = *block;
            if let Block::File(id) = block {
                if seen.contains(&id) {
                    block = Block::Empty;
                } else {
                    seen.insert(id);
                }
            }

            (0..*len).map(move |_| block)
        })
        .enumerate()
        .map(|(i, block)| match block {
            Block::File(f) => i as i64 * f,
            Block::Empty => 0,
        })
        .sum::<i64>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_eg_1() {
        assert_eq!(solve_a(get_sample_input()), 1928);
    }

    #[test]
    fn a() {
        assert_eq!(solve_a(get_input()), 6291146824486);
    }

    #[test]
    fn b_eg_1() {
        assert_eq!(solve_b(get_sample_input()), 2858);
    }

    #[test]
    fn b() {
        assert_eq!(solve_b(get_input()), 6307279963620);
    }

    //
}
