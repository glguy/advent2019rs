use advent::intcode::iterator::MachineIteratorExt;
use std::iter::once;

fn main() {
    let (p1,p2) = solve();
    println!("Part 1: {}", p1);
    println!("Part 2: {}", p2);
}

fn solve() -> (i64, i64) {
    let input = advent::load_input_file(9);
    let pgm = advent::intcode::parse_program(&input).unwrap();
    let p1 = once(1).machined(pgm.clone()).next().unwrap();
    let p2 = once(2).machined(pgm).next().unwrap();
    (p1, p2)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn day09() {
        let (p1, p2) = solve();
        assert_eq!(p1, 2941952859);
        assert_eq!(p2, 66113);
    }
}