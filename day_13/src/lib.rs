pub fn solve(input: String) {
    let maps = parse_input(&input);

    println!("{}", part1(&maps));
}

fn part1(maps: &Vec<Vec<Vec<char>>>) -> usize {
    maps.iter().map(value).sum()
}

fn parse_input(input: &str) -> Vec<Vec<Vec<char>>> {
    input.split("\n\n").map(|block| block.lines().map(|line| line.chars().collect()).collect()).collect()
}

fn value(map: &Vec<Vec<char>>) -> usize {
    if let Some(col) = (1..map.len()).find(|col| is_reflection_line(&map, *col)) {
        100 * col
    } else {
        let transposed = transposed(&map);
        (1..transposed.len()).find(|row| is_reflection_line(&transposed, *row)).unwrap()
    }
}

fn is_reflection_line(map: &Vec<Vec<char>>, line: usize) -> bool {
    map[0..line].iter().rev().zip(map[line..].iter()).all(|(r1, r2)| r1 == r2)
}

fn transposed(map: &Vec<Vec<char>>) -> Vec<Vec<char>> {
    let w = map[0].len();
    let h = map.len();

    (0..w).map(|x| (0..h).map(move |y| map[y][x]).collect()).collect()
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn test_is_reflection_line() {
        let map = &parse_input(EXAMPLE_INPUT)[1];
        assert!(is_reflection_line(map, 4));
    }

    #[test]
    fn test_value_vertical() {
        let map = &parse_input(EXAMPLE_INPUT)[0];
        assert_eq!(value(map), 5);
    }

    #[test]
    fn test_value_horizontal() {
        let map = &parse_input(EXAMPLE_INPUT)[1];
        assert_eq!(value(map), 400);
    }

    const EXAMPLE_INPUT: &str = "#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.

#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#";
}