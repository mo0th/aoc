use core::time;
use std::collections::{HashMap, HashSet, VecDeque};

pub fn get_input() -> String {
    String::from(include_str!("../input"))
}

pub fn get_sample_input() -> String {
    String::from(include_str!("../sample"))
}

fn position_to_num(position: &str) -> u8 {
    position.bytes().nth(0).unwrap() - b'A'
}

pub fn solve_a(input: String) -> i64 {
    let map = input
        .lines()
        .map(|line| {
            let line = line.replace("Valve ", "");
            let (valve, line) = line.split_once(" has flow rate=").unwrap();
            let (flow_rate, line) = if let Some(tup) = line.split_once("; tunnels lead to valves ")
            {
                tup
            } else if let Some(tup) = line.split_once("; tunnel leads to valve ") {
                tup
            } else {
                panic!()
            };
            let flow_rate = flow_rate.parse::<i64>().unwrap();
            let to_valves = line
                .split(", ")
                .map(|v| position_to_num(v))
                .collect::<Vec<_>>();

            (position_to_num(valve), (flow_rate, to_valves))
        })
        .collect::<HashMap<_, _>>();

    let mut queue = VecDeque::new();
    queue.push_back((position_to_num("AA"), 30i64, 0i64, vec![], vec![]));
    let mut max_at_time = HashMap::new();
    let mut max_flow = 0i64;
    let mut paths = vec![];

    while let Some((position, time_left, total_flow, mut seen, mut open)) = queue.pop_front() {
        seen.push(position);
        if time_left < 0 {
            continue;
        }
        max_flow = max_flow.max(total_flow);
        paths.push((time_left, total_flow, seen.clone()));

        let max_at_curr_time = *max_at_time.get(&time_left).unwrap_or(&0);
        if total_flow < max_at_curr_time {
            continue;
        } else if total_flow > max_at_curr_time {
            max_at_time.insert(time_left, max_at_curr_time);
        }

        println!("{time_left} {total_flow}");

        let (flow, to_valves) = map.get(&position).unwrap();

        let new_total_flow = total_flow + flow * (time_left - 2);
        // println!("{:?}", (position, flow, to_valves));

        let should_open = !open.contains(&position);

        if should_open {
            open.push(position);
        }

        for &to in to_valves {
            queue.push_back((to, time_left - 1, total_flow, seen.clone(), open.clone()));
            if flow > &0 && should_open {
                queue.push_back((
                    to,
                    time_left - 2,
                    new_total_flow,
                    seen.clone(),
                    open.clone(),
                ));
            }
        }
    }
    for p in paths {
        println!("{p:?}");
    }

    max_flow
}

pub fn solve_b(input: String) -> i64 {
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    // #[ignore]
    fn a_eg_1() {
        assert_eq!(solve_a(get_sample_input()), 1651);
    }

    #[test]
    #[ignore]
    fn a() {
        assert_eq!(solve_a(get_input()), 0);
    }

    #[test]
    #[ignore]
    fn b_eg_1() {
        assert_eq!(solve_b(get_sample_input()), 0);
    }

    #[test]
    #[ignore]
    fn b() {
        assert_eq!(solve_b(get_input()), 0);
    }

    //
}
