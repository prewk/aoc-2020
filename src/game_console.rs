use anyhow::{ Result, bail, anyhow };
use std::collections::HashSet;

#[derive(PartialOrd, PartialEq, Debug, Copy, Clone)]
pub enum Op {
    Acc(i64),
    Jmp(i64),
    Nop,
}

fn parse_arg(line: &str) -> Result<i64> {
   let mult = match line.chars().nth(4) {
       Some('-') => -1,
       Some('+') => 1,
       None => bail!("Missing sign"),
       _ => bail!("Invalid sign!"),
   };

    let i = match line.get(5..) {
        Some(v) => v.parse::<i64>().map_err(|_| anyhow!("Unparsable integer: {}", v)),
        None => Err(anyhow!("Missing integer")),
    }?;

    Ok(mult * i)
}

impl From<&str> for Op {
    fn from(line: &str) -> Self {
        match line.get(0..3) {
            Some("acc") => Op::Acc(parse_arg(line).unwrap()),
            Some("jmp") => Op::Jmp(parse_arg(line).unwrap()),
            Some("nop") => Op::Nop,
            _ => { panic!("Invalid Op: {}", line); },
        }
    }
}

#[derive(PartialEq, Debug)]
pub struct Program {
    pub ptr: usize,
    pub acc: i64,
    pub instr: Vec<Op>,
    pub dirty: HashSet<usize>,
}

impl Clone for Program {
    fn clone(&self) -> Self {
        Program {
            ptr: self.ptr,
            acc: self.acc,
            instr: self.instr.clone(),
            dirty: self.dirty.clone(),
        }
    }

    fn clone_from(&mut self, source: &Self) {
        unimplemented!()
    }
}

impl From<&str> for Program {
    fn from(input: &str) -> Self {
        Program {
            ptr: 0,
            acc: 0,
            instr: input
                .lines()
                .map(|line| Op::from(line))
                .collect(),
            dirty: HashSet::new(),
        }
    }
}

pub enum OpStatus {
    Ok,
    InfiniteLoop,
    OutOfBounds,
}

impl Program {
    pub fn exec(&mut self) -> OpStatus {
        match self.dirty.contains(&self.ptr) {
            true => OpStatus::InfiniteLoop,
            false => match self.instr.get(self.ptr) {
                None => OpStatus::OutOfBounds,
                Some(Op::Nop) => {
                    self.dirty.insert(self.ptr);
                    self.ptr += 1;
                    OpStatus::Ok
                },
                Some(Op::Acc(val)) => {
                    self.dirty.insert(self.ptr);
                    self.ptr += 1;
                    self.acc += val;

                    OpStatus::Ok
                },
                Some(Op::Jmp(val)) => {
                    self.dirty.insert(self.ptr);
                    self.ptr += *val as usize;

                    OpStatus::Ok
                },
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parser() {
        let input = "nop +0\n\
                           acc +1\n\
                           jmp +4\n\
                           acc +3\n\
                           jmp -3\n\
                           acc -99\n\
                           acc +1\n\
                           jmp -4\n\
                           acc +6";

        let p1 = Program::from(input);

        assert_eq!(p1.instr, vec![
            Op::Nop,
            Op::Acc(1),
            Op::Jmp(4),
            Op::Acc(3),
            Op::Jmp(-3),
            Op::Acc(-99),
            Op::Acc(1),
            Op::Jmp(-4),
            Op::Acc(6),
        ]);
    }

    fn test_exec() {
        let input = "nop +0\n\
                           acc +1\n\
                           jmp +4\n\
                           acc +3\n\
                           jmp -3\n\
                           acc -99\n\
                           acc +1\n\
                           jmp -4\n\
                           acc +6";

        let mut p1 = Program::from(input);

        let mut hung_acc = 0;

        loop {
            match p1.exec() {
                OpStatus::Ok => {}
                OpStatus::InfiniteLoop => {
                    hung_acc = p1.acc;
                }
                OpStatus::OutOfBounds => {
                    assert!(false);
                }
            };
        }

        assert_eq!(hung_acc, 5);
    }
}