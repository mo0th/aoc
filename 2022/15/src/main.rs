use std::env;

use aoc::{get_input, solve_a, solve_b};

fn main() {
    let mut args = env::args();

    args.next();

    let part = match args.next() {
        Some(s) => s,
        None => String::from("x"),
    };

    let run_a = || {
        println!("Part A\n");
        println!("{}", solve_a(get_input(), 2000000));
    };

    let run_b = || {
        println!("Part B\n");
        println!("{}", solve_b(get_input(), 4000000));
    };

    match part.as_str() {
        "a" => run_a(),
        "b" => run_b(),
        _ => {
            run_a();
            run_b();
        }
    }
}
