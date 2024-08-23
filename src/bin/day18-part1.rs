use std::{collections::HashMap, str::FromStr};

use itertools::Itertools;

#[derive(Clone, Copy)]
enum Operand {
    Num(isize),
    Reg(char),
}

impl FromStr for Operand {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(s.parse::<isize>()
            .map_or_else(|_| Self::Reg(s.parse().unwrap()), Self::Num))
    }
}

#[derive(Clone, Copy)]
enum Instruction {
    Snd(char),
    Set(char, Operand),
    Add(char, Operand),
    Mul(char, Operand),
    Mod(char, Operand),
    Rcv(char),
    Jgz(Operand, Operand),
}

impl Instruction {
    fn parse(s: &str) -> Self {
        let (instr, operands) = s.split_once(' ').unwrap();

        match instr {
            "set" => {
                let (x, y) = operands.split_once(' ').unwrap();
                Self::Set(x.parse().unwrap(), y.parse().unwrap())
            }

            "snd" => Self::Snd(operands.parse().unwrap()),

            "add" => {
                let (x, y) = operands.split_once(' ').unwrap();
                Self::Add(x.parse().unwrap(), y.parse().unwrap())
            }

            "mul" => {
                let (x, y) = operands.split_once(' ').unwrap();
                Self::Mul(x.parse().unwrap(), y.parse().unwrap())
            }

            "mod" => {
                let (x, y) = operands.split_once(' ').unwrap();
                Self::Mod(x.parse().unwrap(), y.parse().unwrap())
            }

            "rcv" => Self::Rcv(operands.parse().unwrap()),

            "jgz" => {
                let (x, y) = operands.split_once(' ').unwrap();
                Self::Jgz(x.parse().unwrap(), y.parse().unwrap())
            }

            _ => unreachable!(),
        }
    }
}

fn main() {
    let mut registers = HashMap::new();
    let mut last_played = 0;

    let instructions = include_str!(r"..\..\input\day18.txt")
        .lines()
        .map(Instruction::parse)
        .collect_vec();

    let mut index = 0;

    while let Some(&instruction) = instructions.get(index) {
        match instruction {
            Instruction::Set(x, y) => {
                let y = match y {
                    Operand::Num(num) => num,
                    Operand::Reg(reg) => registers.get(&reg).copied().unwrap_or(0),
                };

                registers.insert(x, y);
            }

            Instruction::Snd(x) => {
                last_played = registers.get(&x).copied().unwrap_or(0);
            }

            Instruction::Add(x, y) => {
                let y = match y {
                    Operand::Num(num) => num,
                    Operand::Reg(reg) => registers.get(&reg).copied().unwrap_or(0),
                };

                registers.insert(x, registers.get(&x).copied().unwrap_or(0) + (y));
            }

            Instruction::Mul(x, y) => {
                let y = match y {
                    Operand::Num(num) => num,
                    Operand::Reg(reg) => registers.get(&reg).copied().unwrap_or(0),
                };

                registers.insert(x, registers.get(&x).copied().unwrap_or(0) * (y));
            }

            Instruction::Mod(x, y) => {
                let y = match y {
                    Operand::Num(num) => num,
                    Operand::Reg(reg) => registers.get(&reg).copied().unwrap_or(0),
                };

                registers.insert(x, registers.get(&x).copied().unwrap_or(0) % (y));
            }

            Instruction::Jgz(x, y) => {
                let y = match y {
                    Operand::Num(num) => num,
                    Operand::Reg(reg) => registers.get(&reg).copied().unwrap_or(0),
                };

                let x = match x {
                    Operand::Num(num) => num,
                    Operand::Reg(reg) => registers.get(&reg).copied().unwrap_or(0),
                };

                if x > 0 {
                    index = index.checked_add_signed(y).unwrap();
                    continue;
                }
            }

            Instruction::Rcv(x) => {
                if registers.get(&x).copied().unwrap_or(0) != 0 {
                    dbg!(last_played);
                    break;
                }
            }
        }

        index += 1;
    }
}
