use crate::seating::Room;
use anyhow::Result;

#[aoc_generator(day11)]
pub fn input_generator(input: &str) -> Room {
    Room::from(input)
}

#[aoc(day11, part1)]
pub fn part1(room: &Room) -> Result<usize> {
    let mut prev = room.tick_part1()?;

    loop {
        let r = prev.tick_part1()?;

        if r.eq(&prev) {
            break;
        }

        prev = r;
    }

    Ok(prev.count_occupied())
}

#[aoc(day11, part2)]
pub fn part2(room: &Room) -> Result<usize> {
    let mut prev = room.tick_part2()?;

    loop {
        let r = prev.tick_part2()?;

        if r.eq(&prev) {
            break;
        }

        prev = r;
    }

    Ok(prev.count_occupied())
}
