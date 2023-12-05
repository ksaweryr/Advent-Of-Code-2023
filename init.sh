#!/bin/bash

mkdir input

cargo new runner --vcs none

cd runner
cargo add chrono
cargo add reqwest -F blocking
cd ..

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
/.cargo/config.toml
EOF

cat > Cargo.toml <<EOF
[workspace]

members = [
    "runner",
$(printf '    "day_%.2d",\n' {1..25})
]
EOF

cat > runner/src/main.rs <<EOF
use runner::get_input;

$(printf 'use day_%.2d;\n' {1..25})

use std::env;

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
EOF

cat > runner/src/lib.rs <<EOF
use chrono::{Datelike, Utc};
use reqwest::StatusCode;
use std::{fs, env};

pub fn get_input(day: usize) -> String {
    fs::read_to_string(format!("input/day_{:0>2}.txt", day))
        .unwrap_or_else(|_| fetch_input(
            day,
            env::var("YEAR").ok().and_then(|s| s.parse::<i32>().ok()).unwrap_or_else(|| Utc::now().year())))
}

fn fetch_input(day: usize, year: i32) -> String {
    let url = format!("https://adventofcode.com/{year}/day/{day}/input");
    let client = reqwest::blocking::Client::new();

    let response = client
        .get(url)
        .header("Cookie", format!("session={}", env::var("SESSION").unwrap_or("".to_string())))
        .send()
        .expect("Error while sending a request");

    let text = match response.status() {
        StatusCode::OK => response.text().unwrap(),
        StatusCode::BAD_REQUEST => panic!("Invalid session cookie"),
        StatusCode::NOT_FOUND => panic!("Puzzle is not available yet"),
        x => panic!("Unkwnon error occured: {x}")
    };

    fs::write(format!("input/day_{:0>2}.txt", day), &text).expect("Couldn't save puzzle input to file");
    text
}
EOF

mkdir .cargo
cat > .cargo/config.toml <<EOF
[env]
YEAR = "$(date +%Y)"
SESSION = "YOUR-SESSION-COOKIE-GOES-HERE"
EOF

git init
git add .