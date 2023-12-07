use std::str::FromStr;

use anyhow::Error;
use counter::Counter;

pub fn solve(input: String) {
    let bids = parse_input(input);

    println!("{}", part1(&bids));
}

fn parse_input(input: String) -> Vec<Bid> {
    input.lines()
        .map(|l| l.parse::<Bid>())
        .collect::<Result<Vec<_>, _>>()
        .expect("Invalid input format")
}

fn part1(bids: &Vec<Bid>) -> usize {
    let mut bids = bids.clone();
    bids.sort_by(|a, b| a.hand.cmp(&b.hand));
    bids.into_iter().enumerate().map(|(i, b)| (i + 1) * b.amount).sum()
}

fn card_value(card: char) -> Option<usize> {
    if card.is_digit(10) && card > '1' {
        return Some(card as usize - '0' as usize);
    }

    match card {
        'T' => Some(10),
        'J' => Some(11),
        'Q' => Some(12),
        'K' => Some(13),
        'A' => Some(14),
        _ => None
    }
}

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord)]
enum HandType {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind
}

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord)]
struct Hand {
    hand_type: HandType,
    hand_value: usize
}

impl FromStr for Hand {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() != 5 {
            return Err(Error::msg("Hand must consist of 5 cards"));
        }

        let counts = s.chars().collect::<Counter<_>>().most_common();
        let hand_type = match counts[0].1 {
            5 => HandType::FiveOfAKind,
            4 => HandType::FourOfAKind,
            3 => if counts[1].1 == 2 {
                HandType::FullHouse
            } else {
                HandType::ThreeOfAKind
            },
            2 => if counts[1].1 == 2 {
                HandType::TwoPair
            } else {
                HandType::OnePair
            },
            1 => HandType::HighCard,
            _ => panic!("Will never happen")
        };
        let hand_value = s.chars()
            .map(card_value)
            .collect::<Option<Vec<_>>>()
            .ok_or(Error::msg("Invalid card "))?
            .into_iter()
            .fold(0, |acc, v| acc * 100 + v);

        Ok(Hand { hand_value, hand_type })
    }
}

#[derive(Clone)]
struct Bid {
    hand: Hand,
    amount: usize
}

impl FromStr for Bid {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (hand, bid) = s.split_once(" ").ok_or(Error::msg("Invalid bid format"))?;

        Ok(Bid {
            hand: hand.parse::<Hand>()?,
            amount: bid.parse::<usize>()?
        })
    }
}

#[cfg(test)]
mod test {
    use crate::*;

    #[test]
    fn example_part1() {
        let input = parse_input(EXAMPLE_INPUT.to_owned());
        assert_eq!(part1(&input), 6440);
    }

    const EXAMPLE_INPUT: &str = "32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483";
}