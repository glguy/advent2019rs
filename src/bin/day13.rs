use advent::intcode::iterator::machine;
use itertools::Itertools;
use std::cell::Cell;
use std::collections::HashMap;

fn main() {
    let input = advent::load_input_file(13);
    let pgm = advent::intcode::parse_program(&input).unwrap();
    println!("{}", part1(pgm.clone()));
    println!("{}", part2(pgm));
}

fn part1(pgm: Vec<i64>) -> usize {
    type World = HashMap<(i64, i64), i64>;
    machine(pgm, [])
        .chunks(3)
        .into_iter()
        .fold(HashMap::new(), |mut w: World, mut c| {
            let x = c.next().unwrap();
            let y = c.next().unwrap();
            let t = c.next().unwrap();
            w.insert((x, y), t);
            w
        })
        .values()
        .filter(|&&i| i == 2)
        .count()
}

fn part2(mut pgm: Vec<i64>) -> i64 {
    let ball: Cell<i64> = Default::default();
    let plat: Cell<i64> = Default::default();
    let mut score: i64 = 0;
    pgm[0] = 2; // enable free play mode

    let inputs = std::iter::from_fn(|| Some((ball.get() - plat.get()).signum()));

    for mut chunk in machine(pgm, inputs).chunks(3).into_iter() {
        let x = chunk.next().unwrap();
        let y = chunk.next().unwrap();
        let t = chunk.next().unwrap();
        if x == -1 && y == 0 {
            score = t
        } else if t == 3 {
            plat.set(x)
        } else if t == 4 {
            ball.set(x)
        }
    }
    score
}
