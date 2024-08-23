use std::{
    collections::HashMap,
    str::FromStr,
    sync::mpsc::{self, Receiver, Sender},
    thread,
    time::Duration,
};

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

fn run(
    instructions: &[Instruction],
    sender: &Sender<isize>,
    receiver: &Receiver<isize>,
    pid: isize,
) -> usize {
    let mut registers = HashMap::from([('p', pid)]);
    let mut index = 0;
    let mut sent_times = 0;

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
                sent_times += 1;
                sender
                    .send(registers.get(&x).copied().unwrap_or(0))
                    .unwrap();
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
                if let Ok(value) = receiver.recv_timeout(Duration::from_millis(500)) {
                    registers.insert(x, value);
                } else {
                    return sent_times;
                }
            }
        }

        index += 1;
    }

    sent_times
}

fn main() {
    let instructions1 = include_str!(r"..\..\input\day18.txt")
        .lines()
        .map(Instruction::parse)
        .collect_vec();
    let instructions2 = instructions1.clone();

    let (sender1, receiver1) = mpsc::channel();
    let (sender2, receiver2) = mpsc::channel();

    let handle1 = thread::spawn(move || run(&instructions1, &sender1, &receiver2, 0));
    let handle2 = thread::spawn(move || run(&instructions2, &sender2, &receiver1, 1));

    handle1.join().unwrap();
    let sent = handle2.join().unwrap();

    dbg!(sent);
}
