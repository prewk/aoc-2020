use regex::Regex;
use std::collections::{HashMap};

#[derive(Debug, Copy, Clone, PartialOrd, PartialEq)]
pub enum MaskBit {
    X,
    One,
    Zero,
}

#[derive(Debug, PartialOrd, PartialEq)]
pub struct Mask {
    bits: Vec<MaskBit>,
}

impl From<&str> for Mask {
    fn from(mask: &str) -> Self {
        Mask {
            bits: mask
                .chars()
                .map(|c| match c {
                    'X' => MaskBit::X,
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

    fn clone_from(&mut self, _: &Self) {
        unimplemented!()
    }
}

impl Mask {
    pub fn apply(&self, value: u64) -> u64 {
        let bin: String = format!("{:b}", value).chars().rev().into_iter().collect();
        let mut rev_out = vec![];

        for (i, bit) in self.bits.iter().rev().enumerate() {
            rev_out.push(match bit {
                MaskBit::X => match i < bin.len() {
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

    pub fn translate_to_v2(&self) -> Vec<Mask> {
        let x_cnt = self.bits.iter().fold(0, |acc, bit| if let MaskBit::X = bit {
            acc + 1
        } else {
            acc
        });
        let base: i32 = 2;
        let pow = base.pow(x_cnt);
        let mut bitmasks = vec![];
        for value in 0..pow {
            let mut bin = format!("{:b}", value);
            let add_leading = x_cnt - bin.len() as u32;
            for _ in 0..add_leading {
                bin = format!("0{}", bin);
            }

            let mut bin_digits = bin.chars();
            let bitmask: Vec<char> = self.bits.iter().map(|bit| match bit {
                MaskBit::X => match bin_digits.next().unwrap() {
                    '1' => '1',
                    '0' => '0',
                    _ => panic!("Invalid binary")
                },
                MaskBit::One => '1',
                MaskBit::Zero => 'X'
            }).collect();

            bitmasks.push(Mask::from(&bitmask.into_iter().collect::<String>()[..]));
        }

        bitmasks
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

    fn clone_from(&mut self, _: &Self) {
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

    pub fn set_part1(&mut self, address: usize, value: u64) {
        self.mem_map.insert(address, self.bitmask.apply(value));
    }

    pub fn set_part2(&mut self, address: usize, value: u64) {
        for bitmask in self.bitmask.translate_to_v2() {
            self.mem_map.insert(bitmask.apply(address as u64) as usize, value);
        }
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
        assert_eq!(Mask::from("XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X").apply(11), 73);
        assert_eq!(Mask::from("XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X").apply(101), 101);
        assert_eq!(Mask::from("XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X").apply(0), 64);
    }

    #[test]
    fn test_translate_to_v2() {
        assert_eq!(Mask::from("000000000000000000000000000000X1001X").translate_to_v2(), vec![
            Mask::from("XXXXXXXXXXXXXXXXXXXXXXXXXXXXXX01XX10"),
            Mask::from("XXXXXXXXXXXXXXXXXXXXXXXXXXXXXX01XX11"),
            Mask::from("XXXXXXXXXXXXXXXXXXXXXXXXXXXXXX11XX10"),
            Mask::from("XXXXXXXXXXXXXXXXXXXXXXXXXXXXXX11XX11"),
        ]);
    }

    #[test]
    fn test_system_part1() {
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
                    system.set_part1(address, value);
                }
            }
        }

        assert_eq!(system.sum(), 165);
    }

    #[test]
    fn test_system_part2() {
        let mut system = System::new();

        for instr in vec![Instruction::from("mask = 000000000000000000000000000000X1001X"),
                          Instruction::from("mem[42] = 100"),
                          Instruction::from("mask = 00000000000000000000000000000000X0XX"),
                          Instruction::from("mem[26] = 1"),
        ] {
            match instr {
                Instruction::SetMask(mask) => {
                    system.set_mask(mask);
                }
                Instruction::SetMem(address, value) => {
                    system.set_part2(address, value);
                }
            }
        }

        assert_eq!(system.sum(), 208);
    }
}