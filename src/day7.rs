use std::collections::HashMap;
use crate::bags::{find_carrier_bag_count, input_str_to_hash_map};

#[aoc_generator(day7)]
pub fn input_generator(input: &str) -> HashMap<String, HashMap<String, u64>> {
    input_str_to_hash_map(input)
}

#[aoc(day7, part1)]
pub fn part(bags: &HashMap<String, HashMap<String, u64>>) -> usize {
    find_carrier_bag_count(bags, "shiny gold bag", 0).len()
}

