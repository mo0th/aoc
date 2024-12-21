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
        println!("Part A");
        println!("{}", solve_a(get_input(), (71, 71), 1024));
    };

    let run_b = || {
        println!("Part B");
        println!("{}", solve_b(get_input()));
    };

    match part.as_str() {
        "a" => run_a(),
        "b" => run_b(),
        _ => {
            run_a();
            println!();
            run_b();
        }
    }
}
