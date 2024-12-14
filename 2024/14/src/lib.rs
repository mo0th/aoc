use std::{
    collections::{HashMap, HashSet},
    io::{stdout, Write},
};

pub fn get_input() -> String {
    String::from(include_str!("../input"))
}

pub fn get_sample_input() -> String {
    String::from(include_str!("../sample"))
}

fn get_quadrant(pos: (i64, i64), dim: &(i64, i64)) -> i64 {
    let mid_x = dim.0 / 2;
    let mid_y = dim.1 / 2;

    if pos.0 < mid_x && pos.1 < mid_y {
        1
    } else if pos.0 < mid_x && pos.1 > mid_y {
        2
    } else if pos.0 > mid_x && pos.1 < mid_y {
        3
    } else if pos.0 > mid_x && pos.1 > mid_y {
        4
    } else {
        0
    }
}

fn robots_to_bytes(dim: &(i64, i64), robots: &Vec<((i64, i64), (i64, i64))>) -> String {
    let mut str = String::with_capacity(((dim.0 + 1) * (dim.1 + 1)) as usize);

    let mut count_by_position = HashMap::new();

    robots.iter().for_each(|(p, _)| {
        *count_by_position.entry(p).or_insert(0) += 1;
    });

    for y in 0..dim.1 {
        for x in 0..dim.0 {
            let count = *count_by_position.get(&(x, y)).unwrap_or(&0);

            str.push(if count == 0 {
                '.'
            } else {
                char::from_digit(count as u32, 10).unwrap()
            });
        }

        str.push_str("\n");
    }

    str
}

pub fn solve_a(input: String, dim: (i64, i64)) -> i64 {
    let robots = input
        .lines()
        .map(|line| {
            let (p, v) = line.split_once(" ").unwrap();
            let p = &p[2..].split_once(",").unwrap();
            let v = &v[2..].split_once(",").unwrap();

            (
                (p.0.parse::<i64>().unwrap(), p.1.parse::<i64>().unwrap()),
                (v.0.parse::<i64>().unwrap(), v.1.parse::<i64>().unwrap()),
            )
        })
        .collect::<Vec<_>>();

    let mut count_by_quadrant = HashMap::new();

    robots.iter().for_each(|(p, v)| {
        let quadrant = get_quadrant(add_velocity_times(p, v, &dim, 100), &dim);

        *count_by_quadrant.entry(quadrant).or_insert(0) += 1;
    });

    count_by_quadrant
        .iter()
        .filter_map(|(q, c)| if q == &0 { None } else { Some(c) })
        .product()
}

fn add_velocity_times(p: &(i64, i64), v: &(i64, i64), dim: &(i64, i64), times: i64) -> (i64, i64) {
    let mut x = (p.0 + v.0 * times) % dim.0;

    if x < 0 {
        x += dim.0;
    }

    let mut y = (p.1 + v.1 * times) % dim.1;

    if y < 0 {
        y += dim.1;
    }

    (x, y)
}

fn add_velocity(p: &(i64, i64), v: &(i64, i64), dim: &(i64, i64)) -> (i64, i64) {
    add_velocity_times(p, v, dim, 1)
}

fn write_robots(
    f: &mut impl Write,
    iteration: i64,
    robots: &Vec<((i64, i64), (i64, i64))>,
    dim: &(i64, i64),
) {
    f.write(iteration.to_string().as_bytes()).unwrap();
    f.write(b"\n").unwrap();
    f.write(robots_to_bytes(&dim, &robots).as_bytes()).unwrap();
    f.write(b"\n\n\n").unwrap();
}

#[cfg(test)]
const PRINT_TREE: bool = false;
#[cfg(not(test))]
const PRINT_TREE: bool = true;

pub fn solve_b(input: String, dim: (i64, i64)) -> i64 {
    let mut robots = input
        .lines()
        .map(|line| {
            let (p, v) = line.split_once(" ").unwrap();
            let p = &p[2..].split_once(",").unwrap();
            let v = &v[2..].split_once(",").unwrap();

            (
                (p.0.parse::<i64>().unwrap(), p.1.parse::<i64>().unwrap()),
                (v.0.parse::<i64>().unwrap(), v.1.parse::<i64>().unwrap()),
            )
        })
        .collect::<Vec<_>>();

    let mut i = 0;
    loop {
        i += 1;
        robots = robots
            .iter()
            .map(|(p, v)| (add_velocity(p, v, &dim), *v))
            .collect::<Vec<_>>();

        let unique_positions = robots.iter().map(|(p, _)| *p).collect::<HashSet<_>>();

        if unique_positions.len() == robots.len() {
            break;
        }
    }

    if PRINT_TREE {
        write_robots(&mut stdout(), i, &robots, &dim);
    }

    return i;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_eg_1() {
        assert_eq!(solve_a(get_sample_input(), (11, 7)), 12);
    }

    #[test]
    fn a() {
        assert_eq!(solve_a(get_input(), (101, 103)), 211773366);
    }

    #[test]
    fn b() {
        assert_eq!(solve_b(get_input(), (101, 103)), 7344);
    }

    //
}
