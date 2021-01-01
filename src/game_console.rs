use anyhow::{ Result, bail, anyhow };

#[derive(PartialOrd, PartialEq, Debug)]
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

#[derive(PartialOrd, PartialEq, Debug)]
pub struct Program {
    pub instr: Vec<Op>,
}

impl From<&str> for Program {
    fn from(input: &str) -> Self {
        Program {
        instr: input
            .lines()
            .map(|line| Op::from(line))
            .collect(),
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
}