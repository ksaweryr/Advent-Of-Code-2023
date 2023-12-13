pub fn solve(input: String) {
    let maps = parse_input(&input);

    println!("{}", get_solution(&maps, 1));
    println!("{}", get_solution(&maps, 2));
}

fn get_solution(maps: &Vec<Vec<Vec<char>>>, part: usize) -> usize {
    maps.iter().map(|m| value(m, part - 1)).sum()
}

fn parse_input(input: &str) -> Vec<Vec<Vec<char>>> {
    input.split("\n\n").map(|block| block.lines().map(|line| line.chars().collect()).collect()).collect()
}

fn value(map: &Vec<Vec<char>>, expected_distance: usize) -> usize {
    if let Some(col) = (1..map.len()).find(|col| is_reflection_line(&map, *col, expected_distance)) {
        100 * col
    } else {
        let transposed = transposed(&map);
        (1..transposed.len()).find(|row| is_reflection_line(&transposed, *row, expected_distance)).unwrap()
    }
}

fn is_reflection_line(map: &Vec<Vec<char>>, line: usize, expected_distance: usize) -> bool {
    map[0..line].iter().rev().zip(map[line..].iter()).map(|(r1, r2)| distance(r1, r2)).sum::<usize>() == expected_distance
}

fn transposed(map: &Vec<Vec<char>>) -> Vec<Vec<char>> {
    let w = map[0].len();
    let h = map.len();

    (0..w).map(|x| (0..h).map(move |y| map[y][x]).collect()).collect()
}

fn distance(lhs: &Vec<char>, rhs: &Vec<char>) -> usize {
    lhs.iter().zip(rhs).filter(|(a, b)| a != b).count()
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn test_is_reflection_line() {
        let map = &parse_input(EXAMPLE_INPUT)[1];
        assert!(is_reflection_line(map, 4, 0));
    }

    #[test]
    fn test_value_vertical() {
        let map = &parse_input(EXAMPLE_INPUT)[0];
        assert_eq!(value(map, 0), 5);
    }

    #[test]
    fn test_value_horizontal() {
        let map = &parse_input(EXAMPLE_INPUT)[1];
        assert_eq!(value(map, 0), 400);
    }

    #[test]
    fn example_part1() {
        let maps = parse_input(EXAMPLE_INPUT);
        assert_eq!(get_solution(&maps, 1), 405);
    }

    #[test]
    fn example_part2() {
        let maps = parse_input(EXAMPLE_INPUT);
        assert_eq!(get_solution(&maps, 2), 400);
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