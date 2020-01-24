use advent::intcode::{parse_program, Machine, Step};
use std::collections::HashMap;
use advent::pos::Pos;

fn main() {
    println!("Part 1: {}", solve1());
    solve2();
}

fn solve1() -> usize {
    let input = advent::load_input_file(11);
    let pgm = parse_program(&input).unwrap();
    let machine = Machine::new(pgm);
    let paint = painter(machine, 0);
    paint.len()
}

fn solve2() {
    let input = advent::load_input_file(11);
    let pgm = parse_program(&input).unwrap();
    let machine = Machine::new(pgm);
    let paint = painter(machine, 1);

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

fn painter(mut machine: Machine, start: i64) -> HashMap<Pos, i64> {
    let mut here = Pos::ORIGIN;
    let mut paint = HashMap::new();
    let mut is_color = true;
    let mut dir = Pos { x:0, y:-1 };

    paint.insert(here, start);

    loop {
        match machine.step().unwrap() {
            Step::Halt => return paint,
            Step::Output(o) => {
                if is_color {
                    paint.insert(here, o);
                } else {
                    dir = turn(dir, o);
                    here += dir;
                }
                is_color = !is_color;
            }
            Step::Input(i) =>
                machine[i] = *paint.get(&here).unwrap_or(&0),
        }
    }
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