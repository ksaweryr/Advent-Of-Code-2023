use std::iter::zip;

use anyhow::Error;

pub fn solve(input: String) {
    let races = parse_input(input);
    println!("{}", part1(&races));
    println!("{}", part2(&races));
}

fn part1(races: &Vec<Race>) -> usize {
    races.iter()
        .map(Race::ways_to_win)
        .fold(1, |a, b| a * b)
}

fn part2(races: &Vec<Race>) -> usize {
    races.iter()
        .map(Race::clone)
        .reduce(|r1, r2| Race { time: r1.time * 100 + r2.time, distance: r1.distance * 10000 + r2.distance })
        .unwrap()
        .ways_to_win()
}

fn parse_input(input: String) -> Vec<Race> {
    let (times, distances) = input.split_once("\n").expect("Invalid input format");
    let times = parse_line(times).unwrap();
    let distances = parse_line(distances).unwrap();
    zip(times.into_iter(), distances.into_iter())
        .map(|(time, distance)| Race { time, distance })
        .collect()
}

fn parse_line(line: &str) -> Result<Vec<usize>, Error> {
    line.split_once(": ")
        .ok_or(Error::msg("Invalid line format"))?.1
        .split_whitespace()
        .map(|w| w.parse::<usize>().map_err(Error::new))
        .collect::<Result<Vec<_>, _>>()
}

#[derive(Clone)]
struct Race {
    time: usize,
    distance: usize
}

impl Race {
    fn ways_to_win(&self) -> usize {
        (0..(self.time + 1)).filter(|x| {
            let x = *x as isize;
            -x * x + self.time as isize * x - self.distance as isize > 0
        })
        .count()
    }
}

#[cfg(test)]
mod test {
    use crate::*;

    #[test]
    fn example_part1() {
        let races = parse_input(EXAMPLE_INPUT.to_owned());
        assert_eq!(part1(&races), 288);
    }

    const EXAMPLE_INPUT: &str = "Time:    7  15   30
Distance:    9  40  200";
}