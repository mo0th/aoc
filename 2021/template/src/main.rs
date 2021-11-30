use std::env;
use std::process;

use aoc::{get_inputs, solve_a, solve_b};

fn main() {
    let (input_a, input_b) = get_inputs().unwrap_or_else(|err| {
        eprintln!("Problem getting inputs {}", err);
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
            println!("{}", solve_a(input_a));
        }
        "b" => {
            println!("Part B\n");
            println!("{}", solve_b(input_b));
        }
        _ => {
            eprintln!("Invalid part");
            process::exit(1);
        }
    }
}
