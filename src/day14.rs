use crate::docking::{System, Instruction};

#[aoc_generator(day14)]
pub fn input_generator(input: &str) -> Vec<Instruction> {
    input
        .lines()
        .map(|line| Instruction::from(line))
        .collect()
}

#[aoc(day14, part1)]
pub fn part1(instructions: &[Instruction]) -> u64 {
    let mut sys = System::new();

    for instr in instructions {
        match instr {
            Instruction::SetMask(mask) => {
                sys.set_mask(mask.clone());
            }
            Instruction::SetMem(address, value) => {
                sys.set(*address, *value);
            }
        }
    }

    sys.sum()
}
