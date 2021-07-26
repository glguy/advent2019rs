use advent::intcode::{parse_program, Machine, Step};
use std::collections::HashMap;

fn main() {
    let input = advent::load_input_file(13);
    let pgm = parse_program(&input).unwrap();
    let m = Machine::new(pgm);

    println!("{}", part1(m.clone()));
    println!("{}", part2(m));
}

fn part1(mut m: Machine) -> usize {
    let mut world: HashMap<(i64, i64), i64> = HashMap::new();
    loop {
        match m.step().unwrap() {
            Step::Input(_) => panic!("halted during part 1"),
            Step::Output(x) => {
                let y = m.step().unwrap().output().unwrap();
                let t = m.step().unwrap().output().unwrap();
                world.insert((x, y), t);
            }
            Step::Halt => break world.values().filter(|&&i| i == 2).count(),
        }
    }
}

fn part2(mut m: Machine) -> i64 {
    let mut ball: i64 = 0;
    let mut plat: i64 = 0;
    let mut score: i64 = 0;
    m[0] = 2; // enable free play mode
    loop {
        match m.step().unwrap() {
            Step::Input(i) => m[i] = (ball - plat).signum(),
            Step::Output(x) => {
                let y = m.step().unwrap().output().unwrap();
                let t = m.step().unwrap().output().unwrap();
                if x == -1 && y == 0 {
                    score = t
                } else if t == 3 {
                    plat = x
                } else if t == 4 {
                    ball = x
                }
            }
            Step::Halt => break score,
        }
    }
}
