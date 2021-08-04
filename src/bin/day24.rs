use advent::pos::{Dir, Pos};
use std::collections::{HashMap, HashSet};
use std::hash::Hash;

fn main() {
    let input = advent::load_input_file(24);
    let initial = parse_input(input);
    println!("Part 1: {}", part1(&initial));
    println!("Part 2: {}", part2(&initial));
}

fn biodiversity(cells: HashSet<Pos>) -> u64 {
    let mut val = 1;
    let mut res = 0;
    for y in 0..5 {
        for x in 0..5 {
            if cells.contains(&Pos { x, y }) {
                res += val
            }
            val *= 2;
        }
    }
    res
}

fn parse_input(input: String) -> Vec<Pos> {
    let mut result = vec![];
    for (y, line) in input.lines().enumerate() {
        for (x, cell) in line.char_indices() {
            if cell == '#' {
                result.push(Pos {
                    x: x as i64,
                    y: y as i64,
                })
            }
        }
    }
    result
}

fn members<P: Copy + Ord>(world: &HashSet<P>) -> Vec<P> {
    let mut v = vec![];
    for &x in world {
        v.push(x)
    }
    v.sort();
    v
}

fn part1(initial: &[Pos]) -> u64 {
    let mut seen: HashSet<Vec<Pos>> = HashSet::new();
    let mut current: HashSet<Pos> = initial.iter().copied().collect();
    while !seen.contains(&members(&current)) {
        seen.insert(members(&current));
        current = step(&current, neighbors1)
    }
    biodiversity(current)
}

fn step<P, F>(start: &HashSet<P>, next: F) -> HashSet<P>
where
    P: Copy + Hash + Eq,
    F: Fn(P) -> Vec<P>,
{
    let mut m: HashMap<P, u64> = HashMap::new();
    for p in start.iter().copied().flat_map(next) {
        m.insert(p, m.get(&p).copied().unwrap_or(0) + 1);
    }

    m.into_iter()
        .filter_map(|(k, v)| {
            if v == 1 || v == 2 && !start.contains(&k) {
                Some(k)
            } else {
                None
            }
        })
        .collect()
}

fn neighbors1(pos: Pos) -> Vec<Pos> {
    Dir::ELEMS
        .iter()
        .map(|&x| pos.advance(x, 1))
        .filter(|p| 0 <= p.x && 0 <= p.y && p.x < 5 && p.y < 5)
        .collect()
}

fn part2(initial: &[Pos]) -> usize {
    let mut current: HashSet<(Pos, i64)> = HashSet::new();
    for &p in initial {
        current.insert((p - Pos { x: 2, y: 2 }, 0));
    }

    for _ in 0..200 {
        current = step(&current, neighbors2)
    }

    current.len()
}

fn neighbors2((pos, level): (Pos, i64)) -> Vec<(Pos, i64)> {

    type Pair<X> = (X, X);
    type PosFn = fn(Pos) -> Pos;
    let xforms: [Pair<PosFn>; 4] = [
        (|x| x, |x| x),
        (Pos::turn_around, Pos::turn_around),
        (Pos::turn_clockwise, Pos::turn_counterclockwise),
        (Pos::turn_counterclockwise, Pos::turn_clockwise),
    ];

    let mut result = vec![];
    for (f, g) in xforms {
        let mut pos = f(pos);
        let mut emit = |p, l| result.push((g(p), l));

        if pos.x == -2 {
            emit(Pos { x: -1, y: 0 }, level - 1)
        } else if pos.x == 1 && pos.y == 0 {
            for i in -2..=2 {
                emit(Pos { x: 2, y: i }, level + 1)
            }
        } else {
            pos.x -= 1;
            emit(pos, level)
        }
    }
    result
}
