use crate::seating::Room;
use anyhow::Result;

#[aoc_generator(day11)]
pub fn input_generator(input: &str) -> Room {
    Room::from(input)
}

#[aoc(day11, part1)]
pub fn part1(room: &Room) -> Result<usize> {
    let mut prev = room.tick()?;

    loop {
        let r = prev.tick()?;

        if r.eq(&prev) {
            break;
        }

        prev = r;
    }

    Ok(prev.count_occupied())
}
