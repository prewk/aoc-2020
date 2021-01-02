use crate::game_console::{Program, OpStatus};
use anyhow::{Result, anyhow};

#[aoc_generator(day8)]
pub fn input_generator(input: &str) -> Program {
    Program::from(input)
}

#[aoc(day8, part1)]
pub fn part1(program: &Program) -> Result<i64> {
    let mut p = program.clone();

    loop {
        match p.exec() {
            OpStatus::Ok => {}
            OpStatus::InfiniteLoop => {
                return Ok(p.acc);
            }
            OpStatus::OutOfBounds => {
                return Err(anyhow!("Out of bounds"));
            }
        };
    }
}