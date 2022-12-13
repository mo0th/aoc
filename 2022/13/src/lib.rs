use std::{cmp::Ordering, ops::Index, str::FromStr};

pub fn get_input() -> String {
    String::from(include_str!("../input"))
}

pub fn get_sample_input() -> String {
    String::from(include_str!("../sample"))
}

#[derive(Debug)]
enum PacketValue {
    Int(i64),
    List(Vec<PacketValue>),
}
use PacketValue::*;

impl Clone for PacketValue {
    fn clone(&self) -> Self {
        match self {
            Int(n) => Int(*n),
            List(vals) => List(vals.clone()),
        }
    }
}

impl FromStr for PacketValue {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut children_stack = vec![vec![]];

        for part in s[1..(s.len() - 1)].split(",").filter(|p| p.len() > 0) {
            // println!("{children_stack:?}");
            let add_depth_amt = part.chars().take_while(|c| *c == '[').count();

            let num_part = part
                .chars()
                .skip(add_depth_amt)
                .take_while(|c| *c >= '0' && *c <= '9')
                .collect::<String>();

            let remove_depth_count = part.chars().skip(add_depth_amt + num_part.len()).count();
            // println!("part {part:?}; add_depth {add_depth_amt}; num_part {num_part:?}; remove_depth {remove_depth_count}");

            (0..add_depth_amt).for_each(|_| children_stack.push(vec![]));

            if let Ok(num) = num_part.parse::<i64>() {
                children_stack.last_mut().unwrap().push(Int(num))
            }

            for _ in 0..remove_depth_count {
                let top = children_stack.pop().unwrap();
                children_stack.last_mut().unwrap().push(List(top.clone()));
            }
        }

        Ok(List(children_stack[0].clone()))
    }
}

impl PacketValue {
    fn to_list_if_needed(&self) -> Self {
        match *self {
            Int(_) => List(vec![self.clone()]),
            List(_) => self.clone(),
        }
    }
}

impl PartialEq for PacketValue {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Int(l0), Self::Int(r0)) => l0 == r0,
            (Self::List(l0), Self::List(r0)) => **l0 == **r0,
            _ => false,
        }
    }
}

impl Eq for PacketValue {}

impl PartialOrd for PacketValue {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match (self, other) {
            (Self::Int(s), Self::Int(o)) => Some(s.cmp(o)),
            (Int(_), List(_)) | (List(_), Int(_)) => self
                .to_list_if_needed()
                .partial_cmp(&other.to_list_if_needed()),
            (List(left), List(right)) => {
                for i in 0..(left.len()) {
                    if i >= right.len() {
                        return Some(Ordering::Greater);
                    }

                    let left = &left[i];
                    let right = &right[i];

                    let cmp = left.cmp(&right);

                    if !matches!(cmp, Ordering::Equal) {
                        return Some(cmp);
                    }
                }

                if left.len() != right.len() {
                    return Some(Ordering::Less);
                }
                return Some(Ordering::Equal);
            }
        }
    }
}

impl Ord for PacketValue {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

pub fn solve_a(input: String) -> i64 {
    input
        .split("\n\n")
        .map(|pair_chunk| pair_chunk.split_once("\n").unwrap())
        .map(|(left, right)| {
            (
                PacketValue::from_str(left).unwrap(),
                PacketValue::from_str(right).unwrap(),
            )
        })
        .map(|(left, right)| left < right)
        .enumerate()
        .filter_map(|(i, ordered)| if ordered { Some(i + 1) } else { None })
        .sum::<usize>() as i64
}

pub fn solve_b(input: String) -> i64 {
    let dividers = vec![
        List(vec![List(vec![Int(2)])]),
        List(vec![List(vec![Int(6)])]),
    ];

    let parts: Vec<_> = input
        .split("\n")
        .filter(|line| line.len() > 0)
        .map(|line| PacketValue::from_str(line).unwrap())
        .collect();

    let mut all = [dividers.clone(), parts].concat();
    all.sort();

    dividers
        .clone()
        .iter()
        .map(|d| all.iter().enumerate().find(|(i, el)| d == *el).unwrap().0 as i64 + 1)
        .product::<i64>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    // #[ignore]
    fn a_eg_1() {
        assert_eq!(solve_a(get_sample_input()), 13);
    }

    #[test]
    // #[ignore]
    fn a() {
        assert_eq!(solve_a(get_input()), 5588);
    }

    #[test]
    // #[ignore]
    fn b_eg_1() {
        assert_eq!(solve_b(get_sample_input()), 140);
    }

    #[test]
    // #[ignore]
    fn b() {
        assert_eq!(solve_b(get_input()), 23958);
    }

    //
}
