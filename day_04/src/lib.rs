use anyhow::Error;
use std::collections::HashSet;
use std::iter::repeat;
use std::str::FromStr;

pub fn solve(input: String) {
    let cards = input.lines()
        .map(number_of_matches)
        .collect::<Result<Vec<usize>, _>>()
        .expect("Couldn't parse input file");

    println!("{}", part1(&cards));
    println!("{}", part2(&cards));
}

fn part1(values: &Vec<usize>) -> usize {
    values.iter()
        .map(|n| n.checked_sub(1).map(|x| 1 << x).unwrap_or(0))
        .sum()
}

fn part2(values: &Vec<usize>) -> usize {
    let mut counts: Vec<usize> = repeat(1).take(values.len()).collect();
    let mut result = 0;

    for (i, v) in values.iter().enumerate() {
        result += counts[i];

        for j in i+1..((i + 1 + v).min(counts.len())) {
            counts[j] += counts[i];
        }
    }

    result
}

fn number_of_matches(s: &str) -> Result<usize, Error> {
    let [winning_numbers, your_numbers]: [&str; 2] = s.split(": ")
        .last()
        .ok_or(Error::msg("Invalid card format"))?
        .split(" | ")
        .collect::<Vec<&str>>()
        .try_into()
        .map_err(|_| Error::msg("Invalid card format"))?;

    let winning_numbers = parse_number_set(winning_numbers)?;
    let your_numbers = parse_number_set(your_numbers)?;

    Ok(winning_numbers.intersection(&your_numbers).count())
}

fn parse_number_set(s: &str) -> Result<HashSet<usize>, <usize as FromStr>::Err> {
    s.split_whitespace().map(|w| w.parse::<usize>()).collect()
}