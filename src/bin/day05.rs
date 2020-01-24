use advent::intcode::{parse_program, simple_machine};

fn main() {
    let (p1, p2) = solve();
    println!("Part 1: {}", p1);
    println!("Part 2: {}", p2);
}

fn solve() -> (i64, i64) {
    let input = advent::load_input_file(5);
    let pgm = parse_program(&input).unwrap();

    let &p1 = simple_machine(&pgm, &[1]).unwrap().last().unwrap();
    let &p2 = simple_machine(&pgm, &[5]).unwrap().last().unwrap();

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