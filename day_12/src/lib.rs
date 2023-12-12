use std::str::FromStr;

use anyhow::Error;

pub fn solve(input: String) {
    let rows = parse_input(&input);

    println!("{}", part1(&rows));
}

fn parse_input(input: &str) -> Vec<Row> {
    input.lines().map(|l| l.parse()).collect::<Result<_, _>>().expect("Invalid input format")
}

fn part1(rows: &Vec<Row>) -> usize {
    rows.iter().map(possible_arrangements).sum()
}

fn possible_arrangements(row: &Row) -> usize {
    let question_marks = row.damaged_record.chars().filter(|c| c == &'?').count() as u32;
    (0..(2usize).pow(question_marks)).map(|i| (0..question_marks).fold(row.damaged_record.clone(), |record, shift| {
        let char = if ((i >> shift) & 1) == 1 { "#" } else { "." };
        record.replacen('?', char, 1)
    }))
    .filter(|r| is_correct(&r, &row.counts))
    .count()
}

fn is_correct(record: &str, numbers: &Vec<usize>) -> bool {
    record.split('.').map(str::len).filter(|x| x != &0).eq(numbers.iter().map(|x| *x))
}

struct Row {
    damaged_record: String,
    counts: Vec<usize>
}

impl FromStr for Row {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (damaged_record, numbers) = s.split_once(' ').ok_or(Error::msg("Invalid row format"))?;
        let damaged_record = damaged_record.to_owned();
        let counts = numbers.split(',').map(|x| x.parse::<usize>().map_err(Error::new)).collect::<Result<Vec<usize>, _>>()?;

        Ok(Row { damaged_record, counts })
    }
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn test_is_correct() {
        assert!(is_correct("#.#.###", &vec![1,1,3]));
    }

    #[test]
    fn test_possible_arrangements() {
        assert_eq!(possible_arrangements(&Row { damaged_record: "?###????????".to_owned(), counts: vec![3,2,1] }), 10);
    }

    #[test]
    fn example_part1() {
        let rows = parse_input(EXAMPLE_INPUT);
        assert_eq!(part1(&rows), 21);
    }

    const EXAMPLE_INPUT: &str = "???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1
";
}