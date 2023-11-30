#!/bin/bash

mkdir input

cargo new runner --vcs none

for day in day_{01..25}; do
    cargo new $day --lib --vcs none
    cat > $day/src/lib.rs <<EOF
pub fn solve(input: String) {
    println!("TBI");
}
EOF
    echo "$day = { path = \"../$day\" }" >> runner/Cargo.toml
done

cat > .gitignore <<EOF
/input
/target
EOF

cat > Cargo.toml <<EOF
[workspace]

members = [
    "runner",
$(printf '    "day_%.2d",\n' {1..25})
]
EOF

cat > runner/src/main.rs <<EOF
$(printf 'use day_%.2d;\n' {1..25})

use std::env;
use std::fs;

const SOLVE_FUNCTIONS: [fn(String); 25] = [
$(printf '     day_%.2d::solve,\n' {1..25})
];

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        println!("Usage: ./{} DAY", args[0]);
        return;
    }

    match args[1].parse::<usize>() {
        Ok(n) => {
            if n < 1 || n > 25 {
                println!("DAY must be in range [1,25]");
            } else {
                SOLVE_FUNCTIONS[n - 1](get_input(n));
            }
        }
        Err(_) => println!("DAY must be an integer")
    }
}

pub fn get_input(day: usize) -> String {
    fs::read_to_string(format!("input/day_{:0>2}.txt", day))
        .expect(&format!("Unable to read input for day {}", day))
}
EOF

git init
git add .