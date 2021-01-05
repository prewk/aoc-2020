use crate::jolts::find_jolt_differences;
use anyhow::Result;

#[aoc_generator(day10)]
pub fn input_generator(input: &str) -> Vec<u64> {
    let mut sorted: Vec<u64> = input.lines().map(|l| l.parse::<u64>().unwrap()).collect();

    sorted.sort();
    sorted.insert(0, 0);
    sorted.push(*sorted.iter().max().unwrap() + 3);

    sorted
}

#[aoc(day10, part1)]
pub fn part1(adapters: &[u64]) -> u64 {
    let map = find_jolt_differences(&adapters.iter().map(|i| *i).collect()).unwrap();

    *map.get(&1).unwrap() * *map.get(&3).unwrap()
}
