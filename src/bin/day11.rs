use advent::intcode::parse_program;
use advent::intcode::iterator::machine;
use std::collections::HashMap;
use advent::pos::Pos;
use std::cell::Cell;
use itertools::Itertools;

fn main() {
    println!("Part 1: {}", solve1());
    solve2();
}

fn solve1() -> usize {
    let input = advent::load_input_file(11);
    let pgm = parse_program(&input).unwrap();
    let paint = painter(pgm, 0);
    paint.len()
}

fn solve2() {
    let input = advent::load_input_file(11);
    let pgm = parse_program(&input).unwrap();
    let paint = painter(pgm, 1);

    let min_x = paint.keys().map(|x|x.x).min().unwrap();
    let min_y = paint.keys().map(|x|x.y).min().unwrap();
    let max_x = paint.keys().map(|x|x.x).max().unwrap();
    let max_y = paint.keys().map(|x|x.y).max().unwrap();

    for y in min_y ..= max_y {
        for x in min_x ..= max_x {
            let c = match paint.get(&Pos{x,y}) {
                Some(0) => '░',
                Some(1) => '█',
                None => ' ',
                _ => '!',
            };
            print!("{}", c)
        }
        println!();
    }
}

fn painter(pgm: Vec<i64>, start: i64) -> HashMap<Pos, i64> {
    let mut here = Pos::ORIGIN;
    let mut paint = HashMap::new();    
    let mut dir = Pos { x:0, y:-1 };
    let current = Cell::new(start);

    paint.insert(here, start);

    let inputs = std::iter::from_fn(|| Some(current.get()));

    for mut command in machine(pgm, inputs).chunks(2).into_iter() {
        let color = command.next().unwrap();
        let look = command.next().unwrap();
        paint.insert(here, color);
        dir = turn(dir, look);
        here += dir;
        current.set(paint.get(&here).copied().unwrap_or(0))
    }
    paint
}

fn turn(dir: Pos, turn: i64) -> Pos {
    match turn {
        0 => dir.turn_counterclockwise(),
        1 => dir.turn_clockwise(),
        _ => panic!("bad turn code"),
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn day11() {
        assert_eq!(solve1(), 1876);
    }
}
