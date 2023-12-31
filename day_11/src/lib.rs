pub fn solve(input: String) {
    let galaxies1 = parse_input(&input, 2);
    let galaxies2 = parse_input(&input, 1000000);

    println!("{}", total_distances(&galaxies1));
    println!("{}", total_distances(&galaxies2));
}

fn parse_input(input: &str, expansion: isize) -> Vec<(isize, isize)> {
    let map: Vec<Vec<char>> = input.lines().map(|l| l.chars().collect()).collect();
    let mut galaxies: Vec<(isize, isize)> = map.iter().enumerate()
        .flat_map(|(y, r)| r.iter().enumerate().filter(|(_, c)| **c == '#').map(move |(x, _)| (x as isize, y as isize)))
        .collect();

    let expanded_rows: Vec<isize> = map.iter().enumerate()
        .filter(|(_, r)| r.iter().all(|c| *c == '.'))
        .map(|(y, _)| y as isize)
        .collect();

    let expanded_cols: Vec<isize> = transposed_iter(&map)
        .map(|mut r| r.all(|c| *c == '.'))
        .enumerate()
        .filter(|(_, b)| *b)
        .map(|(y, _)| y as isize)
        .collect();

    galaxies.iter_mut().for_each(|(x, y)| {
        let vdelta = expanded_rows.iter().filter(|row| *y > **row).count() as isize * (expansion - 1);
        let hdelta = expanded_cols.iter().filter(|col| *x > **col).count() as isize * (expansion - 1);
        *x += hdelta;
        *y += vdelta;
    });

    galaxies
}

fn total_distances(galaxies: &Vec<(isize, isize)>) -> isize {
    galaxies.iter().enumerate()
        .map(|(i, g1)| galaxies.iter().skip(i + 1).map(|g2| manhattan_distance(*g1, *g2)).sum::<isize>())
        .sum()
}

fn transposed_iter<'a, T>(v: &'a Vec<Vec<T>>) -> impl Iterator<Item = impl Iterator<Item = &T> + 'a> + 'a {
    let cols = v.len();
    let rows = v[0].len();

    (0..cols).map(move |y| (0..rows).map(move |x| &v[x][y]))
}

fn manhattan_distance((x1, y1): (isize, isize), (x2, y2): (isize, isize)) -> isize {
    (x1 - x2).abs() + (y1 - y2).abs()
}


#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn example_part1() {
        let galaxies = parse_input(EXAMPLE_INPUT, 2);
        assert_eq!(total_distances(&galaxies), 374);
    }

    const EXAMPLE_INPUT: &str = "...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....";
}