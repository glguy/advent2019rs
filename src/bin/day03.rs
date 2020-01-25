use advent::pos::Pos;
use std::str::FromStr;

fn main() {
    let input: Vec<Vec<Cmd>> = advent::load_input_file(3).lines().map(parse_path).collect();

    println!("{:?}", input)
}

#[derive(Debug)]
struct Cmd {
    direction: Pos,
    distance: i64,
}

fn parse1(string: &str) -> Cmd {
    let direction = match &string[0..1] {
        "U" => Pos{x:0, y:-1},
        "D" => Pos{x:0, y:1},
        "L" => Pos{x:-1, y:0},
        "R" => Pos{x:1, y:0},
        _ => panic!("Bad direction"),
    };
    let distance = i64::from_str(&string[1..]).unwrap();
    Cmd { direction, distance }
}

fn parse_path(string: &str) -> Vec<Cmd> {
    string.split(',').map(parse1).collect()
}