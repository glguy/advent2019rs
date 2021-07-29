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
                    None => return None,
                },
                Ok(Step::Halt) => return None,
                Err(e) => panic!("Bad machine iterator: {:?}", e),
            }
        }
    }
}

pub fn machine<I>(pgm: Vec<i64>, inputs: I) -> MachineIterator<I::IntoIter>
where
    I: IntoIterator<Item = i64>,
{
    MachineIterator {
        machine: Machine::new(pgm),
        inputs: inputs.into_iter(),
    }
}
