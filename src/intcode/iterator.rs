use super::{Machine, Step};

#[derive(Copy, Clone, Debug)]
enum State {
    Ready,
    Halted,
    Crashed,
}

pub struct MachineIterator<I> {
    machine: Machine,
    state: State,
    inputs: I,
}

impl<I: Iterator<Item = i64>> Iterator for MachineIterator<I> {
    type Item = i64;
    fn next(&mut self) -> Option<Self::Item> {
        match self.state {
            State::Ready => loop {
                match self.machine.step() {
                    Err(_) => {
                        self.state = State::Crashed;
                        return None;
                    }
                    Ok(Step::Halt) => {
                        self.state = State::Halted;
                        return None;
                    }
                    Ok(Step::Output(o)) => {
                        self.state = State::Ready;
                        return Some(o);
                    }
                    Ok(Step::Input(i)) => match self.inputs.next() {
                        None => {
                            self.state = State::Crashed;
                            return None;
                        }
                        Some(x) => self.machine[i] = x,
                    },
                }
            },
            _ => None,
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
            state: State::Ready,
            inputs: self,
        }
    }
}

impl<I: Iterator> MachineIteratorExt for I {}
