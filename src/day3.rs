use crate::map::{Map, MapSegment, TobogganSlope};

#[aoc_generator(day3)]
pub fn input_generator(input: &str) -> Map {
    Map::from_segments(input
        .lines()
        .map(|line| MapSegment::from_line(line))
        .collect())
}

#[aoc(day3, part1)]
pub fn part1(map: &Map) -> u64 {
    map.count_trees_by_toboggan(TobogganSlope::Right3Down1)
}

#[aoc(day3, part2)]
pub fn part2(map: &Map) -> u64 {
    map.count_trees_by_toboggan(TobogganSlope::Right1Down1) *
    map.count_trees_by_toboggan(TobogganSlope::Right3Down1) *
    map.count_trees_by_toboggan(TobogganSlope::Right5Down1) *
    map.count_trees_by_toboggan(TobogganSlope::Right7Down1) *
    map.count_trees_by_toboggan(TobogganSlope::Right1Down2)
}