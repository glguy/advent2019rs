use advent::intcode::{parse_program, Machine, Step};

fn main() {
    println!("Part 1: {}", solve1());
}

fn solve1() -> i64 {
    let input = advent::load_input_file(2);
    let pgm = parse_program(&input).unwrap();
    let mut machine = Machine::new(pgm);
    machine[1] = 12;
    machine[2] = 2;

    match machine.step() {
        Ok(Step::Halt) => {},
        _ => panic!("Missing output"),
    }

    machine[0]
}

fn solve2() -> i64 {
    let input = advent::load_input_file(2);
    let pgm = parse_program(&input).unwrap();
    let machine = Machine::new(pgm);

    for x in 0..100 {
        for y in 0..100 {
            let mut machine = machine.clone();
            machine[1] = x;
            machine[2] = y;
            match machine.step() {
                Ok(Step::Halt) => {},
                _ => panic!("Missing output"),
            }

            if machine[0] == 19690720 {
                return x * 100 + y;
            }
        }
    }

    panic!("no answer");
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn day11() {
        assert_eq!(solve1(), 7210630);
        assert_eq!(solve2(), 3892);
    }
}