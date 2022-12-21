use std::collections::HashSet;

pub fn get_input() -> String {
    String::from(include_str!("../input"))
}

pub fn get_sample_input() -> String {
    String::from(include_str!("../sample"))
}

type Point = (i64, i64, i64);

trait AddExt {
    fn padd(&self, rhs: &Self) -> Self;
}

impl AddExt for Point {
    fn padd(&self, rhs: &Self) -> Self {
        (self.0 + rhs.0, self.1 + rhs.1, self.2 + rhs.2)
    }
}

pub fn solve_a(input: String) -> i64 {
    let points = input
        .lines()
        .map(|line| {
            let parts = line
                .split(",")
                .map(|part| part.parse::<i64>().unwrap())
                .collect::<Vec<_>>();
            (parts[0], parts[1], parts[2])
        })
        .collect::<HashSet<_>>();

    let diffs = [
        (1, 0, 0),
        (-1, 0, 0),
        (0, 1, 0),
        (0, -1, 0),
        (0, 0, 1),
        (0, 0, -1),
    ];
    let mut count = 0;

    points.iter().for_each(|p| {
        count += diffs
            .iter()
            .filter(|d| !points.contains(&p.padd(*d)))
            .count();
    });

    count as i64
}

fn minmax<T: Ord + Copy>(input: &Vec<T>) -> Option<(T, T)> {
    let min = input.iter().min()?;
    let max = input.iter().max()?;
    Some((*min, *max))
}

pub fn solve_b(input: String) -> i64 {
    let points = input
        .lines()
        .map(|line| {
            let parts = line
                .split(",")
                .map(|part| part.parse::<i64>().unwrap())
                .collect::<Vec<_>>();
            (parts[0], parts[1], parts[2])
        })
        .collect::<HashSet<_>>();

    let diffs = [
        (1, 0, 0),
        (-1, 0, 0),
        (0, 1, 0),
        (0, -1, 0),
        (0, 0, 1),
        (0, 0, -1),
    ];
    let mut count = 0;

    let points_vec: Vec<_> = points.iter().collect();

    let (x_min, x_max) = minmax(&points.iter().map(|p| p.0).collect()).unwrap();
    let (y_min, y_max) = minmax(&points.iter().map(|p| p.1).collect()).unwrap();
    let (z_min, z_max) = minmax(&points.iter().map(|p| p.2).collect()).unwrap();

    // let mut visible = HashSet::<Point>::new();

    // points.iter().for_each(|p| {
    //     count += diffs
    //         .iter()
    //         .filter(|d| !points.contains(&p.padd(*d)))
    //         .count();
    // });

    count as i64
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[ignore]
    fn a_eg_1() {
        assert_eq!(solve_a(get_sample_input()), 64);
    }

    #[test]
    #[ignore]
    fn a() {
        assert_eq!(solve_a(get_input()), 3396);
    }

    #[test]
    // #[ignore]
    fn b_eg_1() {
        assert_eq!(solve_b(get_sample_input()), 58);
    }

    #[test]
    #[ignore]
    fn b() {
        assert_eq!(solve_b(get_input()), 0);
    }

    //
}
