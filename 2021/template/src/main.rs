use std::env;
use std::process;

use aoc::{get_input, solve_a, solve_b};

fn main() {
    let input = get_input().unwrap_or_else(|err| {
        eprintln!("Problem getting input {}", err);
        process::exit(1);
    });

    let mut args = env::args();

    args.next();

    let part = match args.next() {
        Some(s) => s,
        None => {
            eprintln!("Invalid part");
            process::exit(1);
        }
    };

    match part.as_str() {
        "a" => {
            println!("Part A\n");
            println!("{}", solve_a(input));
        }
        "b" => {
            println!("Part B\n");
            println!("{}", solve_b(input));
        }
        _ => {
            eprintln!("Invalid part");
            process::exit(1);
        }
    }
}
