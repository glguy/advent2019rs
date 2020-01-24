use advent::intcode::{parse_program, Machine, Step};
use std::collections::VecDeque;

fn main() {
    let (p1, p2) = solve();
    println!("Part 1: {}", p1);
    println!("Part 2: {}", p2);


}

fn solve() -> (i64, i64) {
    let input = advent::load_input_file(7);
    let pgm = parse_program(&input).unwrap();
    let machine = Machine::new(pgm);
    let mut params = [0, 1, 2, 3, 4];
    let mut p1 = 0;

    while {
        let mut controller = Controller::new(machine.clone(), &params);
        p1 = p1.max(controller.nth(0).unwrap());
        next_permutation(&mut params)
    }{}
    
    let mut params = [5, 6, 7, 8, 9];
    let mut p2 = 0;
    while {
        let controller = Controller::new(machine.clone(), &params);
        p2 = p2.max(controller.last().unwrap());
        next_permutation(&mut params)
    }{}

    (p1, p2)
}

fn next_permutation<V: Ord>(array: &mut [V]) -> bool {

    let mut i = array.len() - 1;
    while i > 0 && array[i-1] >= array[i] { i -= 1 }

    if i <= 0 { return false }

    let mut j = array.len() - 1;
    while array[j] <= array[i - 1] { j -= 1 }

    assert!(j >= i);

    array.swap(i - 1, j);
    array[i ..].reverse();

    true
}

struct Controller {
    machines: Vec<Machine>,
    loopback: VecDeque<i64>,
}

impl Controller {
    fn new(machine: Machine, params: &[i64]) -> Self {
        let mut machines = vec![];
        for &param in params.iter().rev() {
            let mut machine = machine.clone();
            match machine.step().unwrap() {
                Step::Input(i) => {
                    machine[i] = param;
                    machines.push(machine);
                }
                _ => panic!("didn't need input?"),
            }
        }

        Controller {
            machines,
            loopback: VecDeque::from(vec![0]),
        }
    }
}

impl Iterator for Controller {
    type Item = i64;

    fn next(&mut self) -> Option<Self::Item> {
        let mut stalls = vec![];

        loop {
            let stalls_n = stalls.len();
            match self.machines[stalls_n].step().unwrap() {
                Step::Output(o) => {
                    match stalls.pop() {
                        None => {
                            self.loopback.push_back(o);
                            return Some(o);
                        }
                        Some(i) => {
                            self.machines[stalls.len()][i] = o;
                        }
                    }
                }
                Step::Input(i) => {
                    if stalls_n + 1 == self.machines.len() {
                        let o = self.loopback.pop_front().expect("infinite loop");
                        self.machines[stalls_n][i] = o;
                    } else {
                        stalls.push(i);
                    }
                }
                Step::Halt => return None,
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn day07() {
        let (p1, p2) = solve();
        assert_eq!(p1, 34852);
        assert_eq!(p2, 44282086);
    }

    #[test]
    fn example() {
        let pgm = vec![3, 26, 1001, 26, -4, 26, 3, 27, 1002, 27, 2, 27, 1, 27, 26, 27, 4, 27, 1001, 28, -1, 28, 1005, 28, 6, 99, 0, 0, 5];
        let params = [9, 8, 7, 6, 5];
        let machine = Machine::new(pgm);
        let controller = Controller::new(machine, &params);
        let outputs = controller.collect::<Vec<i64>>();
        assert_eq!(outputs, vec![129, 4257, 136353, 4363425, 139629729]);
    }
}