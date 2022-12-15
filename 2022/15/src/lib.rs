use rayon::prelude::*;
use std::collections::HashSet;

pub fn get_input() -> String {
    String::from(include_str!("../input"))
}

pub fn get_sample_input() -> String {
    String::from(include_str!("../sample"))
}

trait DistExt {
    fn dist(&self, other: &Self) -> i64;
}

impl DistExt for (i64, i64) {
    fn dist(&self, other: &Self) -> i64 {
        (self.0 - other.0).abs() + (self.1 - other.1).abs()
    }
}

pub fn solve_a(input: String, y: i64) -> i64 {
    let sensors: Vec<((i64, i64), (i64, i64))> = input
        .lines()
        .map(|line| {
            let line = line.replace("Sensor at x=", "");
            let (sx, line) = line.split_once(", y=").unwrap();
            let (sy, line) = line.split_once(": closest beacon is at x=").unwrap();
            let (bx, by) = line.split_once(", y=").unwrap();

            return (
                (sx.parse().unwrap(), sy.parse().unwrap()),
                (bx.parse().unwrap(), by.parse().unwrap()),
            );
        })
        .collect::<Vec<_>>();

    let mut blocked: HashSet<(i64, i64)> = HashSet::new();
    let beacons = sensors.iter().map(|(_, b)| *b).collect::<HashSet<_>>();

    sensors.iter().for_each(|(sensor, beacon)| {
        let beacon_dist = sensor.dist(&beacon);
        let dist_to_y = sensor.dist(&(sensor.0, y));
        let leftover_for_x = beacon_dist - dist_to_y;
        let x_start = sensor.0 - leftover_for_x;
        let x_end = sensor.0 + leftover_for_x;
        blocked.extend((x_start..=x_end).map(|x| (x, y)))
    });

    blocked.difference(&beacons).count() as i64
}

pub fn solve_b(input: String, y_max: i64) -> i64 {
    let sensors: Vec<((i64, i64), i64)> = input
        .lines()
        .map(|line| {
            let line = line.replace("Sensor at x=", "");
            let (sx, line) = line.split_once(", y=").unwrap();
            let (sy, line) = line.split_once(": closest beacon is at x=").unwrap();
            let (bx, by) = line.split_once(", y=").unwrap();

            return (
                (sx.parse().unwrap(), sy.parse().unwrap()),
                (bx.parse().unwrap(), by.parse().unwrap()),
            );
        })
        .map(|(sensor, beacon)| (sensor, sensor.dist(&beacon)))
        .collect::<Vec<_>>();

    (0..=y_max)
        .into_par_iter()
        .map(|y| {
            let mut row_intervals: Vec<_> = sensors
                .iter()
                .filter_map(|(sensor, beacon_dist)| {
                    let y_diff = (sensor.1 - y).abs();
                    let x_diff = beacon_dist - y_diff;

                    if x_diff < 0 {
                        return None;
                    }

                    return Some((sensor.0 - x_diff, sensor.0 + x_diff));
                })
                .collect();

            row_intervals.sort_by_key(|s| s.0);

            let mut merged: Vec<_> = vec![];
            let mut prev = row_intervals[0];

            for interval in row_intervals.into_iter().skip(1) {
                if prev.1 + 1 >= interval.0 {
                    prev = (prev.0, prev.1.max(interval.1));
                } else {
                    merged.push(prev);
                    prev = interval
                }
            }
            merged.push(prev);

            (y, merged)
        })
        // .find(|(_, intervals)| intervals.len() > 1)
        .find_any(|(_, intervals)| intervals.len() > 1)
        .map(|(y, intervals)| {
            let x = intervals[0].1 + 1;

            x * 4_000_000 + (y as i64)
        })
        .unwrap_or(-1)

    // let result = (0..=x_max)
    //     .into_par_iter()
    //     .flat_map(|x| {
    //         let it = (0..=y_max).into_par_iter().map(move |y| (x, y));
    //         it
    //     })
    //     .find_any(|point| {
    //         sensors
    //             .iter()
    //             .all(|(sensor, dist)| sensor.dist(&point) > *dist)
    //     });
    // return result
    //     .map(|point| point.0 * 4000000 + point.1)
    //     .unwrap_or(-1);

    // for x in 0..=x_max {
    //     for y in 0..=y_max {
    //         if sensors
    //             .iter()
    //             .all(|(sensor, dist)| sensor.dist(&(x, y)) > *dist)
    //         {
    //             return x * 4000000 + y;
    //         }
    //     }
    // }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[ignore]
    fn a_eg_1() {
        assert_eq!(solve_a(get_sample_input(), 10), 26);
    }

    #[test]
    #[ignore]
    fn a() {
        assert_eq!(solve_a(get_input(), 2000000), 4724228);
    }

    #[test]
    #[ignore]
    fn b_eg_1() {
        assert_eq!(solve_b(get_sample_input(), 20), 56000011);
    }

    #[test]
    // #[ignore]
    fn b() {
        assert_eq!(solve_b(get_input(), 4000000), 13622251246513);
    }

    //
}
