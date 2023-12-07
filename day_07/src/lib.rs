use std::str::FromStr;

use anyhow::Error;
use counter::Counter;

pub fn solve(input: String) {
    println!("{}", winnings(&parse_input_1(&input)));
    println!("{}", winnings(&parse_input_2(&input)));
}

fn parse_input_1(input: &str) -> Vec<Bid> {
    input.lines()
        .map(|l| l.parse::<Bid>())
        .collect::<Result<Vec<_>, _>>()
        .expect("Invalid input format")
}

fn parse_input_2(input: &str) -> Vec<Bid> {
    input.lines()
        .map(|l| Bid::from_str_with_jokers(l))
        .collect::<Result<Vec<_>, _>>()
        .expect("Invalid input format")
}

fn winnings(bids: &Vec<Bid>) -> usize {
    let mut bids = bids.clone();
    bids.sort_by(|a, b| a.hand.cmp(&b.hand));
    bids.into_iter().enumerate().map(|(i, b)| (i + 1) * b.amount).sum()
}

fn card_value(card: char, use_jokers: bool) -> Option<usize> {
    if card.is_digit(10) && card > '1' {
        return Some(card as usize - '0' as usize);
    }

    match card {
        'T' => Some(10),
        'J' => Some(if !use_jokers { 11 } else { 1 }),
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
            .map(|c| card_value(c, false))
            .collect::<Option<Vec<_>>>()
            .ok_or(Error::msg("Invalid card "))?
            .into_iter()
            .fold(0, |acc, v| acc * 100 + v);

        Ok(Hand { hand_value, hand_type })
    }
}

impl Hand {
    fn from_str_with_jokers(s: &str) -> Result<Self, Error> {
        let hand_value = s.chars()
            .map(|c| card_value(c, true))
            .collect::<Option<Vec<_>>>()
            .ok_or(Error::msg("Invalid card "))?
            .into_iter()
            .fold(0, |acc, v| acc * 100 + v);

        let number_of_jokers = s.chars().filter(|c| c == &'J').count();

        if number_of_jokers >= 4 {
            return Ok(Hand { hand_value, hand_type: HandType::FiveOfAKind });
        }

        let counts = s.chars().filter(|c| c != &'J').collect::<Counter<_>>().most_common();

        let hand_type = match counts[0].1 + number_of_jokers {
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

impl Bid {
    fn from_str_with_jokers(s: &str) -> Result<Self, Error> {
        let (hand, bid) = s.split_once(" ").ok_or(Error::msg("Invalid bid format"))?;

        Ok(Bid {
            hand: Hand::from_str_with_jokers(hand)?,
            amount: bid.parse::<usize>()?
        })
    }
}

#[cfg(test)]
mod test {
    use crate::*;

    #[test]
    fn example_part1() {
        let input = parse_input_1(EXAMPLE_INPUT);
        assert_eq!(winnings(&input), 6440);
    }

    #[test]
    fn example_part2() {
        let input = parse_input_2(EXAMPLE_INPUT);
        assert_eq!(winnings(&input), 5905);
    }

    const EXAMPLE_INPUT: &str = "32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483";
}