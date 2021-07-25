use std::convert::TryFrom;
use std::num::ParseIntError;
use std::ops::{Index, IndexMut};
use std::str::FromStr;

pub fn parse_program(input: &str) -> Result<Vec<i64>, ParseIntError> {
    input.trim().split(',').map(|x| i64::from_str(x)).collect()
}

#[derive(Debug, Clone)]
pub struct Machine {
    pc: i64,
    rel_base: i64,
    memory: Vec<i64>,
}

impl Index<usize> for Machine {
    type Output = i64;
    fn index(&self, idx: usize) -> &i64 {
        match self.memory.get(idx) {
            None => &0,
            Some(val) => val,
        }
    }
}

impl IndexMut<usize> for Machine {
    fn index_mut(&mut self, idx: usize) -> &mut i64 {
        if self.memory.len() <= idx {
            let mut new_size = 512;
            while new_size <= idx {
                new_size *= 2;
            }
            self.memory.resize(new_size, 0)
        }
        &mut self.memory[idx]
    }
}

impl Machine {
    pub fn new(memory: Vec<i64>) -> Self {
        Machine {
            pc: 0,
            rel_base: 0,
            memory,
        }
    }

    fn arg_ptr(&self, opcode: i64, arg: i64) -> Result<usize, Error> {
        let pos = to_usize(self.pc + arg)?;
        match opcode / i64::pow(10, 1 + arg as u32) % 10 {
            0 => to_usize(self[pos]),
            1 => Ok(pos),
            2 => to_usize(self[pos] + self.rel_base),
            _ => Err(Error::BadParameterMode),
        }
    }

    pub fn step(&mut self) -> Result<Step, Error> {
        let mut opcode;

        macro_rules! read {
            ( $arg:literal ) => {{
                let pos = self.arg_ptr(opcode, $arg)?;
                self[pos]
            }};
        }

        macro_rules! jump {
            ( $pc:expr, $res:expr ) => {{
                let r = $res; // <- must be computed before pc updates
                self.pc = $pc;
                return Ok(r);
            }};
        }

        macro_rules! simple {
            ( $arg:literal, $val:expr ) => {{
                let v = $val;
                let pos = self.arg_ptr(opcode, $arg)?;
                self[pos] = v;
                self.pc += $arg + 1;
            }};
        }

        macro_rules! arithmetic {
            ( $op:expr ) => {{
                let v1 = read!(1);
                let v2 = read!(2);
                let r = $op(v1, v2).ok_or(Error::ArithmeticOverflow)?;
                let pos = self.arg_ptr(opcode, 3)?;
                self[pos] = r;
                self.pc += 4;
            }};
        }

        loop {
            opcode = self[to_usize(self.pc)?];

            match opcode % 100 {
                1 => arithmetic!(i64::checked_add),
                2 => arithmetic!(i64::checked_mul),
                3 => jump!(self.pc + 2, Step::Input(self.arg_ptr(opcode, 1)?)),
                4 => jump!(self.pc + 2, Step::Output(read!(1))),
                5 => self.pc = if read!(1) != 0 { read!(2) } else { self.pc + 3 },
                6 => self.pc = if read!(1) == 0 { read!(2) } else { self.pc + 3 },
                7 => simple!(3, (read!(1) < read!(2)) as i64),
                8 => simple!(3, (read!(1) == read!(2)) as i64),
                9 => {
                    self.rel_base += read!(1);
                    self.pc += 2
                }
                99 => break Ok(Step::Halt),
                _ => break Err(Error::BadOpcode),
            }
        }
    }
}

fn to_usize(val: i64) -> Result<usize, Error> {
    match usize::try_from(val) {
        Ok(uval) => Ok(uval),
        Err(_) => Err(Error::BadAddress),
    }
}

#[derive(Debug, Eq, PartialEq, Clone, Copy)]
pub enum Step {
    Halt,
    Output(i64),
    Input(usize),
}

impl Step {
    pub fn output(self) -> Option<i64> {
        match self {
            Self::Output(o) => Some(o),
            _ => None,
        }
    }

    pub fn input(self) -> Option<usize> {
        match self {
            Self::Input(i) => Some(i),
            _ => None,
        }
    }
}

#[derive(Debug, Eq, PartialEq, Clone, Copy)]
pub enum Error {
    BadAddress,
    BadOpcode,
    BadParameterMode,
    ArithmeticOverflow,
}

pub fn simple_machine(pgm: &[i64], inputs: &[i64]) -> Option<Vec<i64>> {
    let mut m = Machine::new(pgm.to_vec());
    let mut outputs = vec![];
    let mut i = 0;

    while let Ok(res) = m.step() {
        match res {
            Step::Halt => return Some(outputs),
            Step::Output(o) => outputs.push(o),
            Step::Input(pos) => match inputs.get(i) {
                None => break,
                Some(&val) => {
                    m[pos] = val;
                    i += 1
                }
            },
        }
    }
    None
}

pub fn to_ascii(vals: &[i64]) -> String {
    vals.iter().map(|&x| x as u8 as char).collect()
}

pub fn from_ascii(val: &str) -> Vec<i64> {
    val.bytes().map(|x| x as i64).collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn it_works() {
        let pgm = [3, 12, 6, 12, 15, 1, 13, 14, 13, 4, 13, 99, -1, 0, 1, 9];
        assert_eq!(simple_machine(&pgm, &[0]), Some(vec![0]));
        assert_eq!(simple_machine(&pgm, &[10]), Some(vec![1]));
    }

    #[test]
    fn day9_tests() {
        let pgm = [1102, 34915192, 34915192, 7, 4, 7, 99, 0];
        assert_eq!(simple_machine(&pgm, &[]), Some(vec![1219070632396864]));
        let quine = [
            109, 1, 204, -1, 1001, 100, 1, 100, 1008, 100, 16, 101, 1006, 101, 0, 99,
        ];
        assert_eq!(simple_machine(&quine, &[]), Some(quine.to_vec()));
    }

    #[test]
    fn compare_test() {
        let pgm = [
            3, 21, 1008, 21, 8, 20, 1005, 20, 22, 107, 8, 21, 20, 1006, 20, 31, 1106, 0, 36, 98, 0,
            0, 1002, 21, 125, 20, 4, 20, 1105, 1, 46, 104, 999, 1105, 1, 46, 1101, 1000, 1, 20, 4,
            20, 1105, 1, 46, 98, 99,
        ];
        assert_eq!(simple_machine(&pgm, &[7]), Some(vec![999]));
        assert_eq!(simple_machine(&pgm, &[8]), Some(vec![1000]));
        assert_eq!(simple_machine(&pgm, &[9]), Some(vec![1001]));
    }

    #[test]
    fn day02() {
        let input =
            fs::read_to_string("/Users/emertens/Source/advent2019/inputs/input02.txt").unwrap();
        let pgm = parse_program(&input).unwrap();
        let mut m = Machine::new(pgm);
        m[1] = 12;
        m[2] = 2;
        loop {
            match m.step() {
                Ok(Step::Halt) => break,
                _ => {}
            }
        }
        assert_eq!(m[0], 7210630);
    }
}
