use advent::intcode::{parse_program, Machine};

fn main() {
    let input = advent::load_input_file(19);
    let pgm = parse_program(&input).unwrap();
    let m = Machine::new(pgm);

    println!("Part 1: {}", part1(m.clone()));
    println!("Part 2: {}", part2(m));
}

fn query(mut m: Machine, x: i64, y: i64) -> bool {
    let i = m.step().unwrap().input().unwrap();
    m[i] = x;
    let i = m.step().unwrap().input().unwrap();
    m[i] = y;
    m.step().unwrap().output().unwrap() != 0
}

fn part1(m: Machine) -> usize {
    let mut n: usize = 0;
    for x in 0..50 {
        for y in 0..50 {
            if query(m.clone(), x, y) {
                n += 1;
            }
        }
    }
    n
}

fn part2(m: Machine) -> i64 {
    let mut y: i64 = 99;
    let mut x: i64 = 0;

    loop {
        while !query(m.clone(), x, y) {
            x += 1
        }
        if query(m.clone(), x+99, y-99) {
            break x * 10_000 + y-99
        }
        y += 1
    }
}
