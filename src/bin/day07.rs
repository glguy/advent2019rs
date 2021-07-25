use advent::intcode::iterator::MachineIteratorExt;
use advent::intcode::parse_program;
use std::cell::RefCell;
use std::collections::VecDeque;
use std::iter::from_fn;
use std::rc::Rc;

fn main() {
    let (p1, p2) = solve();
    println!("Part 1: {}", p1);
    println!("Part 2: {}", p2);
}

fn solve() -> (i64, i64) {
    let input = advent::load_input_file(7);
    let pgm = parse_program(&input).unwrap();
    let mut params = [0, 1, 2, 3, 4];
    let mut p1 = 0;

    while {
        let mut controller = make_controller(&pgm, &params);
        p1 = p1.max(controller.nth(0).unwrap());
        next_permutation(&mut params)
    } {}
    let mut params = [5, 6, 7, 8, 9];
    let mut p2 = 0;
    while {
        let controller = make_controller(&pgm, &params);
        p2 = p2.max(controller.last().unwrap());
        next_permutation(&mut params)
    } {}

    (p1, p2)
}

fn next_permutation<V: Ord>(array: &mut [V]) -> bool {
    let mut i = array.len() - 1;
    while i > 0 && array[i - 1] >= array[i] {
        i -= 1
    }

    if i <= 0 {
        return false;
    }

    let mut j = array.len() - 1;
    while array[j] <= array[i - 1] {
        j -= 1
    }

    assert!(j >= i);

    array.swap(i - 1, j);
    array[i..].reverse();

    true
}

fn make_controller(pgm: &[i64], params: &[i64]) -> impl Iterator<Item = i64> {
    let loopback = Rc::new(RefCell::new(VecDeque::from(vec![0])));

    let front_loopback = loopback.clone();
    let loopbox: Box<dyn Iterator<Item = i64>> =
        Box::new(from_fn(move || front_loopback.borrow_mut().pop_front()));

    params
        .iter()
        .fold(loopbox, |it, &p| {
            Box::new(vec![p].into_iter().chain(it).machined(pgm.to_vec()))
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
