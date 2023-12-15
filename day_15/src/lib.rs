use std::str::FromStr;

use anyhow::Error;

pub fn solve(input: String) {
    let instructions = parse_input(&input);

    println!("{}", part1(&instructions));
    println!("{}", part2(&instructions));
}

fn parse_input(input: &str) -> Vec<&str> {
    input.trim().split(",").collect()
}

fn part1(instructions: &Vec<&str>) -> usize {
    instructions.iter().map(|s| hash(*s)).sum()
}

fn part2(instructions: &Vec<&str>) -> usize {
    let mut hm = AOCHashMap::new();

    instructions.iter().map(|i| i.parse::<Operation>().expect("Invalid instruction"))
        .for_each(|op| {
            match op {
                Operation::Remove(label) => hm.remove(&label),
                Operation::Put(label, value) => hm.put(&label, value)
            }
        }
    );
    
    hm.power()
}

fn hash(s: &str) -> usize {
    s.chars().map(|c| c as u8 as usize).fold(0, |acc, c| 17 * (acc + c) % 256)
}

enum Operation {
    Remove(String),
    Put(String, usize)
}

impl FromStr for Operation {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.chars().last().ok_or(Error::msg("Can't parse empty string"))? == '-' {
            Ok(Self::Remove((s[..s.len() - 1]).to_owned()))
        } else {
            let (label, value) = s.split_once('=').ok_or(Error::msg("Invalid format"))?;

            Ok(Self::Put(label.to_owned(), value.parse()?))
        }
    }
}

struct AOCHashMap {
    bins: Vec<Vec<(String, usize)>>
}

impl AOCHashMap {
    fn new() -> Self {
        AOCHashMap {
            bins: vec![vec![]; 256]
        }
    }
    fn remove(&mut self, label: &str) {
        let h = hash(label);
        let pos = self.bins[h].iter().position(|(s, _)| s == label);
        match pos {
            Some(idx) => { self.bins[h].remove(idx); },
            None => {}
        };
    }

    fn put(&mut self, label: &str, value: usize) {
        let h = hash(label);
        let pos = self.bins[h].iter().position(|(s, _)| s == label);
        match pos {
            Some(idx) => { self.bins[h][idx] = (label.to_owned(), value); },
            None => { self.bins[h].push((label.to_owned(), value)) }
        };
    }

    fn power(&self) -> usize {
        self.bins.iter().enumerate()
            .flat_map(|(x, v)| v.iter().enumerate().map(move |(y, (_, p))| (x + 1) * (y + 1) * p))
            .sum()
    }
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn test_hash() {
        assert_eq!(EXAMPLE_INPUT.split(",").map(hash).collect::<Vec<_>>(), vec![30, 253, 97, 47, 14, 180, 9, 197, 48, 214, 231]);
    }

    #[test]
    fn example_part1() {
        let instructions = parse_input(EXAMPLE_INPUT);
        assert_eq!(part1(&instructions), 1320);
    }

    #[test]
    fn example_part2() {
        let instructions = parse_input(EXAMPLE_INPUT);
        assert_eq!(part2(&instructions), 145);
    }

    const EXAMPLE_INPUT: &str = "rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7";
}