use std::collections::{VecDeque, HashSet};

pub fn solve(input: String) {
    let (map, starting_pos) = parse_input(&input);

    println!("{}", part1(&map, starting_pos, 64));
}

fn part1(map: &Vec<Vec<char>>, starting_pos: (usize, usize), max_distance: usize) -> usize {
    let mut q = VecDeque::<(usize, (usize, usize))>::new();
    let mut visited = HashSet::<(usize, usize)>::new();
    let mut cnt = 0;

    q.push_back((0, starting_pos));
    visited.insert(starting_pos);

    while !q.is_empty() {
        let (distance, (x, y)) = q.pop_front().unwrap();

        if distance % 2 == 0 {
            cnt += 1;
        }

        if distance == max_distance {
            continue;
        }

        if x > 0 && !visited.contains(&(x - 1, y)) && map[y][x - 1] != '#' {
            q.push_back((distance + 1, (x - 1, y)));
            visited.insert((x - 1, y));
        }

        if x < map[0].len() - 1 && !visited.contains(&(x + 1, y)) && map[y][x + 1] != '#' {
            q.push_back((distance + 1, (x + 1, y)));
            visited.insert((x + 1, y));
        }

        if y > 0 && !visited.contains(&(x, y - 1)) && map[y - 1][x] != '#' {
            q.push_back((distance + 1, (x, y - 1)));
            visited.insert((x, y - 1));
        }

        if y < map.len() && !visited.contains(&(x, y + 1)) && map[y + 1][x] != '#' {
            q.push_back((distance + 1, (x, y + 1)));
            visited.insert((x, y + 1));
        }
    }

    cnt
}

fn parse_input(input: &str) -> (Vec<Vec<char>>, (usize, usize)) {
    let map = input.lines().map(|l| l.chars().collect::<Vec<_>>()).collect::<Vec<_>>();
    let starting_pos = map.iter()
        .enumerate()
        .find_map(|(y, row)| row.iter().position(|c| c == &'S').map(|x| (x, y)))
        .expect("No starting position");

    (map, starting_pos)
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn example_part1() {
        let (map, starting_pos) = parse_input(EXAMPLE_INPUT);

        assert_eq!(part1(&map, starting_pos, 6), 16);
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