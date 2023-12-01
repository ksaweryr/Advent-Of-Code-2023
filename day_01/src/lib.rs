mod aho_corasick;

use std::collections::HashMap;

use aho_corasick::AhoCorasick;

pub fn solve(input: String) {
    let lines: Vec<_> = input.lines().collect();
    let part1 = lines
        .clone()
        .into_iter()
        .map(task1)
        .sum::<u64>();

    let part2 = lines
        .into_iter()
        .map(task2)
        .sum::<u64>();

    println!("{part1}");
    println!("{part2}");
}

fn task1(line: &str) -> u64 {
    let digits = line.chars().filter(char::is_ascii_digit).map(|c| c as u64 - '0' as u64).collect::<Vec<_>>();
    digits[0] * 10 + digits.last().unwrap()
}

fn task2(line: &str) -> u64 {
    let mut mapper = HashMap::new();
    let words: Vec<String> = vec!["one", "two", "three", "four", "five", "six", "seven", "eight", "nine", "1", "2", "3", "4", "5", "6", "7", "8", "9"].into_iter().map(|s| s.to_owned()).collect();

    for (i, w) in words.iter().enumerate() {
        mapper.insert(w.to_owned().to_owned(), ((i % 9) + 1) as u64);
    }

    let ac = AhoCorasick::from_words(words.clone());

    let digits: Vec<_> = ac.find_matches(&line).map(|m| *mapper.get(&m.word).unwrap()).collect();

    digits[0] * 10 + digits.last().unwrap()
}