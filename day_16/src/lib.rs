use std::collections::HashSet;

pub fn solve(input: String) {
    let contraption = parse_input(&input);

    println!("{}", part1(&contraption));
    println!("{}", part2(&contraption));
}

fn parse_input(input: &str) -> Contraption {
    Contraption {
        map: input.lines().map(|l| l.chars().collect()).collect()
    }
}

fn part1(contraption: &Contraption) -> usize {
    contraption.simulate_beam(Default::default())
}

fn part2(contraption: &Contraption) -> usize {
    let w = contraption.map[0].len();
    let h = contraption.map.len();

    (0..w).map(|x| contraption.simulate_beam(Beam { position: (x, 0), direction: DOWN })).max().unwrap().max(
        (0..w).map(|x| contraption.simulate_beam(Beam { position: (x, h - 1), direction: UP })).max().unwrap()
    ).max(
        (0..h).map(|y| contraption.simulate_beam(Beam { position: (0, y), direction: RIGHT })).max().unwrap()
    ).max(
        (0..h).map(|y| contraption.simulate_beam(Beam { position: (w - 1, y), direction: LEFT })).max().unwrap()
    )
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
enum Direction {
    UP,
    DOWN,
    LEFT,
    RIGHT
}

use Direction::*;

impl Direction {
    fn next(&self) -> Self {
        match self {
            UP => RIGHT,
            RIGHT => DOWN,
            DOWN => LEFT,
            LEFT => UP
        }
    }

    fn prev(&self) -> Self {
        match self {
            UP => LEFT,
            LEFT => DOWN,
            DOWN => RIGHT,
            RIGHT => UP
        }
    }

    fn is_horizontal(&self) -> bool {
        self == &LEFT || self == &RIGHT
    }

    fn is_vertical(&self) -> bool {
        !self.is_horizontal()
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
struct Beam {
    position: (usize, usize),
    direction: Direction
}

impl Default for Beam {
    fn default() -> Self {
        Self { position: (0, 0), direction: RIGHT }
    }
}

impl Beam {
    fn can_continue(&self, w: usize, h: usize) -> bool {
        !((self.position.0 == 0 && self.direction == LEFT) ||
        (self.position.0 == w - 1 && self.direction == RIGHT) ||
        (self.position.1 == 0 && self.direction == UP) ||
        (self.position.1 == h - 1 && self.direction == DOWN))
    }

    fn step(&mut self) {
        match self.direction {
            LEFT => self.position.0 -= 1,
            RIGHT => self.position.0 += 1,
            UP => self.position.1 -= 1,
            DOWN => self.position.1 += 1
        }
    }

    fn mirror(&mut self, mirror: char) {
        if (mirror == '/' && self.direction.is_vertical()) || (mirror == '\\' && self.direction.is_horizontal()) {
            self.direction = self.direction.next();
        } else {
            self.direction = self.direction.prev();
        }
    }

    fn split(&self, splitter: char) -> Option<Vec<Beam>> {
        if (splitter == '-' && self.direction.is_horizontal()) || (splitter == '|' && self.direction.is_vertical()) {
            None
        } else {
            Some(vec![
                Beam { position: self.position, direction: self.direction.next() },
                Beam { position: self.position, direction: self.direction.prev() }
            ])
        }
    }
}

#[derive(Debug)]
struct Contraption {
    map: Vec<Vec<char>>
}

impl Contraption {
    fn simulate_beam(&self, beam: Beam) -> usize {
        let w = self.map[0].len();
        let h = self.map.len();
        let mut energized_tiles = HashSet::new();
        let mut visited_states: HashSet<Beam> = HashSet::new();
        let mut beam = beam;

        let c = self.map[beam.position.1][beam.position.0];

        if c == '/' || c == '\\' {
            beam.mirror(c);
        }

        let mut beams: Vec<Beam> = if c == '|' || c == '-' {
            beam.split(c).unwrap_or(vec![beam])
        } else {
            vec![beam]
        };

        energized_tiles.insert(beam.position);
        visited_states.insert(beam);

        while !beams.is_empty() {
            let mut beam = beams.pop().unwrap();

            while beam.can_continue(w, h) {
                beam.step();

                if visited_states.contains(&beam) {
                    break;
                }

                energized_tiles.insert(beam.position);
                visited_states.insert(beam);

                let c = self.map[beam.position.1][beam.position.0];
                if match c {
                    '/' | '\\' => {
                        beam.mirror(c);
                        false
                    },
                    '|' | '-' => {
                        let mut new_beams = beam.split(c);

                        match &mut new_beams {
                            None => false,
                            Some(v) => {
                                beams.append(v);
                                true
                            }
                        }
                    },
                    _ => false
                } {
                    break
                }
            }
        }

        energized_tiles.len()
    }
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn example_part1() {
        let contraption = parse_input(EXAMPLE_INPUT);

        assert_eq!(part1(&contraption), 46);
    }

    #[test]
    fn example_part2() {
        let contraption = parse_input(EXAMPLE_INPUT);

        assert_eq!(part2(&contraption), 51);
    }

    const EXAMPLE_INPUT: &str = r#".|...\....
|.-.\.....
.....|-...
........|.
..........
.........\
..../.\\..
.-.-/..|..
.|....-|.\
..//.|...."#;
}