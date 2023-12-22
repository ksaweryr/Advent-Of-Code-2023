use std::{str::FromStr, iter::repeat, collections::HashSet};

use anyhow::Error;

pub fn solve(input: String) {
    let bricks = parse_input(&input);

    let (settled, safe_to_disintegrate) = settle(bricks);

    println!("{}", safe_to_disintegrate);
}

fn parse_input(input: &str) -> Vec<Brick> {
    input.lines().map(|l| l.parse()).collect::<Result<Vec<_>, _>>().expect("Invalid input")
}

fn settle(mut bricks: Vec<Brick>) -> (Vec<Brick>, usize) {
    let max_x = bricks.iter().map(|b| b.1.x).max().unwrap();
    let max_y = bricks.iter().map(|b| b.1.y).max().unwrap();
    let mut brickmap: Vec<Vec<Option<usize>>> = vec![vec![None; max_x + 1]; max_y + 1];
    let mut result: Vec<Brick> = Vec::new();
    let mut supporting_bricks = HashSet::new();

    bricks.sort();

    for (i, Brick(Cube { x: x1, y: y1, z: z1 }, Cube { x: x2, y: y2, z: z2 })) in bricks.into_iter().enumerate() {
        let new_z = (y1..y2 + 1)
            .flat_map(|y| (x1..x2 + 1)
                .zip(repeat(y))
                .map(|(x, y)| brickmap[y][x].map(|idx| result[idx].1.z).unwrap_or(0)))
            .max().unwrap() + 1;

        let delta = z1 - new_z;

        result.push(Brick(Cube::new(x1, y1, new_z), Cube::new(x2, y2, z2 - delta)));

        let mut indices = HashSet::new();

        for y in y1..y2 + 1 {
            for x in x1..x2 + 1 {
                if let Some(idx) = brickmap[y][x] {
                    if result[idx].1.z == new_z - 1 {
                        indices.insert(idx);
                    }
                }

                brickmap[y][x] = Some(i);
            }
        }

        if indices.len() == 1 {
            supporting_bricks.insert(indices.into_iter().last().unwrap());
        }
    }

    let safe_to_disintegrate = result.len() - supporting_bricks.len();

    (result, safe_to_disintegrate)
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

        assert_eq!(settle(bricks).1, 5);
    }

    const EXAMPLE_INPUT: &str = "1,0,1~1,2,1
0,0,2~2,0,2
0,2,3~2,2,3
0,0,4~0,2,4
2,0,5~2,2,5
0,1,6~2,1,6
1,1,8~1,1,9";
}