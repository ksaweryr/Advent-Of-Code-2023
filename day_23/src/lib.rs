use std::{collections::{HashMap, VecDeque, HashSet}, cell::RefCell};

pub fn solve(input: String) {
    let g = parse_input(&input);

    println!("{}", part1(&g));
    println!("{}", part2(&g));
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

        for npos in g.neighbours(pos, true) {
            if pred == Some(npos) {
                continue;
            }
            parents.entry(npos).or_default().push(pos);
            q.push_back((Some(pos), npos));
        }
    }

    max_distance1((g.0[0].len() - 2, g.0.len() - 1), &parents, &mut HashMap::new())
}

fn max_distance1(pos: (usize, usize), parents: &HashMap<(usize, usize), Vec<(usize, usize)>>, dp: &mut HashMap<(usize, usize), usize>) -> usize {
    if !dp.contains_key(&pos) {
        let v = parents[&pos].iter()
            .map(|&parent| max_distance1(parent, parents, dp))
            .max()
            .map(|x| x + 1)
            .unwrap_or(0);
        dp.insert(pos, v);
    }

    dp[&pos]
}

fn part2(g: &Graph) -> usize {
    let mut adj_list = HashMap::<(usize, usize), HashSet<Edge>>::new();
    let mut visited = HashSet::new();
    let mut q = VecDeque::new();

    q.push_back(Extender { source: (1, 0), prev: (1, 0), dist: 1, curr: (1, 1) });

    while !q.is_empty() {
        let Extender { source, prev, dist, curr } = q.pop_front().unwrap();
        let neighbours = g.neighbours(curr, false);

        if neighbours.len() == 2 {
            let next = neighbours.into_iter().filter(|&n| n != prev).last().unwrap();
            q.push_back(Extender { source, prev: curr, dist: dist + 1, curr: next });
        } else {
            adj_list.entry(source).or_default().insert(Edge { target: curr, dist });
            adj_list.entry(curr).or_default().insert(Edge { target: source, dist });
            if !visited.contains(&curr) {
                neighbours.into_iter()
                    .filter(|&n| n != prev)
                    .for_each(|n| q.push_back(Extender { source: curr, prev: curr, dist: 1, curr: n}));
                visited.insert(curr);
            }
        }
    }

    max_distance2((1, 0), (g.0[0].len() - 2, g.0.len() - 1), &adj_list, RefCell::new(HashSet::new())).unwrap()
}

fn max_distance2(from: (usize, usize), to: (usize, usize), adj_list: &HashMap<(usize, usize), HashSet<Edge>>, visited: RefCell<HashSet<(usize, usize)>>) -> Option<usize> {
    if from == to {
        return Some(0);
    }

    visited.borrow_mut().insert(from);

    let result = adj_list[&from].iter()
        .filter(|Edge { target, .. }| !visited.borrow().contains(target))
        .filter_map(|&Edge { target, dist }| 
            max_distance2(target, to, adj_list, visited.clone()).map(|x| x + dist))
        .max();

    visited.borrow_mut().remove(&from);

    result
}

#[derive(Debug, Eq, Hash, PartialEq)]
struct Edge {
    target: (usize, usize),
    dist: usize
}

struct Extender {
    source: (usize, usize),
    prev: (usize, usize),
    dist: usize,
    curr: (usize, usize)
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
    fn next(&self, position: (usize, usize), direction: Direction, part_1: bool) -> Option<(usize, usize)> {
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

        if self.0[y][x] != '#' && (!part_1 || self.0[y][x] != c) {
            Some((x, y))
        } else {
            None
        }
    }

    fn neighbours(&self, position: (usize, usize), part_1: bool) -> Vec<(usize, usize)> {
        [UP, DOWN, LEFT, RIGHT].into_iter()
            .filter_map(|dir| self.next(position, dir, part_1))
            .collect()
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

    #[test]
    fn example_part2() {
        let g = parse_input(EXAMPLE_INPUT);

        assert_eq!(part2(&g), 154);
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