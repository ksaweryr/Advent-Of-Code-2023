use anyhow::Error;

pub fn solve(input: String) {
    let lines = parse_input(&input);

    println!("{}", part1(&lines));
    println!("{}", part2(&lines));
}

fn parse_input(input: &str) -> Vec<Vec<isize>> {
    input.lines()
        .map(parse_line)
        .collect::<Result<Vec<_>, _>>()
        .expect("Invalid input format")
}

fn parse_line(line: &str) -> Result<Vec<isize>, Error> {
    line.split(" ")
        .map(|x| x.parse().map_err(|e| Error::from(e)))
        .collect()
}

fn part1(lines: &Vec<Vec<isize>>) -> isize {
    lines.iter().map(next_value).sum()
}

fn part2(lines: &Vec<Vec<isize>>) -> isize {
    lines.iter().map(previous_value).sum()
}

fn next_value(row: &Vec<isize>) -> isize {
    if row.iter().all(|x| x == &0) {
        0
    } else {
        row.last().unwrap() + next_value(&differences(&row))
    }
}

fn previous_value(row: &Vec<isize>) -> isize {
    if row.iter().all(|x| x == &0) {
        0
    } else {
        row.first().unwrap() - previous_value(&differences(row))
    }
}

fn differences(row: &Vec<isize>) -> Vec<isize> {
    row.windows(2).map(|a| a[1] - a[0]).collect()
}


#[cfg(test)]
mod test {
    use crate::*;

    #[test]
    fn example_part1() {
        let lines = parse_input(EXAMPLE_INPUT);
        assert_eq!(part1(&lines), 114);
    }

    #[test]
    fn example_part2() {
        let lines = parse_input(EXAMPLE_INPUT);
        assert_eq!(part2(&lines), 2);
    }

    const EXAMPLE_INPUT: &str = "0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45";
}