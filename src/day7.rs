use std::collections::HashMap;
use crate::bags::{find_carrier_bags, input_str_to_hash_map, find_content_count};

#[aoc_generator(day7)]
pub fn input_generator(input: &str) -> HashMap<String, HashMap<String, u64>> {
    input_str_to_hash_map(input)
}

#[aoc(day7, part1)]
pub fn part1(bags: &HashMap<String, HashMap<String, u64>>) -> usize {
    find_carrier_bags(bags, "shiny gold bag").len()
}

#[aoc(day7, part2)]
pub fn part2(bags: &HashMap<String, HashMap<String, u64>>) -> u64 {
    find_content_count(bags, "shiny gold bag", 0)
}
