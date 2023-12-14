use std::collections::{HashSet, HashMap};

pub fn solve(input: String) {
    let map = parse_input(&input);

    println!("{}", part1(&map));
    println!("{}", part2(&map));
}

fn parse_input(input: &str) -> Vec<Vec<char>> {
    input.lines().map(|line| line.chars().collect()).collect()
}

fn part1(map: &Vec<Vec<char>>) -> usize {
    let mut map = map.clone();
    
    roll_north(&mut map);
    total_load(&map)
}

fn part2(map: &Vec<Vec<char>>) -> usize {
    let mut map = map.clone();
    let mut seen_states: HashMap<Vec<(usize, usize)>, usize> = HashMap::new();
    let mut i = 0;

    seen_states.insert(get_state(&map), 0);

    while i < 1000000000 - 1 {
        cycle(&mut map);
        let state = get_state(&map);

        if seen_states.contains_key(&state) {
            let skip = i - seen_states.get(&state).unwrap() + 1;
            while i + skip < 1000000000 {
                i += skip;
            }
            seen_states.clear();
        } else {
            i += 1;
            seen_states.insert(state, i);
        }
    }

    total_load(&map)
}

fn get_state(map: &Vec<Vec<char>>) -> Vec<(usize, usize)> {
    let mut v = map.iter().enumerate()
        .flat_map(|(y, row)| row.iter().enumerate()
            .filter(|(_, c)| c == &&'O')
            .map(move |(x, _)| (x, y))
        ).collect::<Vec<(usize, usize)>>();

    v.sort();

    v
}

fn cycle(map: &mut Vec<Vec<char>>) {
    roll_north(map);
    roll_west(map);
    roll_south(map);
    roll_east(map);
}

fn roll_north(map: &mut Vec<Vec<char>>) {
    roll_horizontal(map, false);
}

fn roll_south(map: &mut Vec<Vec<char>>) {
    roll_horizontal(map, true);
}

fn roll_west(map: &mut Vec<Vec<char>>) {
    roll_vertical(map, false);
}

fn roll_east(map: &mut Vec<Vec<char>>) {
    roll_vertical(map, true);
}

fn roll_horizontal(map: &mut Vec<Vec<char>>, reversed: bool) {
    let mut min_positions: Vec<isize> = (0..map[0].len()).map(|x| numbers_iterator(map.len(), reversed).find(|y| map[*y][x] != '#').unwrap() as isize).collect();
    let delta: isize = if reversed { -1 } else { 1 };

    for y in numbers_iterator(map.len(), reversed) {
        for x in 0..map[0].len() {
            match map[y][x] {
                'O' => {
                    map[y][x] = '.';
                    map[min_positions[x] as usize][x] = 'O';
                    min_positions[x] += delta;
                },
                '#' => {
                    min_positions[x] = y as isize + delta;
                },
                _ => {}
            };
        }
    }
}

fn roll_vertical(map: &mut Vec<Vec<char>>, reversed: bool) {
    let mut min_positions: Vec<isize> = (0..map.len()).map(|y| numbers_iterator(map[0].len(), reversed).find(|x| map[y][*x] != '#').unwrap() as isize).collect();
    let delta: isize = if reversed { -1 } else { 1 };

    for x in numbers_iterator(map[0].len(), reversed) {
        for y in 0..map.len() {
            match map[y][x] {
                'O' => {
                    map[y][x] = '.';
                    map[y][min_positions[y] as usize] = 'O';
                    min_positions[y] += delta;
                },
                '#' => {
                    min_positions[y] = x as isize + delta;
                },
                _ => {}
            };
        }
    }
}

fn numbers_iterator(max_y: usize, reversed: bool) -> Box<dyn Iterator<Item = usize>> {
    let iterator = 0..max_y;

    if reversed {
        return Box::new(iterator.rev());
    } else {
        return Box::new(iterator);
    }
}

fn total_load(map: &Vec<Vec<char>>) -> usize {
    let h = map.len();

    map.iter().enumerate().map(|(y, row)| row.iter().map(|c| if c == &'O' { h - y } else { 0 }).sum::<usize>()).sum()
}

#[cfg(test)]
mod tests {
    use crate::*;

    use std::iter::once;

    #[test]
    fn example_part1() {
        assert_eq!(part1(&parse_input(EXAMPLE_INPUT)), 136);
    }

    #[test]
    fn example_part2() {
        assert_eq!(part2(&parse_input(EXAMPLE_INPUT)), 64);
    }

    #[test]
    fn test_cycle() {
        let mut map = parse_input(EXAMPLE_INPUT);
        cycle(&mut map);
        assert_eq!(dumps(&map), ".....#....\n....#...O#\n...OO##...\n.OO#......\n.....OOO#.\n.O#...O#.#\n....O#....\n......OOOO\n#...O###..\n#..OO#....\n");
        cycle(&mut map);
        assert_eq!(dumps(&map), ".....#....\n....#...O#\n.....##...\n..O#......\n.....OOO#.\n.O#...O#.#\n....O#...O\n.......OOO\n#..OO###..\n#.OOO#...O\n");
        cycle(&mut map);
        assert_eq!(dumps(&map), ".....#....\n....#...O#\n.....##...\n..O#......\n.....OOO#.\n.O#...O#.#\n....O#...O\n.......OOO\n#...O###.O\n#.OOO#...O\n");
    }

    fn dumps(map: &Vec<Vec<char>>) -> String {
        map.iter().map(|row| row.iter().chain(once(&'\n')).collect::<String>()).collect()
    }

    const EXAMPLE_INPUT: &str = "O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#....";
}