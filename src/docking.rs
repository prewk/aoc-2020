use regex::Regex;
use std::collections::{HashSet, HashMap};

#[derive(Debug, Copy, Clone)]
pub enum MaskBit {
    PassThrough,
    One,
    Zero,
}

#[derive(Debug)]
pub struct Mask {
    bits: Vec<MaskBit>,
}

impl From<&str> for Mask {
    fn from(mask: &str) -> Self {
        Mask {
            bits: mask
                .chars()
                .map(|c| match c {
                    'X' => MaskBit::PassThrough,
                    '1' => MaskBit::One,
                    '0' => MaskBit::Zero,
                    _ => panic!("Invalid mask bit: {}", c),
                })
                .collect()
        }
    }
}

impl Clone for Mask {
    fn clone(&self) -> Self {
        Mask {
            bits: self.bits.clone()
        }
    }

    fn clone_from(&mut self, source: &Self) {
        unimplemented!()
    }
}

impl Mask {
    pub fn merge(&self, value: u64) -> u64 {
        let bin: String = format!("{:b}", value).chars().rev().into_iter().collect();
        let mut rev_out = vec![];

        for (i, bit) in self.bits.iter().rev().enumerate() {
            rev_out.push(match bit {
                MaskBit::PassThrough => match i < bin.len() {
                    true => bin.chars().nth(i).unwrap(),
                    false => '0',
                },
                MaskBit::One => '1',
                MaskBit::Zero => '0',
            });
        }

        let out: String = rev_out.into_iter().rev().collect();

        isize::from_str_radix(&out, 2).unwrap() as u64
    }
}

pub struct System {
    mem_map: HashMap<usize, u64>,
    bitmask: Mask,
}

impl Clone for System {
    fn clone(&self) -> Self {
        System {
            mem_map: self.mem_map.clone(),
            bitmask: self.bitmask.clone(),
        }
    }

    fn clone_from(&mut self, source: &Self) {
        unimplemented!()
    }
}

impl System {
    pub fn new() -> System {
        System {
            mem_map: HashMap::new(),
            bitmask: Mask::from("XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X"),
        }
    }

    pub fn set_mask(&mut self, mask: Mask) {
        self.bitmask = mask;
    }

    pub fn set(&mut self, address: usize, value: u64) {
        self.mem_map.insert(address, self.bitmask.merge(value));
    }

    pub fn sum(&self) -> u64 {
        self.mem_map
            .iter()
            .fold(0, |acc, (_, &value)| acc + value)
    }
}

#[derive(Debug)]
pub enum Instruction {
    SetMask(Mask),
    SetMem(usize, u64),
}

impl From<&str> for Instruction {
    fn from(input: &str) -> Self {
        let mut split = input.split(" = ").into_iter();
        let part1 = split.next().unwrap();
        let part2 = split.next().unwrap();
        let re = Regex::new("^mem\\[(\\d+)] = (\\d+)$").unwrap();

        match part1 {
            "mask" => Instruction::SetMask(Mask::from(part2)),
            _ => {
                let captures = re.captures(input).unwrap();

                let address = captures.get(1).unwrap().as_str().parse::<usize>().unwrap();
                let value = captures.get(2).unwrap().as_str().parse::<u64>().unwrap();

                Instruction::SetMem(address, value)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_merge_mask() {
        assert_eq!(Mask::from("XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X").merge(11), 73);
        assert_eq!(Mask::from("XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X").merge(101), 101);
        assert_eq!(Mask::from("XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X").merge(0), 64);
    }

    #[test]
    fn test_system() {
        let mut system = System::new();

        for instr in vec![Instruction::from("mask = XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X"),
                          Instruction::from("mem[8] = 11"),
                          Instruction::from("mem[7] = 101"),
                          Instruction::from("mem[8] = 0"),
        ] {
            match instr {
                Instruction::SetMask(mask) => {
                    system.set_mask(mask);
                }
                Instruction::SetMem(address, value) => {
                    system.set(address, value);
                }
            }
        }

        assert_eq!(system.sum(), 165);
    }
}