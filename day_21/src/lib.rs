use std::collections::{VecDeque, HashSet};

use matrix::Matrix;
use vector::Vector;

use crate::fraction::Fraction;

mod fraction;
mod matrix;
mod vector;

pub fn solve(input: String) {
    let (map, starting_pos) = parse_input(&input);
    
    println!("{}", part1(&map, starting_pos));
    println!("{}", part2(&map, starting_pos));
}

fn parse_input(input: &str) -> (Vec<Vec<char>>, (isize, isize)) {
    let map = input.lines().map(|l| l.chars().collect::<Vec<_>>()).collect::<Vec<_>>();
    let starting_pos = map.iter()
        .enumerate()
        .find_map(|(y, row)| row.iter().position(|c| c == &'S').map(|x| (x as isize, y as isize)))
        .expect("No starting position");

    (map, starting_pos)
}

fn part1(map: &Vec<Vec<char>>, starting_pos: (isize, isize)) -> usize {
    bfs(map, starting_pos, 64)
}

fn part2(map: &Vec<Vec<char>>, starting_pos: (isize, isize)) -> usize {
    let dim = map.len();
    let steps = 26501365;
    let x = steps % dim;

    let x0 = x;
    let x1 = x + dim;
    let x2 = x + 2 * dim;

    let y0 = (bfs(map, starting_pos, x0) as i128).into();
    let y1 = (bfs(map, starting_pos, x1) as i128).into();
    let y2 = (bfs(map, starting_pos, x2) as i128).into();

    let x0 = (x0 as i128).into();
    let x1 = (x1 as i128).into();
    let x2 = (x2 as i128).into();

    let m = Matrix([
        [x0 * x0, x0, Fraction::new(1, 1)],
        [x1 * x1, x1, Fraction::new(1, 1)],
        [x2 * x2, x2, Fraction::new(1, 1)]
    ]);
    let ys = Vector([y0, y1, y2]);

    let [a, b, c] = (m.inv().unwrap() * ys).0;

    let steps: Fraction = (steps as i128).into();
    let result: Fraction = a * steps * steps + b * steps + c;
    assert_eq!(result.q, 1);

    result.p as usize
}

fn bfs(map: &Vec<Vec<char>>, starting_pos: (isize, isize), max_distance: usize) -> usize {
    let mut q = VecDeque::<(usize, (isize, isize))>::new();
    let mut visited = HashSet::<(isize, isize)>::new();
    let mut cnt = 0;
    let dim = map.len() as isize;
    let md = |x: isize| x.rem_euclid(dim) as usize;

    q.push_back((0, starting_pos));
    visited.insert(starting_pos);

    while !q.is_empty() {
        let (distance, (x, y)) = q.pop_front().unwrap();

        if distance % 2 == max_distance % 2 {
            cnt += 1;
        }

        if distance == max_distance {
            continue;
        }

        if !visited.contains(&(x - 1, y)) && map[md(y)][md(x - 1)] != '#' {
            q.push_back((distance + 1, (x - 1, y)));
            visited.insert((x - 1, y));
        }

        if !visited.contains(&(x + 1, y)) && map[md(y)][md(x + 1)] != '#' {
            q.push_back((distance + 1, (x + 1, y)));
            visited.insert((x + 1, y));
        }

        if !visited.contains(&(x, y - 1)) && map[md(y - 1)][md(x)] != '#' {
            q.push_back((distance + 1, (x, y - 1)));
            visited.insert((x, y - 1));
        }

        if !visited.contains(&(x, y + 1)) && map[md(y + 1)][md(x)] != '#' {
            q.push_back((distance + 1, (x, y + 1)));
            visited.insert((x, y + 1));
        }
    }

    cnt
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn example_part1() {
        let (map, starting_pos) = parse_input(EXAMPLE_INPUT);

        assert_eq!(bfs(&map, starting_pos, 6), 16);
    }

    const EXAMPLE_INPUT: &str = "...........
.....###.#.
.###.##..#.
..#.#...#..
....#.#....
.##..S####.
.##..#...#.
.......##..
.##.#.####.
.##..##.##.
...........";
}