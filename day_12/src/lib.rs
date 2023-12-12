#![feature(iter_intersperse)]
#![feature(iter_repeat_n)]

use std::iter::repeat_n;
use std::str::FromStr;

use anyhow::Error;

pub fn solve(input: String) {
    let rows = parse_input(&input);

    println!("{}", part1(&rows));
    println!("{}", part2(&rows));
}

fn parse_input(input: &str) -> Vec<Row> {
    input.lines().map(|l| l.parse()).collect::<Result<_, _>>().expect("Invalid input format")
}

fn part1(rows: &Vec<Row>) -> usize {
    // rows.iter().map(possible_arrangements).sum()
    calculate_answer(rows)
}

fn part2(rows: &Vec<Row>) -> usize {
    calculate_answer(&rows.iter().map(Row::unfolded).collect())
}

fn calculate_answer(rows: &Vec<Row>) -> usize {
    rows.iter().map(|row| possible_arrangements_dp(row.damaged_record.as_ref(), &row.counts, &mut vec![vec![None; row.damaged_record.len() + 1]; row.counts.len() + 1])).sum()
}

fn possible_arrangements_dp(record: &str, counts: &[usize], dp: &mut Vec<Vec<Option<usize>>>) -> usize {
    if counts.len() == 0 {
        return if record.chars().any(|x| x == '#') {
            0
        } else {
            1
        };
    }

    if let None = dp[counts.len()][record.len()] {
        let n = counts[0];
        let mut result = 0;

        for i in 0..record.len() - (counts.len() - 1 + &counts[1..].iter().sum()) - n + 1 {
            if record.chars().nth(i + n) != Some('#') && record[i..i+n].chars().all(|x| x != '.') {
                result += possible_arrangements_dp(&record[(i+n+1).min(record.len())..], &counts[1..], dp);
            }

            if record.chars().nth(i).unwrap() == '#' {
                break;
            }
        }

        dp[counts.len()][record.len()] = Some(result);
    }

    dp[counts.len()][record.len()].unwrap()
}

#[allow(dead_code)]
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

#[derive(Debug)]
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

impl Row {
    fn unfolded(&self) -> Self {
        let damaged_record = repeat_n(self.damaged_record.clone(), 5).intersperse("?".to_owned()).collect();
        let counts = self.counts.iter().map(Clone::clone).cycle().take(self.counts.len() * 5).collect::<Vec<usize>>();

        Self { damaged_record, counts }
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
    fn test_possible_arrangements_dp() {
        assert_eq!(possible_arrangements_dp("???#.?.#??.?##?????#", &vec![2,3,2,3], &mut vec![vec![None; 21]; 5]), 1);
    }

    #[test]
    fn example_part1() {
        let rows = parse_input(EXAMPLE_INPUT);
        assert_eq!(part1(&rows), 21);
    }

    #[test]
    fn example_part2() {
        let rows = parse_input(EXAMPLE_INPUT);
        assert_eq!(part2(&rows), 525152);
    }

    const EXAMPLE_INPUT: &str = "???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1
";
}