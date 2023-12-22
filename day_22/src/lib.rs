use std::{str::FromStr, iter::repeat, collections::HashSet};

use anyhow::Error;

pub fn solve(input: String) {
    let bricks = parse_input(&input);

    let result = settle(bricks);

    println!("{}", result.0);
    println!("{}", result.1);
}

fn parse_input(input: &str) -> Vec<Brick> {
    input.lines().map(|l| l.parse()).collect::<Result<Vec<_>, _>>().expect("Invalid input")
}

fn settle(mut bricks: Vec<Brick>) -> (usize, usize) {
    let max_x = bricks.iter().map(|b| b.1.x).max().unwrap();
    let max_y = bricks.iter().map(|b| b.1.y).max().unwrap();
    let mut brickmap: Vec<Vec<Option<usize>>> = vec![vec![None; max_x + 1]; max_y + 1];
    let mut settled: Vec<Brick> = Vec::new();
    let mut supporting_bricks = HashSet::new();
    let mut supported_by: Vec<HashSet<usize>> = vec![];

    bricks.sort();

    for (i, Brick(Cube { x: x1, y: y1, z: z1 }, Cube { x: x2, y: y2, z: z2 })) in bricks.into_iter().enumerate() {
        let new_z = (y1..y2 + 1)
            .flat_map(|y| (x1..x2 + 1)
                .zip(repeat(y))
                .map(|(x, y)| brickmap[y][x].map(|idx| settled[idx].1.z).unwrap_or(0)))
            .max().unwrap() + 1;

        let delta = z1 - new_z;

        settled.push(Brick(Cube::new(x1, y1, new_z), Cube::new(x2, y2, z2 - delta)));

        let mut supporters = HashSet::new();

        for y in y1..y2 + 1 {
            for x in x1..x2 + 1 {
                if let Some(idx) = brickmap[y][x] {
                    if settled[idx].1.z == new_z - 1 {
                        supporters.insert(idx);
                    }
                }

                brickmap[y][x] = Some(i);
            }
        }

        if supporters.len() == 1 {
            supporting_bricks.insert(*supporters.iter().last().unwrap());
        }

        supported_by.push(supporters);
    }

    let safe_to_disintegrate = settled.len() - supporting_bricks.len();

    let mut total_part2 = 0;

    for i in 0..settled.len() {
        let mut deleted = HashSet::new();
        deleted.insert(i);

        for j in i+1..settled.len() {
            if supported_by[j].len() != 0 && supported_by[j].is_subset(&deleted) {
                deleted.insert(j);
            }
        }

        total_part2 += deleted.len() - 1;
    }

    (safe_to_disintegrate, total_part2)
}

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
struct Cube {
    z: usize,
    y: usize,
    x: usize
}

impl Cube {
    fn new(x: usize, y: usize, z: usize) -> Self {
        Cube { x, y, z }
    }
}

impl FromStr for Cube {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let [x, y, z] = s.split(",")
            .map(|x| x.parse::<usize>())
            .collect::<Result<Vec<_>, _>>()?
            .try_into()
            .map_err(|_| Error::msg("Invalid cube format"))?;
        
        Ok(Cube { x, y, z})
    }
}

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
struct Brick(Cube, Cube);

impl FromStr for Brick {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (p1, p2) = s.split_once("~")
            .ok_or(Error::msg("Invalid brick format"))?;

        Ok(Brick(p1.parse()?, p2.parse()?))
    }
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn example_part1() {
        let bricks = parse_input(EXAMPLE_INPUT);

        assert_eq!(settle(bricks).0, 5);
    }

    #[test]
    fn example_part2() {
        let bricks = parse_input(EXAMPLE_INPUT);

        assert_eq!(settle(bricks).1, 7);
    }

    const EXAMPLE_INPUT: &str = "1,0,1~1,2,1
0,0,2~2,0,2
0,2,3~2,2,3
0,0,4~0,2,4
2,0,5~2,2,5
0,1,6~2,1,6
1,1,8~1,1,9";
}