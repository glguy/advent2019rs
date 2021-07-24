use advent::pos::Pos;
use std::str::FromStr;
use std::iter;
use std::collections::HashMap;

fn main() {
    let mut input: Vec<Vec<Cmd>> = advent::load_input_file(3).lines().map(parse_path).collect();
    let path2 = input.remove(1);
    let path1 = input.remove(0);

    let mut grid1: HashMap<Pos, usize> = HashMap::new();
    generate_path(path1)
        .enumerate()
        .for_each(|(i, x)| { grid1.insert(x, i + 1); });

    let mut p1 = i64::max_value();
    let mut p2 = usize::max_value();
    for (i, x) in generate_path(path2).enumerate() {
        if let Some(&j) = grid1.get(&x) {
            p1 = p1.min(x.norm1());
            p2 = p2.min(1 + i + j)
        }
    }
    println!("Part 1: {}", p1);
    println!("Part 2: {}", p2);
}

#[derive(Debug, Clone, Copy)]
struct Cmd {
    direction: Pos,
    distance: i64,
}

fn parse1(string: &str) -> Cmd {
    let direction = match &string[0..1] {
        "U" => Pos { x: 0, y: -1 },
        "D" => Pos { x: 0, y: 1 },
        "L" => Pos { x: -1, y: 0 },
        "R" => Pos { x: 1, y: 0 },
        _ => panic!("Bad direction"),
    };
    let distance = i64::from_str(&string[1..]).unwrap();
    Cmd { direction, distance }
}

fn parse_path(string: &str) -> Vec<Cmd> {
    string.split(',').map(parse1).collect()
}

fn generate_path(cmds: Vec<Cmd>) -> impl Iterator<Item=Pos> {
    cmds.into_iter()
        .flat_map(|cmd| iter::repeat(cmd.direction).take(cmd.distance as usize))
        .scan(Pos::ORIGIN, |st, x| {
            *st += x;
            Some(*st)
        })
}