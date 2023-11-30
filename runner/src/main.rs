use day_01;
use day_02;
use day_03;
use day_04;
use day_05;
use day_06;
use day_07;
use day_08;
use day_09;
use day_10;
use day_11;
use day_12;
use day_13;
use day_14;
use day_15;
use day_16;
use day_17;
use day_18;
use day_19;
use day_20;
use day_21;
use day_22;
use day_23;
use day_24;
use day_25;

use std::env;
use std::fs;

const SOLVE_FUNCTIONS: [fn(String); 25] = [
     day_01::solve,
     day_02::solve,
     day_03::solve,
     day_04::solve,
     day_05::solve,
     day_06::solve,
     day_07::solve,
     day_08::solve,
     day_09::solve,
     day_10::solve,
     day_11::solve,
     day_12::solve,
     day_13::solve,
     day_14::solve,
     day_15::solve,
     day_16::solve,
     day_17::solve,
     day_18::solve,
     day_19::solve,
     day_20::solve,
     day_21::solve,
     day_22::solve,
     day_23::solve,
     day_24::solve,
     day_25::solve,
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
