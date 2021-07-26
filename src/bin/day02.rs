use advent::intcode::Machine;

fn main() {
    let input = advent::load_input_file(2);
    let pgm = advent::intcode::parse_program(&input).unwrap();
    println!("Part 1: {}", solve1(&pgm));
    println!("Part 2: {}", solve2(&pgm));
}

fn eval(pgm: &[i64], x: i64, y: i64) -> i64 {
    let mut machine = Machine::new(pgm.to_vec());
    machine[1] = x;
    machine[2] = y;
    let _ = machine.step();
    machine[0]
}

fn solve1(pgm: &[i64]) -> i64 {
    eval(pgm, 12, 2)
}

fn solve2(pgm: &[i64]) -> i64 {
    for x in 0..100 {
        for y in 0..100 {
            if eval(pgm, x, y) == 19690720 {
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
        let input = advent::load_input_file(2);
        let pgm = advent::intcode::parse_program(&input).unwrap();
        assert_eq!(solve1(&pgm), 7210630);
        assert_eq!(solve2(&pgm), 3892);
    }
}