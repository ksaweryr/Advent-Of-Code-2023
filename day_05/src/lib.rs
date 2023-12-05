use std::collections::VecDeque;
use std::iter;
use std::str::FromStr;

use anyhow::Error;

pub fn solve(input: String) {
    let (seeds, maps) = parse_input(input);

    println!("{}", part1(&seeds, &maps));
    println!("{}", part2(&seeds, &maps));
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

fn part2(seeds: &Vec<usize>, maps: &Vec<RangeMap>) -> usize {
    let seed_ranges = RangeMap {
        map: seeds
        .windows(2)
        .enumerate()
        .filter(|(idx, _)| idx % 2 == 0)
        .map(|(_, x)| Range { source_start: x[0], target_start: x[0], length: x[1] })
        .collect()
    };

    maps.iter()
        .fold(seed_ranges, |acc, m| acc.flatten(m))
        .map
        .iter()
        .map(|r| r.target_start)
        .min().unwrap()
}

#[derive(Clone, Debug, PartialEq)]
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

    fn contains_range(&self, other: &Self) -> bool {
        self.contains_range_start(other) && self.contains_range_end(other)
    }

    fn contains_range_start(&self, other: &Self) -> bool {
        self.source_start <= other.target_start && self.source_start + self.length >= other.target_start
    }

    fn contains_range_end(&self, other: &Self) -> bool {
        self.source_start <= other.target_start + other.length && self.source_start + self.length >= other.target_start + other.length
    }

    fn is_contained(&self, other: &Self) -> bool {
        other.target_start <= self.source_start && other.target_start + other.length >= self.source_start + self.length
    }
    
    fn merge(&self, other: &Self) -> RangeMergeResult {
        if self.contains_range(other) {
            let merged = Range {
                source_start: other.source_start,
                target_start: other.target_start + self.target_start - self.source_start,
                length: other.length
            };
            RangeMergeResult {
                merged: Some(merged),
                unused_sources: Box::new(iter::empty())
            }
        } else if self.contains_range_start(other) {
            let merged_length = self.source_start + self.length - other.target_start;
            let merged = Range {
                source_start: other.source_start,
                target_start: other.target_start + self.target_start - self.source_start,
                length: merged_length
            };
            let unused = Range {
                source_start: other.source_start + merged_length,
                target_start: other.target_start + merged_length,
                length: other.length - merged_length
            };
            RangeMergeResult {
                merged: Some(merged),
                unused_sources: Box::new(iter::once(unused))
            }
        } else if self.contains_range_end(other) {
            let merged_length = other.target_start + other.length - self.source_start;
            let merged = Range {
                source_start: other.source_start + other.length - merged_length,
                target_start: self.target_start,
                length: merged_length
            };
            let unused = Range {
                source_start: other.source_start,
                target_start: other.target_start,
                length: other.length - merged_length
            };
            RangeMergeResult {
                merged: Some(merged),
                unused_sources: Box::new(iter::once(unused))
            }
        } else if self.is_contained(&other) {
            let prefix_length = self.source_start - other.target_start;
            let prefix = Range {
                source_start: other.source_start,
                target_start: other.target_start,
                length: prefix_length
            };
            let suffix_length = other.target_start + other.length - (self.source_start + self.length);
            let suffix = Range {
                source_start: other.source_start + other.length - suffix_length,
                target_start: other.target_start + other.length - suffix_length,
                length: suffix_length
            };
            let merged = Range {
                source_start: other.source_start + prefix_length,
                target_start: self.target_start,
                length: other.length - prefix_length - suffix_length
            };
            RangeMergeResult {
                merged: Some(merged),
                unused_sources: Box::new(iter::once(prefix).chain(iter::once(suffix)))
            }
        } else {
            RangeMergeResult {
                merged: None,
                unused_sources: Box::new(iter::once(other.clone()))
            }
        }
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

struct RangeMergeResult {
    merged: Option<Range>,
    unused_sources: Box<dyn Iterator<Item=Range>>
}

#[derive(Clone, Debug, PartialEq)]
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

    fn flatten(&self, next_level: &RangeMap) -> RangeMap {
        let mut queue = VecDeque::<Range>::new();
        let mut result: Vec<Range> = vec![];

        for prev in &self.map {
            queue.push_back(prev.clone());
        }

        while !queue.is_empty() {
            let prev = queue.pop_front().unwrap();
            let mut found = false;

            for next in &next_level.map {
                let partial_result = next.merge(&prev);

                match partial_result.merged {
                    Some(r) => {
                        if r.length != 0 {
                            result.push(r);
                            partial_result.unused_sources.for_each(|r| queue.push_back(r));
                            found = true;
                            break;
                        }
                    },
                    None => {}
                }
            }

            if !found {
                result.push(prev);
            }
        }

        RangeMap { map: result }
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
    use crate::*;

    #[test]
    fn example_part1() {
        let (seeds, maps) = parse_input(EXAMPLE_INPUT.to_owned());
        assert_eq!(part1(&seeds, &maps), 35);
    }

    #[test]
    fn example_part2() {
        let (seeds, maps) = parse_input(EXAMPLE_INPUT.to_owned());
        assert_eq!(part2(&seeds, &maps), 46);
    }

    #[test]
    fn merge() {
        let source_range = Range { source_start: 0, target_start: 10, length: 5 };

        // next contains prev
        let target_range = Range { source_start: 10, target_start: 20, length: 5 };
        let mut merged = target_range.merge(&source_range);
        assert_eq!(merged.merged, Some(Range { source_start: 0, target_start: 20, length: 5 }));
        assert_eq!(merged.unused_sources.next(), None);

        // next contains start of prev
        let target_range = Range { source_start: 7, target_start: 20, length: 5 };
        let mut merged = target_range.merge(&source_range);
        assert_eq!(merged.merged, Some(Range { source_start: 0, target_start: 23, length: 2 }));
        assert_eq!(merged.unused_sources.next(), Some(Range { source_start: 2, target_start: 12, length: 3 }));

        // next contains end of prev
        let target_range = Range { source_start: 11, target_start: 20, length: 7 };
        let mut merged = target_range.merge(&source_range);
        assert_eq!(merged.merged, Some(Range { source_start: 1, target_start: 20, length: 4 }));
        assert_eq!(merged.unused_sources.next(), Some(Range { source_start: 0, target_start: 10, length: 1 }));

        // prev contains next
        let target_range = Range { source_start: 12, target_start: 20, length: 2 };
        let mut merged = target_range.merge(&source_range);
        assert_eq!(merged.merged, Some(Range { source_start: 2, target_start: 20, length: 2 }));
        assert_eq!(merged.unused_sources.next(), Some(Range { source_start: 0, target_start: 10, length: 2 }));
        assert_eq!(merged.unused_sources.next(), Some(Range { source_start: 4, target_start: 14, length: 1 }));

        // next and prev are disjoint
        let target_range = Range { source_start: 20, target_start: 30, length: 5 };
        let mut merged = target_range.merge(&source_range);
        assert_eq!(merged.merged, None);
        assert_eq!(merged.unused_sources.next(), Some(source_range));
    }

    #[test]
    fn flatten() {
        let map1 = RangeMap {
            map: vec![
                Range { source_start: 0, target_start: 10, length: 5 },
                Range { source_start: 100, target_start: 200, length: 10 }
            ]
        };

        let map2 = RangeMap {
            map: vec![
                Range { source_start: 10, target_start: 20, length: 3},
                Range { source_start: 200, target_start: 300, length: 10 }
            ]
        };

        let map3 = map1.flatten(&map2);
        let expected = RangeMap {
            map: vec![
                Range { source_start: 0, target_start: 20, length: 3 },
                Range { source_start: 100, target_start: 300, length: 10 },
                Range { source_start: 3, target_start: 13, length: 2 }
            ]
        };

        assert_eq!(map3, expected);
    }

    const EXAMPLE_INPUT: &str = "seeds: 79 14 55 13

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