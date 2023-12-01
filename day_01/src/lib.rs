mod aho_corasick;

use std::collections::HashMap;

use aho_corasick::AhoCorasick;

pub fn solve(input: String) {
    let lines: Vec<_> = input.lines().collect();
    let part1 = lines
        .clone()
        .into_iter()
        .map(evaluate_line)
        .sum::<u64>();

    let part2 = lines
        .into_iter()
        .map(|s| {
            let res = evaluate_line(read_digits(s).as_ref());
            // println!("{s} {res}");
            res
        })
        .sum::<u64>();

    println!("{part1}");
    println!("{part2}");
}

fn read_digits(line: &str) -> String {
    let before = line.to_owned();
    let mut line = line.to_owned();
    let mut mapper = HashMap::new();
    let words: Vec<String> = vec!["one", "two", "three", "four", "five", "six", "seven", "eight", "nine"].into_iter().map(|s| s.to_owned()).collect();

    for (i, w) in words.iter().enumerate() {
        mapper.insert(w.to_owned().to_owned(), (i + 1).to_string());
    }

    let ac = AhoCorasick::from_words(words.clone());

    match ac.match_first(&line) {
        Some((start, word)) => {
            let mut target = mapper.get(&word).unwrap().to_owned();
            target.push(word.chars().last().unwrap());
            line.replace_range(start..(start+word.len()), target.as_ref())
        },
        None => {}
    };

    let ac = AhoCorasick::from_words(words.into_iter().map(|s| reversed(&s)).collect());

    match ac.match_first(&reversed(&line)) {
        Some((start, word)) => line.replace_range((line.len() - (start+word.len()))..(line.len() - start), mapper.get(&reversed(&word)).unwrap()),
        None => {}
    }

    println!("{before} {}", evaluate_line(&line));

    line
}

fn evaluate_line(line: &str) -> u64 {
    let digits = line.chars().filter(char::is_ascii_digit).map(|c| c as u64 - '0' as u64).collect::<Vec<_>>();
    digits[0] * 10 + digits.last().unwrap()
}

fn reversed(s: &str) -> String {
    s.chars().rev().collect::<String>()
}