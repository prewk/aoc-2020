use crate::password::PasswordEntry;

#[aoc_generator(day2)]
pub fn input_generator(input: &str) -> Vec<PasswordEntry> {
    input
        .lines()
        .map(|line| PasswordEntry::from_line(line))
        .collect()
}

#[aoc(day2, part1)]
pub fn part1(passwords: &[PasswordEntry]) -> u64 {
    passwords
        .iter()
        .filter(|password| password.sled_rental_validate())
        .count() as u64
}

#[aoc(day2, part2)]
pub fn part2(passwords: &[PasswordEntry]) -> u64 {
    passwords
        .iter()
        .filter(|password| password.toboggan_validate())
        .count() as u64
}