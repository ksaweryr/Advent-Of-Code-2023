use std::{str::FromStr, collections::HashMap};

use anyhow::Error;

pub fn solve(input: String) {
    let (directions, nodes) = parse_input(&input).unwrap();

    println!("{}", part1(&directions, &nodes));
}

fn parse_input(input: &str) -> Result<(Vec<char>, Vec<Node>), Error> {
    let mut lines = input.lines();
    let directions = lines.nth(0).ok_or(Error::msg("Invalid input"))?.chars().collect();
    let nodes = lines.skip(1).map(|l| l.parse::<Node>()).collect::<Result<Vec<Node>, _>>()?;

    Ok((directions, nodes))
}

fn part1(directions: &Vec<char>, nodes: &Vec<Node>) -> usize {
    let map = nodes.iter().map(|n| (n.label.as_ref(), n)).collect::<HashMap<_, _>>();

    directions.iter().cycle().scan("AAA", |state, direction| {
        *state = &map.get(*state).expect(&format!("Invalid node {}", *state)).get_next(*direction).expect("Invalid direction");
        Some(*state)
    })
    .enumerate()
    .find(|(_, n)| *n == "ZZZ").unwrap().0 + 1
}

struct Node {
    label: String,
    left: String,
    right: String
}

impl FromStr for Node {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (label, edges) = s.split_once(" = ").ok_or(Error::msg("Invalid entry format"))?;
        let (left, right) = edges[1..edges.len() - 1].split_once(", ").ok_or(Error::msg("Invalid edges format"))?;

        Ok(Node { label: label.to_owned(), left: left.to_owned(), right: right.to_owned() })
    }
}

impl Node {
    fn get_next(&self, next: char) -> Option<&str> {
        match next {
            'L' => Some(&self.left),
            'R' => Some(&self.right),
            _   => None
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn example1_part1() {
        let (directions, nodes) = parse_input(&EXAPLE_INPUT_1).unwrap();
        assert_eq!(part1(&directions, &nodes), 2);
    }

    #[test]
    fn example2_part1() {
        let (directions, nodes) = parse_input(&EXAPLE_INPUT_2).unwrap();
        assert_eq!(part1(&directions, &nodes), 6);
    }

    const EXAPLE_INPUT_1: &str = "RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)";

const EXAPLE_INPUT_2: &str = "LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)";
}