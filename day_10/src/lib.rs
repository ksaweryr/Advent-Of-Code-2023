/*
WARNING: the code that follows will make you cry:
            A safety pig has been provided for your benefit

                         _
 _._ _..._ .-',     _.._(`))
'-. `     '  /-._.-'    ',/
   )         \            '.
  / _    _    |             \
 |  a    a    /              |
 \   .-.                     ;  
  '-('' ).-'       ,'       ;
     '-;           |      .'
        \           \    /
        | 7  .__  _.-\   \
        | |  |  ``/  /`  /
       /,_|  |   /,_/   /
          /,_/      '`-'
*/
#![feature(iter_intersperse)]
#![feature(let_chains)]

use std::collections::HashSet;
use std::iter::successors;
use std::rc::Rc;
use std::str::FromStr;

use anyhow::Error;

pub fn solve(input: String) {
    let maze = input.parse::<Maze>().expect("Couldn't parse input");

    println!("{}", part1(&maze));
    println!("{}", part2(&maze));
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

fn part2(maze: &Maze) -> usize {
    let (loop_length, inflated) = maze.clone().inflate();
    let outside_nodes_count = flood_fill_count(&inflated);

    (maze.map[0].len() * maze.map.len()) - loop_length - outside_nodes_count
}

#[derive(Clone)]
struct Maze {
    map: Rc<Vec<Vec<char>>>,
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

        Ok(Maze { map: Rc::new(map), current_position, last_position })
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

    /*
        ......
        .F--7.
        .|..|.
        .L--J.
        ......

        ooooooooooooo
        o.o.o.o.o.o.o
        ooooooooooooo
        o.oxxxxxxxo.o
        oooxoooooxooo
        o.oxo.o.oxo.o
        oooxoooooxooo
        o.oxxxxxxxo.o
        ooooooooooooo
        o.o.o.o.o.o.o
        ooooooooooooo
     */
    fn inflate(mut self) -> (usize, Vec<Vec<char>>) {
        let mut loop_elements: HashSet<(usize, usize)> = HashSet::new();
        loop_elements.insert(self.last_position);
        loop_elements.insert(self.current_position);

        while !self.step() {
            loop_elements.insert(self.current_position);
        }

        let w = 2 * self.map[0].len() + 1;
        let mut result: Vec<Vec<char>> = Vec::new();
        result.push("o".chars().cycle().take(w).collect());

        for (y, row) in self.map.iter().enumerate() {
            let mut irow = vec!['o'];
            let mut irow2 = vec!['o'];

            for (x, c1) in row.iter().enumerate() {
                if loop_elements.contains(&(x, y)) {
                    irow.push('x');
                } else {
                    irow.push('.');
                }

                if let Some(c2) = row.get(x + 1) && ['-', 'L', 'F', 'S'].contains(c1) && ['-', 'J', '7', 'S'].contains(c2)
                    && loop_elements.contains(&(x, y)) && loop_elements.contains(&(x + 1, y)) {

                    irow.push('x');
                } else {
                    irow.push('o');
                }

                if let Some(c2) = self.get((x, y + 1)) && ['|', 'F', '7', 'S'].contains(c1) && ['|', 'L', 'J', 'S'].contains(&c2)
                    && loop_elements.contains(&(x, y)) && loop_elements.contains(&(x, y + 1)) {

                    irow2.push('x');
                } else {
                    irow2.push('o');
                }

                irow2.push('o');
            }

            result.push(irow);
            result.push(irow2);
        }

        (loop_elements.len(), result)
    }
}

fn get_elem<T: Clone>(v: &Vec<Vec<T>>, x: usize, y: usize) -> Option<T> {
    v.get(y).and_then(|r| r.get(x)).map(|e| e.clone())
}

fn flood_fill_count(map: &Vec<Vec<char>>) -> usize {
    let mut stack: Vec<(usize, usize)> = Vec::new();
    let mut visited: HashSet<(usize, usize)> = HashSet::new();
    let mut result = 0;

    stack.push((0, 0));
    visited.insert((0, 0));

    while !stack.is_empty() {
        let (x, y) = stack.pop().unwrap();
        if map[y][x] == '.' {
            result += 1;
        }

        for (dx, dy) in [(0, -1), (0, 1), (-1, 0), (1, 0)] {
            if (x == 0 && dx == -1) || (x == map[0].len() - 1 && dx == 1) || (y == 0 && dy == -1) || (y == map.len() - 1 && dy == 1) {
                continue;
            }

            let x1 = (x as isize + dx) as usize;
            let y1 = (y as isize + dy) as usize;

            if map[y1][x1] != 'x' && !visited.contains(&(x1, y1)) {
                visited.insert((x1, y1));
                stack.push((x1, y1));
            }
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn example_part1() {
        let maze = EXAMPLE1_INPUT.parse::<Maze>().unwrap();

        assert_eq!(part1(&maze), 8);
    }

    #[test]
    fn example_part2() {
        let maze = EXAMPLE2_INPUT.parse::<Maze>().unwrap();

        assert_eq!(part2(&maze), 10);
    }

    #[test]
    fn test_inflate() {
        let maze = EXAMPLE_INFLATE.parse::<Maze>().unwrap();
        let result: String = maze.inflate().1.into_iter()
            .map(|r| r.into_iter().collect::<String>())
            .intersperse("\n".to_owned())
            .collect();

        assert_eq!(result, EXAMPLE_INFLATE_RESULT);
    }

    const EXAMPLE1_INPUT: &str = "..F7.
.FJ|.
SJ.L7
|F--J
LJ...";

    const EXAMPLE2_INPUT: &str = "FF7F7F7F7F7F7F7F---7
L|LJS|||||||||||F--J
FL-7LJLJ||||||LJL-77
F--JF--7||LJLJIF7FJ-
L---JF-JLJIIIIFJLJJ7
|F|F-JF---7IIIL7L|7|
|FFJF7L7F-JF7IIL---7
7-L-JL7||F7|L7F-7F7|
L.L7LFJ|||||FJL7||LJ
L7JLJL-JLJLJL--JLJ.L";

    const EXAMPLE_INFLATE: &str = "......
.S--7.
.|..|.
.L--J.
......";

    const EXAMPLE_INFLATE_RESULT: &str = "ooooooooooooo
o.o.o.o.o.o.o
ooooooooooooo
o.oxxxxxxxo.o
oooxoooooxooo
o.oxo.o.oxo.o
oooxoooooxooo
o.oxxxxxxxo.o
ooooooooooooo
o.o.o.o.o.o.o
ooooooooooooo";
}