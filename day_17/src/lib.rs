use std::{cmp::Reverse, collections::{BinaryHeap, HashSet}};

pub fn solve(input: String) {
    let map = parse_input(&input);

    println!("{}", part1(&map));
    println!("{}", part2(&map));
}

fn parse_input(input: &str) -> Vec<Vec<usize>> {
    input.lines().map(|l| l.chars().map(|c| (c as u8 - '0' as u8) as usize).collect()).collect()
}

fn part1(map: &Vec<Vec<usize>>) -> usize {
    dijkstra(map, 1, 3).expect("Path for part 1 doesn't exist")
}

fn part2(map: &Vec<Vec<usize>>) -> usize {
    dijkstra(map, 4, 10).expect("Path for part 2 doesn't exist")
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
enum Direction {
    UP,
    DOWN,
    LEFT,
    RIGHT
}

use Direction::*;

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
struct Node {
    x: usize,
    y: usize,
    moves_vertically: bool
}

impl Node {
    fn new(x: usize, y: usize, moves_vertically: bool) -> Self {
        Node { x, y, moves_vertically }
    }
}

fn dijkstra(map: &Vec<Vec<usize>>, min_straight: usize, max_straight: usize) -> Option<usize> {
    let w = map[0].len();
    let h = map.len();

    let mut pq = BinaryHeap::new();
    let mut visited = HashSet::<Node>::new();

    let starth = Node::new(0, 0, false);
    let startv = Node::new(0, 0, true);

    pq.push(Reverse((0, starth)));
    pq.push(Reverse((0, startv)));

    while !pq.is_empty() {
        let Reverse((distance, node)) = pq.pop().unwrap();

        if node.x == map[0].len() - 1 && node.y == map.len() - 1 {
            return Some(distance);
        }

        if visited.contains(&node) {
            continue;
        }

        visited.insert(node);

        for dir in [LEFT, RIGHT, UP, DOWN] {
            if node.moves_vertically {
                if dir == UP || dir == DOWN {
                    continue;
                }
            } else if dir == LEFT || dir == RIGHT {
                continue;
            }

            let mut delta = 0;
            for i in 1..max_straight + 1 {
                let x = match dir {
                    LEFT => node.x.checked_sub(i),
                    RIGHT => if node.x + i >= w { None } else { Some(node.x + i) },
                    _ => Some(node.x)
                };

                let y = match dir {
                    UP => node.y.checked_sub(i),
                    DOWN => if node.y + i >= h { None } else { Some(node.y + i) },
                    _ => Some(node.y)
                };

                if x.is_none() || y.is_none() {
                    break;
                }

                let x = x.unwrap();
                let y = y.unwrap();

                delta += map[y][x];

                if i >= min_straight {
                    pq.push(Reverse((distance + delta, Node::new(x, y, !node.moves_vertically))));
                }
            }
        }
    }

    None
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn example_part1() {
        let map = parse_input(EXAMPLE_INPUT);

        assert_eq!(part1(&map), 102);
    }

    #[test]
    fn example_part2() {
        let map = parse_input(EXAMPLE_INPUT);

        assert_eq!(part2(&map), 94);
    }

    const EXAMPLE_INPUT: &str = "2413432311323
3215453535623
3255245654254
3446585845452
4546657867536
1438598798454
4457876987766
3637877979653
4654967986887
4564679986453
1224686865563
2546548887735
4322674655533";
}