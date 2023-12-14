pub fn solve(input: String) {
    let map = parse_input(&input);

    println!("{}", part1(&map));
}

fn parse_input(input: &str) -> Vec<Vec<char>> {
    input.lines().map(|line| line.chars().collect()).collect()
}

fn part1(map: &Vec<Vec<char>>) -> usize {
    let mut map = map.clone();
    
    roll_north(&mut map);
    score(&map)
}

fn roll_north(map: &mut Vec<Vec<char>>) {
    for y in 0..map.len() {
        for x in 0..map[0].len() {
            if map[y][x] == 'O' {
                map[y][x] = '.';
                let mut p = y;

                while p > 0 && map[p - 1][x] == '.' {
                    p -= 1;
                }

                map[p][x] = 'O';
            }
        }
    }
}

fn score(map: &Vec<Vec<char>>) -> usize {
    let h = map.len();

    map.iter().enumerate().map(|(y, row)| row.iter().map(|c| if c == &'O' { h - y } else { 0 }).sum::<usize>()).sum()
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn example_part1() {
        assert_eq!(part1(&parse_input(EXAMPLE_INPUT)), 136);
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