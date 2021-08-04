use gcd::Gcd;
use regex::Regex;
use std::str::FromStr;

const DIM: usize = 3;

#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
struct Element {
    pos: i64,
    vel: i64,
}

fn main() {
    let input: Vec<[i64; DIM]> = parse(&advent::load_input_file(12));
    let sims = transpose_sim(input);
    println!("Part 1: {}", part1(&sims));
    println!("Part 2: {}", part2(&sims));
}

fn transpose_sim(input: Vec<[i64; DIM]>) -> [Vec<Element>; DIM] {
    let f = |i| {
        input
            .iter()
            .map(|a| Element { pos: a[i], vel: 0 })
            .collect()
    };
    [f(0), f(1), f(2)]
}

fn part1(inputs: &[Vec<Element>]) -> i64 {
    let dims: Vec<Vec<Element>> = inputs
        .iter()
        .cloned()
        .map(|mut dim| {
            for _ in 0..1000 {
                step_simulation(&mut dim)
            }
            dim
        })
        .collect();

    dims[0]
        .iter()
        .zip(&dims[1])
        .zip(&dims[2])
        .map(|((x, y), z)| {
            (x.pos.abs() + y.pos.abs() + z.pos.abs()) * (x.vel.abs() + y.vel.abs() + z.vel.abs())
        })
        .sum()
}

fn part2(input: &[Vec<Element>]) -> u64 {
    input
        .iter()
        .map(|sim| cycle_length(sim))
        .reduce(lcm)
        .unwrap()
}

fn parse(input: &str) -> Vec<[i64; DIM]> {
    let regex: Regex = Regex::new("<x=(-?[0-9]+), y=(-?[0-9]+), z=(-?[0-9]+)>").unwrap();
    input
        .lines()
        .map(|line| {
            let caps = regex.captures(line).unwrap();
            let f = |i| i64::from_str(&caps[i]).unwrap();
            [f(1), f(2), f(3)]
        })
        .collect()
}

fn step_simulation(elements: &mut [Element]) {
    for i in 0..elements.len() {
        for j in i + 1..elements.len() {
            let acceleration = (elements[i].pos - elements[j].pos).signum();
            elements[i].vel -= acceleration;
            elements[j].vel += acceleration;
        }
    }
    for element in elements {
        element.pos += element.vel
    }
}

fn lcm(x: u64, y: u64) -> u64 {
    x / x.gcd(y) * y
}

fn cycle_length(elements: &[Element]) -> u64 {
    let mut current = elements.to_vec();
    step_simulation(&mut current);
    for n in 1.. {
        if current == elements {
            return n;
        } else {
            step_simulation(&mut current);
        }
    }
    panic!("overflow")
}
