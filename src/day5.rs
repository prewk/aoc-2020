use crate::boarding::Seat;
use std::collections::HashSet;
use anyhow::{Result, bail};

#[aoc_generator(day5)]
pub fn input_generator(input: &str) -> Vec<Seat> {
    input
        .lines()
        .map(|line| Seat::from(line))
        .collect()
}

#[aoc(day5, part1)]
pub fn part1(seats: &[Seat]) -> u64 {
    seats
        .iter()
        .map(|s| s.get_seat_id())
        .max()
        .unwrap()
}

#[aoc(day5, part2)]
pub fn part2(seats: &[Seat]) -> Result<u64> {
    let mut ids = HashSet::new();

    for s in seats {
        ids.insert(s.get_seat_id());
    }

    for id in &ids {
        let behind = id - 1;
        let before = id + 1;

        if !ids.contains(&behind) && ids.contains(&(behind - 1)) {
            return Ok(behind);
        }
        if !ids.contains(&before) && ids.contains(&(before + 1)) {
            return Ok(before);
        }
    }

    bail!("Oh noes");
}