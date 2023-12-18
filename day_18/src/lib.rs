#![feature(iter_map_windows)]

use std::str::FromStr;

use anyhow::Error;

pub fn solve(input: String) {
    let plan = parse_input(&input);

    println!("{}", part1(&plan));
    println!("{}", part2(&plan));
}

fn parse_input(input: &str) -> Vec<PlanPart> {
    input.lines().map(|x| x.parse::<PlanPart>().expect("Invalid input")).collect()
}

fn part1(plan: &Vec<PlanPart>) -> isize {
    let boundary_cells = plan.iter().map(|p| p.distance).sum::<isize>();
    let vertices = plan.iter().scan((0, 0), |state, p| {
        *state = p.direction.move_from(state, p.distance);
        Some(*state)
    }).collect::<Vec<(isize, isize)>>();
    let vertex_count = vertices.len() as isize;
    let concave_vertices = (vertex_count - 4) / 2;

    shoelace(&vertices) + (concave_vertices + (vertex_count - concave_vertices) * 3) / 4 + (boundary_cells - vertex_count) / 2
}

fn part2(plan: &Vec<PlanPart>) -> isize {
    let boundary_cells = plan.iter().map(|p| p.rgb.distance()).sum::<isize>();
    let vertices = plan.iter().scan((0, 0), |state, p| {
        *state = p.rgb.direction().move_from(state, p.rgb.distance());
        Some(*state)
    }).collect::<Vec<(isize, isize)>>();
    let vertex_count = vertices.len() as isize;
    let concave_vertices = (vertex_count - 4) / 2;

    shoelace(&vertices) + (concave_vertices + (vertex_count - concave_vertices) * 3) / 4 + (boundary_cells - vertex_count) / 2
}

fn shoelace(points: &Vec<(isize, isize)>) -> isize {
    (points.iter()
        .map_windows(|[a, b]| shoelace_part(a, b))
        .sum::<isize>() + shoelace_part(points.last().unwrap(), &points[0])).abs() / 2
}

fn shoelace_part((x1, y1): &(isize, isize), (x2, y2): &(isize, isize)) -> isize {
    (y1 + y2) * (x2 - x1)
}

#[derive(Debug, Eq, PartialEq)]
enum Direction {
    UP,
    DOWN,
    LEFT,
    RIGHT
}

use Direction::*;

impl Direction {
    fn move_from(&self, (x, y): &(isize, isize), distance: isize) -> (isize, isize) {
        match self {
            UP => (*x, y - distance),
            DOWN => (*x, y + distance),
            LEFT => (x - distance, *y),
            RIGHT => (x + distance, *y)
        }
    }
}

impl FromStr for Direction {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() == 1 {
            match s.chars().nth(0).unwrap() {
                'U' => Ok(UP),
                'D' => Ok(DOWN),
                'L' => Ok(LEFT),
                'R' => Ok(RIGHT),
                _ => Err(Error::msg(format!("`{}` is not a valid direction", s)))
            }
        } else {
            Err(Error::msg(format!("`{}` is not a valid direction", s)))
        }
    }
}

#[derive(Debug)]
struct Colour {
    r: usize,
    g: usize,
    b: usize
}

impl Colour {
    fn distance(&self) -> isize {
        ((self.r * 256 * 256 + self.g * 256 + self.b) / 16) as isize
    }
    
    fn direction(&self) -> Direction {
        match self.b % 16 {
            0 => RIGHT,
            1 => DOWN,
            2 => LEFT,
            3 => UP,
            _ => panic!("Invalid colour")
        }
    }
}

impl FromStr for Colour {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() != 9 || &s[..2] != "(#" || s.chars().last().unwrap() != ')' {
            return Err(Error::msg("Invalid colour format"));
        }

        let val = s[2..8].chars()
            .map(|c| c.to_digit(16).ok_or(Error::msg("Invalid hexadecimal digit")))
            .fold(Ok(0), |acc, d| acc.map(|a| a * 16).and_then(|a| d.map(|b| a + b)))? as usize;
        let r = val / (256 * 256);
        let g = val / 256 % 256;
        let b = val % 256;

        Ok(Colour { r, g, b })
    }
}

#[derive(Debug)]
struct PlanPart {
    direction: Direction,
    distance: isize,
    rgb: Colour
}

impl FromStr for PlanPart {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let [a, b, c] = s.split(" ").collect::<Vec<&str>>().try_into().map_err(|_| Error::msg("Invalid plan format"))?;

        let direction = a.parse::<Direction>()?;
        let distance = b.parse::<isize>()?;
        let rgb = c.parse::<Colour>()?;

        Ok(PlanPart { direction, distance, rgb })
    }
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn example_part1() {
        let plan = parse_input(EXAMPLE_INPUT);
        assert_eq!(part1(&plan), 62);
    }

    #[test]
    fn example_part2() {
        let plan = parse_input(EXAMPLE_INPUT);
        assert_eq!(part2(&plan), 952408144115);
    }

    const EXAMPLE_INPUT: &str = "R 6 (#70c710)
D 5 (#0dc571)
L 2 (#5713f0)
D 2 (#d2c081)
R 2 (#59c680)
D 2 (#411b91)
L 5 (#8ceee2)
U 2 (#caa173)
L 1 (#1b58a2)
U 2 (#caa171)
R 2 (#7807d2)
U 3 (#a77fa3)
L 2 (#015232)
U 2 (#7a21e3)";
}