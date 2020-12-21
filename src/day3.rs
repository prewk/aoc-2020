use crate::map::{Map, MapSegment, TobogganDir, TobogganTraveler};

#[aoc_generator(day3)]
pub fn input_generator(input: &str) -> Map {
    Map::from_segments(input
        .lines()
        .map(|line| MapSegment::from_line(line))
        .collect())
}

#[aoc(day3, part1)]
pub fn part1(map: &Map) -> u64 {
    map.count_trees_by_traveler(TobogganTraveler::new(vec![
        TobogganDir::Right,
        TobogganDir::Right,
        TobogganDir::Right,
        TobogganDir::Down,
    ]))
}

#[aoc(day3, part2)]
pub fn part2(map: &Map) -> u64 {
    map.count_trees_by_traveler(TobogganTraveler::new(vec![
        TobogganDir::Right,
        TobogganDir::Down,
    ])) *
    map.count_trees_by_traveler(TobogganTraveler::new(vec![
        TobogganDir::Right,
        TobogganDir::Right,
        TobogganDir::Right,
        TobogganDir::Down,
    ])) *
    map.count_trees_by_traveler(TobogganTraveler::new(vec![
        TobogganDir::Right,
        TobogganDir::Right,
        TobogganDir::Right,
        TobogganDir::Right,
        TobogganDir::Right,
        TobogganDir::Down,
    ])) *
    map.count_trees_by_traveler(TobogganTraveler::new(vec![
        TobogganDir::Right,
        TobogganDir::Right,
        TobogganDir::Right,
        TobogganDir::Right,
        TobogganDir::Right,
        TobogganDir::Right,
        TobogganDir::Right,
        TobogganDir::Down,
    ])) *
    map.count_trees_by_traveler(TobogganTraveler::new(vec![
        TobogganDir::Right,
        TobogganDir::Down,
        TobogganDir::Down,
    ]))
}