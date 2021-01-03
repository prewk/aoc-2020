use anyhow::{Result, bail};
use crate::xmas::{test_number, find_weakness};

#[aoc_generator(day9)]
pub fn input_generator(input: &str) -> Vec<u64> {
    input
        .lines()
        .map(|l| l.parse::<u64>().unwrap())
        .collect()
}

#[aoc(day9, part1)]
pub fn part1(all: &[u64]) -> Result<u64> {
    for n in 25..all.len() {
        if !test_number(all, n)? {
            return Ok(all[n])
        }
    }

    bail!("No number found")
}

#[aoc(day9, part2)]
pub fn part2(all: &[u64]) -> Result<u64> {
    return find_weakness(all, part1(all).unwrap());
}