use crate::ship::{Instruction, Boat1, Boat2};

#[aoc_generator(day12)]
pub fn input_generator(input: &str) -> Vec<Instruction> {
    input
        .lines()
        .map(|line| Instruction::from(line))
        .collect()
}

#[aoc(day12, part1)]
pub fn part1(instructions: &[Instruction]) -> u64 {
    let mut boat = Boat1::new();

    for instr in instructions {
        boat.tick(instr);
    }

    boat.get_manhattan_dist()
}

#[aoc(day12, part2)]
pub fn part2(instructions: &[Instruction]) -> u64 {
    let mut boat = Boat2::new();

    for instr in instructions {
        boat.tick(instr);
    }

    boat.get_manhattan_dist()
}
