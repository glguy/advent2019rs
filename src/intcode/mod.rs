pub mod iterator;

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
        self.memory.get(idx).unwrap_or(&0)
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
        loop {
            let opcode = self[to_usize(self.pc)?];

            macro_rules! ptr {
                ( $arg:literal ) => { self.arg_ptr(opcode, $arg)? }
            }

            macro_rules! val {
                ( $arg:literal ) => { self[ptr!($arg)] };
            }

            macro_rules! io {
                ( $res:expr ) => {{
                    let r = $res; // <- must be computed before pc updates
                    self.pc += 2;
                    break Ok(r);
                }};
            }

            macro_rules! compute {
                ( $val:expr ) => {{
                    let p = ptr!(3);
                    self[p] = $val;
                    self.pc += 4;
                }};
            }

            match opcode % 100 {
                1 => compute!(val!(1).checked_add(val!(2)).ok_or(Error::ArithmeticOverflow)?),
                2 => compute!(val!(1).checked_mul(val!(2)).ok_or(Error::ArithmeticOverflow)?),
                3 => io!(Step::Input(ptr!(1))),
                4 => io!(Step::Output(val!(1))),
                5 => if val!(1) != 0 { self.pc = val!(2) } else { self.pc += 3 },
                6 => if val!(1) == 0 { self.pc = val!(2) } else { self.pc += 3 },
                7 => compute!((val!(1) < val!(2)) as i64),
                8 => compute!((val!(1) == val!(2)) as i64),
                9 => {
                    self.rel_base += val!(1);
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

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    fn run<I: IntoIterator<Item = i64>>(pgm: &[i64], input: I) -> Vec<i64> {
        use iterator::machine;
        machine(input, pgm.to_vec()).collect()
    }

    #[test]
    fn it_works() {
        let pgm = [3, 12, 6, 12, 15, 1, 13, 14, 13, 4, 13, 99, -1, 0, 1, 9];
        assert_eq!(run(&pgm, [0]), vec![0]);
        assert_eq!(run(&pgm, [10]), vec![1]);
    }

    #[test]
    fn day9_tests() {
        let pgm = [1102, 34915192, 34915192, 7, 4, 7, 99, 0];
        assert_eq!(run(&pgm, []), vec![1219070632396864]);
        let quine = [
            109, 1, 204, -1, 1001, 100, 1, 100, 1008, 100, 16, 101, 1006, 101, 0, 99,
        ];
        assert_eq!(run(&quine, []), quine.to_vec());
   }

    #[test]
    fn compare_test() {
        let pgm = [
            3, 21, 1008, 21, 8, 20, 1005, 20, 22, 107, 8, 21, 20, 1006, 20, 31, 1106, 0, 36, 98, 0,
            0, 1002, 21, 125, 20, 4, 20, 1105, 1, 46, 104, 999, 1105, 1, 46, 1101, 1000, 1, 20, 4,
            20, 1105, 1, 46, 98, 99,
        ];
        assert_eq!(run(&pgm, [7]), vec![999]);
        assert_eq!(run(&pgm, [8]), vec![1000]);
        assert_eq!(run(&pgm, [9]), vec![1001]);
    }

    #[test]
    fn day02() {
        let input =
            fs::read_to_string("/Users/emertens/Source/advent2019/inputs/input02.txt").unwrap();
        let pgm = parse_program(&input).unwrap();
        let mut m = Machine::new(pgm);
        m[1] = 12;
        m[2] = 2;
        let _ = m.step();
        assert_eq!(m[0], 7210630);
    }
}
