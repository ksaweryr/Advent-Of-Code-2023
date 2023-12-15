pub fn solve(input: String) {
    let instructions = parse_input(&input);

    println!("{}", part1(&instructions));
}

fn parse_input(input: &str) -> Vec<&str> {
    input.trim().split(",").collect()
}

fn part1(instructions: &Vec<&str>) -> usize {
    instructions.iter().map(|s| hash(*s)).sum()
}

fn hash(s: &str) -> usize {
    s.chars().map(|c| c as u8 as usize).fold(0, |acc, c| 17 * (acc + c) % 256)
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn test_hash() {
        assert_eq!(EXAMPLE_INPUT.split(",").map(hash).collect::<Vec<_>>(), vec![30, 253, 97, 47, 14, 180, 9, 197, 48, 214, 231]);
    }

    #[test]
    fn example_part1() {
        let instructions = parse_input(EXAMPLE_INPUT);
        assert_eq!(part1(&instructions), 1320);
    }

    const EXAMPLE_INPUT: &str = "rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7";
}