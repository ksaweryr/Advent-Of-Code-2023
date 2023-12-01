pub fn solve(input: String) {
    let part1 = input
        .lines()
        .map(|line| line.chars().filter(char::is_ascii_digit).map(|c| c as u64 - '0' as u64).collect::<Vec<_>>())
        .map(|nums| nums[0] * 10 + nums.last().unwrap()).sum::<u64>();

    println!("{part1}");
}
