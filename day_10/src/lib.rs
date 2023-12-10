#![feature(let_chains)]

use std::iter::successors;
use std::str::FromStr;

use anyhow::Error;

pub fn solve(input: String) {
    let maze = input.parse::<Maze>().expect("Couldn't parse input");

    println!("{}", part1(&maze));
}

fn part1(maze: &Maze) -> usize {
    (successors(Some(2usize), |x| Some(x + 1)).scan(maze.clone(), |state, x| {
        if state.step() {
            None
        } else {
            Some(x)
        }
    }).last().unwrap() + 1) / 2
}

#[derive(Clone)]
struct Maze {
    map: Vec<Vec<char>>,
    current_position: (usize, usize),
    last_position: (usize, usize)
}

impl FromStr for Maze {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let map: Vec<Vec<char>> = s.lines().map(|l| l.chars().collect()).collect();
        let last_position@(x, y) = s.lines().enumerate()
            .find_map(|(y, l)| l.chars().enumerate()
                .find_map(|(x, c)| if c == 'S' { Some((x, y)) } else { None }))
            .ok_or(Error::msg("No starting position in the map"))?;

        let current_position = (if let Some(c) = get_elem(&map, x, y - 1) && ['|', '7', 'F'].contains(&c) {
            Ok((x, y - 1))
        } else if let Some(c) = get_elem(&map, x, y + 1) && ['|', 'L', 'J'].contains(&c) {
            Ok((x, y + 1))
        } else if let Some(c) = get_elem(&map, x - 1, y) && ['-', 'L', 'F'].contains(&c) {
            Ok((x - 1, y))
        } else if let Some(c) = get_elem(&map, x + 1, y) && ['-', 'J', '7'].contains(&c) {
            Ok((x + 1, y))
        } else {
            Err(Error::msg("No valid path found"))
        })?;

        Ok(Maze { map, current_position, last_position })
    }
}

impl Maze {
    fn get(&self, position: (usize, usize)) -> Option<char> {
        let (x, y) = position;
        get_elem(&self.map, x, y)
    }

    fn step(&mut self) -> bool {
        let (x, y) = self.current_position;
        let moves = match self.get(self.current_position).unwrap() {
            '|' => [(x, y - 1), (x,  y + 1)],
            '-' => [(x - 1, y), (x + 1, y)],
            'L' => [(x, y - 1), (x + 1, y)],
            'J' => [(x, y - 1), (x - 1, y)],
            '7' => [(x, y + 1), (x - 1, y)],
            'F' => [(x, y + 1), (x + 1, y)],
            _   => panic!("Invalid character {} at position {:?}", self.get(self.current_position).unwrap(), self.current_position)
        };

        for m in moves {
            if m != self.last_position {
                self.last_position = self.current_position;
                self.current_position = m;
                break;
            }
        }

        self.get(self.current_position).unwrap() == 'S'
    }
}

fn get_elem<T: Clone>(v: &Vec<Vec<T>>, x: usize, y: usize) -> Option<T> {
    v.get(y).and_then(|r| r.get(x)).map(|e| e.clone())
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn example1_part1() {
        let maze = EXAMPLE1_INPUT.parse::<Maze>().unwrap();

        assert_eq!(part1(&maze), 8);
    }

    #[test]
    fn example2_part1() {
        let maze = EXAMPLE2_INPUT.parse::<Maze>().unwrap();

        assert_eq!(part1(&maze), 4);
    }

    const EXAMPLE1_INPUT: &str = "..F7.
.FJ|.
SJ.L7
|F--J
LJ...";

    const EXAMPLE2_INPUT: &str = ".....
.S-7.
.|.|.
.L-J.
.....";
}