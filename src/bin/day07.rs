use std::cell::RefCell;
use std::collections::VecDeque;
use std::iter::{from_fn, once};
use std::rc::Rc;
use advent::intcode::iterator::machine;
use permutohedron::Heap;

fn main() {
    let (p1, p2) = solve();
    println!("Part 1: {}", p1);
    println!("Part 2: {}", p2);
}

fn solve() -> (i64, i64) {
    let input = advent::load_input_file(7);
    let pgm = advent::intcode::parse_program(&input).unwrap();

    let p1 = Heap::new(&mut [0, 1, 2, 3, 4])
        .map(|params| make_controller(&pgm, &params).next().unwrap())
        .max()
        .unwrap();
    let p2 = Heap::new(&mut [5, 6, 7, 8, 9])
        .map(|params| make_controller(&pgm, &params).last().unwrap())
        .max()
        .unwrap();

    (p1, p2)
}

fn make_controller(pgm: &[i64], params: &[i64]) -> impl Iterator<Item = i64> {
    let loopback = Rc::new(RefCell::new(VecDeque::from(vec![0])));

    let front_loopback = loopback.clone();
    let loopbox: Box<dyn Iterator<Item = i64>> =
        Box::new(from_fn(move || front_loopback.borrow_mut().pop_front()));

    params
        .iter()
        .fold(loopbox, |it, &param| {
            Box::new(machine(pgm.to_vec(), once(param).chain(it)))
        })
        .map(move |x| {
            loopback.borrow_mut().push_back(x);
            x
        })
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
        let pgm = vec![
            3, 26, 1001, 26, -4, 26, 3, 27, 1002, 27, 2, 27, 1, 27, 26, 27, 4, 27, 1001, 28, -1,
            28, 1005, 28, 6, 99, 0, 0, 5,
        ];
        let params = [9, 8, 7, 6, 5];
        let controller = make_controller(&pgm, &params);
        let outputs = controller.collect::<Vec<i64>>();
        assert_eq!(outputs, vec![129, 4257, 136353, 4363425, 139629729]);
    }
}
