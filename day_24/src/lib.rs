use std::str::FromStr;

use anyhow::Error;

pub fn solve(input: String) {
    let hailstones = parse_input(&input);

    println!("{}", part1(&hailstones, (200000000000000.0, 400000000000000.0)));
}

fn parse_input(input: &str) -> Vec<Hailstone> {
    input.lines()
        .map(|l| l.parse())
        .collect::<Result<Vec<_>, _>>()
        .expect("Invalid input")
}

fn part1(hailstones: &Vec<Hailstone>, (lbound, hbound): (f64, f64)) -> usize {
    hailstones.iter().enumerate()
        .map(|(i, &h)| hailstones.iter()
            .skip(i + 1)
            .filter_map(|&h2| h.intersection_point(h2))
            .filter(|&(x, y)| x >= lbound && x <= hbound && y >= lbound && y <= hbound)
            .count())
        .sum()
}

#[derive(Clone, Copy, Debug)]
struct Line2D {
    // ax + by = c
    a: f64,
    b: f64,
    c: f64
}

impl Line2D {
    fn intersection_point(self, other: Line2D) -> Option<(f64, f64)> {
        let w = self.a * other.b - other.a * self.b;

        if w == 0.0 {
            None
        } else {
            let wx = other.b * self.c - self.b * other.c;
            let wy = self.a * other.c - other.a * self.c;

            Some((wx / w, wy / w))
        }
    }
}

#[derive(Clone, Copy, Debug)]
struct Hailstone {
    position: (isize, isize, isize),
    velocity: (isize, isize, isize)
}

impl Hailstone {
    fn intersection_point(self, other: Hailstone) -> Option<(f64, f64)> {
        match <Hailstone as Into<Line2D>>::into(self).intersection_point(other.into()) {
            res@Some((x, _)) => {
                if (x - self.position.0 as f64).signum() == self.velocity.0.signum() as f64
                    && (x - other.position.0 as f64).signum() == other.velocity.0.signum() as f64 {
                    res
                } else {
                    None
                }
            },
            None => None
        }
    }
}

impl Into<Line2D> for Hailstone {
    fn into(self) -> Line2D {
        let a = self.velocity.1 as f64;
        let b = -self.velocity.0 as f64;
        let c = a * self.position.0 as f64 + b * self.position.1 as f64;

        Line2D { a, b, c }
    }
}

impl FromStr for Hailstone {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (position, velocity) = s.split_once(" @ ").ok_or(Error::msg("Invalid format"))?;

        let position = parse_3tuple(position)?;
        let velocity = parse_3tuple(velocity)?;

        Ok(Hailstone { position, velocity })
    }
}

fn parse_3tuple(s: &str) -> Result<(isize, isize, isize), Error> {
    let [a, b, c] = s.split(", ")
        .map(|x| x.trim().parse())
        .collect::<Result<Vec<_>, _>>()?
        .try_into()
        .map_err(|_| Error::msg("Invalid format"))?;

    Ok((a, b, c))
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn example_part1() {
        let hailstones = parse_input(EXAMPLE_INPUT);

        assert_eq!(part1(&hailstones, (7.0, 27.0)), 2);
    }

    const EXAMPLE_INPUT: &str = "19, 13, 30 @ -2,  1, -2
18, 19, 22 @ -1, -1, -2
20, 25, 34 @ -2, -2, -4
12, 31, 28 @ -1, -2, -1
20, 19, 15 @  1, -5, -3";
}