use advent::load_input_file;
use std::str::FromStr;
use std::iter;

fn main() {
    let input = load_input_file(1);

    let numbers: Vec<i64> = input.lines().flat_map(i64::from_str).collect();

    let p1: i64 = numbers.iter().cloned().map(fuel1).sum();
    println!("Part 1: {}", p1);

    let p2: i64 = numbers.into_iter().map(fuel2).sum();
    println!("Part 2: {}", p2);
}

fn fuel1(weight: i64) -> i64 {
    weight / 3 - 2
}

fn fuel2(weight: i64) -> i64 {

    fn step(& (mut x): &i64) -> Option<i64> {
            x = fuel1(x);
            if x > 0 { Some(x) } else { None }
    }

    iter::successors(Some(weight), step).skip(1).sum()
}