use crate::train_tickets::Notes;

#[aoc_generator(day16)]
pub fn input_generator(input: &str) -> Notes {
    Notes::from(input)
}

#[aoc(day16, part1)]
pub fn part1(notes: &Notes) -> usize {
    notes.get_invalid_sum()
}

