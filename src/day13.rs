use crate::bus::{Table};
use anyhow::Result;

#[aoc_generator(day13)]
pub fn input_generator(input: &str) -> (u64, Table) {
    let mut ts = 0;
    let mut table: Option<Table> = None;

    for (i, line) in input.lines().enumerate() {
        match i {
            0 => {
                ts = line.parse::<u64>().unwrap();
            },
            1 => {
                table = Some(Table::from(line));
            },
            _ => panic!("Too many lines"),
        }
    }

    (ts, table.unwrap())
}

#[aoc(day13, part1)]
pub fn part1((ts, t): &(u64, Table)) -> Result<u64> {
    let (id, delta) = t.find_earliest_bus(*ts)?;

    Ok(id * delta)
}
