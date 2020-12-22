use crate::passport::Passport;

#[aoc_generator(day4)]
pub fn input_generator(input: &str) -> Vec<Passport> {
    input
        .split(&"\n\n")
        .map(|chunk| Passport::from_line(&chunk.replace(&"\n", &" ")))
        .collect()
}

#[aoc(day4, part1)]
pub fn part1(passports: &[Passport]) -> u64 {
    passports
        .iter()
        .fold(0, |acc, p| {
            match p.validate() {
                true => acc + 1,
                false => acc,
            }
        })
}