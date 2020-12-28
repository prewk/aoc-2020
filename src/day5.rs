use crate::boarding::Seat;

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
