use std::{str::FromStr, collections::HashMap};

use anyhow::Error;

pub fn solve(input: String) {
    let (directions, map) = parse_input(&input).unwrap();

    println!("{}", part1(&directions, &map));
    println!("{}", part2(&directions, &map));
}

fn parse_input(input: &str) -> Result<(Vec<char>, HashMap<String, Node>), Error> {
    let mut lines = input.lines();
    let directions = lines.nth(0).ok_or(Error::msg("Invalid input"))?.chars().collect();
    let nodes = lines.skip(1).map(|l| l.parse::<Node>()).collect::<Result<Vec<Node>, _>>()?;
    let map = nodes.into_iter().map(|n| (n.label.clone(), n)).collect::<HashMap<_, _>>();

    Ok((directions, map))
}

fn part1(directions: &Vec<char>, map: &HashMap<String, Node>) -> usize {
    path_length(directions, map, |s| s == "AAA", |s| s == "ZZZ")
}

fn part2(directions: &Vec<char>, map: &HashMap<String, Node>) -> usize {
    map.iter()
        .filter(|(k, _)| k.ends_with('A')).map(|(_, v)| &v.label)
        .map(|s| path_length(directions, map, |x| x == s, |x| x.ends_with('Z')))
        .reduce(|acc, x| lcm(acc, x)).unwrap()
}

fn path_length<P1, P2>(directions: &Vec<char>, map: &HashMap<String, Node>, start: P1, end: P2) -> usize
    where P1: Fn(&str) -> bool, P2: Fn(&str) -> bool {
    
    let start_node = (*map.keys().find(|x| start(x)).unwrap()).clone();
    directions.iter().cycle().scan(start_node, |state, direction| {
        *state = map.get(state).expect(&format!("Invalid node {}", *state)).get_next(*direction).expect("Invalid direction").to_owned();
        Some(state.clone())
    })
    .enumerate()
    .find(|(_, n)| end(n)).unwrap().0 + 1
}

fn gcd(a: usize, b: usize) -> usize {
    let (a, b) = if a < b {
        (b, a)
    } else {
        (a, b)
    };

    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

fn lcm(a: usize, b: usize) -> usize {
    (a * b) / gcd(a, b)
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
        let (directions, map) = parse_input(&EXAPLE_INPUT_1).unwrap();
        assert_eq!(part1(&directions, &map), 2);
    }

    #[test]
    fn example2_part1() {
        let (directions, map) = parse_input(&EXAPLE_INPUT_2).unwrap();
        assert_eq!(part1(&directions, &map), 6);
    }

    #[test]
    fn example_part2() {
        let (directions, map) = parse_input(&EXAPLE_INPUT_3).unwrap();
        assert_eq!(part2(&directions, &map), 6);
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

const EXAPLE_INPUT_3: &str = "LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)";
}