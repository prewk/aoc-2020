use crate::expense_report::*;

#[aoc_generator(day1)]
pub fn input_generator(input: &str) -> Vec<u64> {
    input
        .lines()
        .map(|line| line.parse::<u64>())
        .filter_map(Result::ok)
        .collect()
}

#[aoc(day1, part1)]
pub fn part1(entries: &[u64]) -> u64 {
    find_2020_2_sum_and_mul(entries).expect("Not found!")
}

#[aoc(day1, part2)]
pub fn part2(entries: &[u64]) -> u64 {
    find_2020_3_sum_and_mul(entries).expect("Not found!")
}