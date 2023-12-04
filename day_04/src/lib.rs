use anyhow::Error;
use std::collections::HashSet;
use std::str::FromStr;

pub fn solve(input: String) {
    let cards = input.lines()
        .map(|s| s.parse::<Card>())
        .collect::<Result<Vec<Card>, _>>()
        .expect("Couldn't parse input file");

    let part1_result = cards.iter()
        .map(|c| c.winning_numbers.intersection(&c.your_numbers).count().checked_sub(1).map(|x| 1 << x).unwrap_or(0))
        .sum::<usize>();

    println!("{}", part1_result);
}

struct Card {
    winning_numbers: HashSet<usize>,
    your_numbers: HashSet<usize>
}

impl FromStr for Card {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let [winning_numbers, your_numbers]: [&str; 2] = s.split(": ")
            .last()
            .ok_or(Error::msg("Invalid card format"))?
            .split(" | ")
            .collect::<Vec<&str>>()
            .try_into()
            .map_err(|_| Error::msg("Invalid card format"))?;

        let winning_numbers = parse_number_set(winning_numbers)?;
        let your_numbers = parse_number_set(your_numbers)?;

        Ok(Card { winning_numbers, your_numbers })
    }
}

fn parse_number_set(s: &str) -> Result<HashSet<usize>, <usize as FromStr>::Err> {
    s.split_whitespace().map(|w| w.parse::<usize>()).collect()
}