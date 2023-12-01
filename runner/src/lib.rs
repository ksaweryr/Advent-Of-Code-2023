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