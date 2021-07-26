use super::{Machine, Step};

pub struct MachineIterator<I> {
    machine: Machine,
    inputs: I,
}

impl<I: Iterator<Item = i64>> Iterator for MachineIterator<I> {
    type Item = i64;
    fn next(&mut self) -> Option<Self::Item> {
        loop {
            match self.machine.step() {                    
                Ok(Step::Output(o)) => return Some(o),
                Ok(Step::Input(i)) => match self.inputs.next() {
                    Some(x) => self.machine[i] = x,
                    None => {
                        return None
                    }
                },
                _ => {
                    return None
                }
            }
        }
    }
}

pub trait MachineIteratorExt: Iterator {
    fn machined(self, pgm: Vec<i64>) -> MachineIterator<Self>
    where
        Self: Sized,
    {
        MachineIterator {
            machine: Machine::new(pgm),
            inputs: self,
        }
    }
}

impl<I: Iterator<Item = i64>> MachineIteratorExt for I {}
