use advent::intcode::iterator::machine;

fn main() {
    let (p1, p2) = solve();
    println!("Part 1: {}", p1);
    println!("Part 2: {}", p2);
}

fn solve() -> (i64, i64) {
    let input = advent::load_input_file(5);
    let pgm = advent::intcode::parse_program(&input).unwrap();

    let p1 = machine(pgm.clone(), [1]).last().unwrap();
    let p2 = machine(pgm, [5]).last().unwrap();
    (p1, p2)
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn day05() {
        let (p1, p2) = solve();
        assert_eq!(p1, 15508323);
        assert_eq!(p2, 9006327);
    }
}