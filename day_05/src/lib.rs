use std::str::FromStr;

use anyhow::Error;

pub fn solve(input: String) {
    let (seeds, maps) = parse_input(input);

    println!("{}", part1(&seeds, &maps));
}

fn parse_input(input: String) -> (Vec<usize>, Vec<RangeMap>) {
    let (seeds, maps) = input.split_once("\n\n").expect("Invalid input format");
    let seeds = seeds.split_once(": ").expect("Invalid seeds format")
        .1.split(" ")
        .map(|s| s.parse::<usize>())
        .collect::<Result<Vec<_>, _>>().expect("Invalid seed value");

    let maps = maps.split("\n\n")
        .map(|m| m.parse::<RangeMap>())
        .collect::<Result<Vec<_>, _>>().expect("Invalid map format");

    (seeds, maps)
}

fn part1(seeds: &Vec<usize>, maps: &Vec<RangeMap>) -> usize {
    seeds.iter().map(|s| maps.iter().fold(*s, |acc, m| m.destination(acc))).min().unwrap()
}

struct Range {
    target_start: usize,
    source_start: usize,
    length: usize
}


impl Range {
    fn contains(&self, value: usize) -> bool {
        value >= self.source_start && value < self.source_start + self.length
    }

    fn position(&self, value: usize) -> Option<usize> {
        if self.contains(value) {
            Some(value - self.source_start)
        } else {
            None
        }
    }

    fn target(&self, value: usize) -> Option<usize> {
        self.position(value).map(|pos| self.target_start + pos)
    }
}

impl FromStr for Range {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let [target_start, source_start, length]: [usize; 3] = s
            .split(" ")
            .map(|w| w.parse::<usize>())
            .collect::<Result<Vec<usize>, _>>()?
            .try_into()
            .map_err(|_| Error::msg("Invalid length of map entry"))?;

        Ok(Range { target_start, source_start, length })
    }
}

struct RangeMap {
    map: Vec<Range>
}

impl RangeMap {
    fn destination(&self, value: usize) -> usize {
        self.map
            .iter()
            .find_map(|range| range.target(value))
            .unwrap_or(value)
    }
}

impl FromStr for RangeMap {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(RangeMap {
            map: s.lines()
                .skip(1)
                .map(|line| line.parse::<Range>())
                .collect::<Result<Vec<_>, _>>()?
        })
    }
}

#[cfg(test)]
mod test {
    use crate::{part1, parse_input};

    #[test]
    fn example_part1() {
        let (seeds, maps) = parse_input(example_input.to_owned());
        assert_eq!(part1(&seeds, &maps), 35);
    }

    const example_input: &str = "seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4";
}