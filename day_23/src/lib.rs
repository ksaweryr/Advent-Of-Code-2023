use std::collections::{HashMap, VecDeque, HashSet};

pub fn solve(input: String) {
    let g = parse_input(&input);

    println!("{}", part1(&g));
}

fn parse_input(input: &str) -> Graph {
    Graph(input.lines().map(|l| l.chars().collect()).collect())
}

fn part1(g: &Graph) -> usize {
    let mut parents = HashMap::<(usize, usize), Vec<(usize, usize)>>::new();
    let mut visited = HashSet::new();
    let mut q = VecDeque::new();

    parents.insert((1, 0), vec![]);
    q.push_back((None, (1, 0)));

    while !q.is_empty() {
        let (pred, pos) = q.pop_front().unwrap();

        if visited.contains(&pos) {
            continue;
        }

        visited.insert(pos);

        for dir in [UP, DOWN, LEFT, RIGHT] {
            if let Some(npos) = g.next(pos, dir) {
                if pred == Some(npos) {
                    continue;
                }
                parents.entry(npos).or_default().push(pos);
                q.push_back((Some(pos), npos));
            }
        }
    }

    max_distance((g.0[0].len() - 2, g.0.len() - 1), &parents, &mut HashMap::new())
}

fn max_distance(pos: (usize, usize), parents: &HashMap<(usize, usize), Vec<(usize, usize)>>, dp: &mut HashMap<(usize, usize), usize>) -> usize {
    if !dp.contains_key(&pos) {
        let v = parents[&pos].iter()
            .map(|&parent| max_distance(parent, parents, dp))
            .max()
            .map(|x| x + 1)
            .unwrap_or(0);
        dp.insert(pos, v);
    }

    dp[&pos]
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Direction {
    UP,
    DOWN,
    LEFT,
    RIGHT
}

use Direction::*;

#[derive(Debug)]
struct Graph(Vec<Vec<char>>);

impl Graph {
    fn next(&self, position: (usize, usize), direction: Direction) -> Option<(usize, usize)> {
        let (x, y, c) = match direction {
            LEFT => if position.0 != 0 {
                Some((position.0 - 1, position.1, '>'))
            } else {
                None
            },
            RIGHT => if position.0 != self.0[0].len() - 1 {
                Some((position.0 + 1, position.1, '<'))
            } else {
                None
            },
            UP => if position.1 != 0 {
                Some((position.0, position.1 - 1, 'v'))
            } else {
                None
            },
            DOWN => if position.1 != self.0.len() - 1 {
                Some((position.0, position.1 + 1, '^'))
            } else {
                None
            }
        }?;

        if self.0[y][x] != '#' && self.0[y][x] != c {
            Some((x, y))
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn example_part1() {
        let g = parse_input(EXAMPLE_INPUT);

        assert_eq!(part1(&g), 94);
    }

    const EXAMPLE_INPUT: &str = "#.#####################
#.......#########...###
#######.#########.#.###
###.....#.>.>.###.#.###
###v#####.#v#.###.#.###
###.>...#.#.#.....#...#
###v###.#.#.#########.#
###...#.#.#.......#...#
#####.#.#.#######.#.###
#.....#.#.#.......#...#
#.#####.#.#.#########v#
#.#...#...#...###...>.#
#.#.#v#######v###.###v#
#...#.>.#...>.>.#.###.#
#####v#.#.###v#.#.###.#
#.....#...#...#.#.#...#
#.#########.###.#.#.###
#...###...#...#...#.###
###.###.#.###v#####v###
#...#...#.#.>.>.#.>.###
#.###.###.#.###.#.#v###
#.....###...###...#...#
#####################.#";
}