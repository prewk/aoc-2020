use crate::game_console::{Program, OpStatus, generate_possible_uncorrupted};
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

#[aoc(day8, part2)]
pub fn part2(program: &Program) -> Result<i64> {
    let possible = generate_possible_uncorrupted(program);

    let mut finished: Option<i64> = None;

    for mut p in possible {
        loop {
            match p.exec() {
                OpStatus::Ok => {}
                OpStatus::InfiniteLoop => {
                    break;
                }
                OpStatus::OutOfBounds => {
                    finished = Some(p.acc);
                    break;
                }
            };
        }
    }

    finished.ok_or(anyhow!("No program finished correctly"))
}

