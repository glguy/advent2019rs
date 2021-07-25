use advent::intcode::{from_ascii, parse_program, simple_machine, to_ascii};
use advent::load_input_file;
use advent::pos::{Dir, Pos};
use std::collections::HashSet;

const SUBLEN: usize = 20;
const SUBNAMES: [&'static str; 3] = ["A", "B", "C"];

#[derive(Copy, Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
enum Turn {
    /// Right turn
    R,
    /// Left turn 
    L,
}

/// A sequence of turns and steps to give to the robot
type Instructions<'a> = &'a [(Turn, i64)];

/// Parse the input file and print solutions to both parts 
fn main() {
    let input = load_input_file(17);
    let mut pgm = parse_program(&input).unwrap();
    let output = to_ascii(&simple_machine(&pgm, &[]).unwrap());
    let (start, world) = scan_map(&output);
    print!("Part 1: {}\n", part1(&world));
    print!("Part 2: {}\n", part2(&mut pgm, &world, start));
}

/// Predicate for locations surrounded by positions in the world
fn is_surrounded(world: &HashSet<Pos>, p: Pos) -> bool {
    Dir::ELEMS.iter().all(|&x| world.contains(&p.advance(x, 1)))
}

/// Computes part 1 score for locations of girder intersections
fn part1(world: &HashSet<Pos>) -> i64 {
    world
        .iter()
        .filter_map(|&p| {
            if is_surrounded(world, p) {
                Some(p.x * p.y)
            } else {
                None
            }
        })
        .sum()
}

/// Compute a program that guides the robot across the whole world map,
/// run that program, and find the final score value.
fn part2(pgm: &mut [i64], world: &HashSet<Pos>, start: Pos) -> i64 {
    // Solve the uncompressed robot instruction sequence
    let path = world_path(&world, start);

    // Compute the intcode program input that solves the puzzle
    let input_string = build_program(&path);

    // Switch program into interactive mode
    pgm[0] = 2;

    // Run program with computed input values and return the final output value
    simple_machine(&pgm, &from_ascii(&input_string))
        .unwrap()
        .last()
        .unwrap()
        .to_owned()
}

// Compute the ASCII input that solves the robot puzzle
fn build_program(path: Instructions) -> String {
    // Search for a valid main and sub routines
    let (main_routine, subroutines) = program_search(&path).unwrap();

    // Build corresponding text input
    let mut input_string = main_routine.join(",");
    input_string += "\n";
    for subroutine in subroutines {
        input_string += &render_instructions(&subroutine);
        input_string += "\n";
    }
    input_string += "no\n";
    input_string
}

// Convert intcode output into a starting location and world map
fn scan_map(output: &str) -> (Pos, HashSet<Pos>) {
    let mut start = Pos::ORIGIN;
    let mut world = HashSet::new();
    for (y, line) in output.lines().enumerate() {
        for (x, cell) in line.chars().enumerate() {
            let p = Pos {
                x: x as i64,
                y: y as i64,
            };
            match cell {
                '#' => {
                    world.insert(p);
                }
                '^' => {
                    world.insert(p);
                    start = p
                }
                '.' => {}
                _ => panic!("unexpected cell: {}", cell),
            }
        }
    }
    (start, world)
}

// Convert the world map and starting point into a sequence
// of turn and advance instructions
fn world_path(world: &HashSet<Pos>, mut here: Pos) -> Vec<(Turn, i64)> {
    let mut result = vec![];
    let mut face = Dir::N;

    'outer: loop {
        for (turn, op) in [
            (Turn::R, Dir::turn_clockwise as fn(Dir) -> Dir),
            (Turn::L, Dir::turn_around),
        ] {
            face = op(face);
            let found = (1..)
                .into_iter()
                .map(|n| (n, here.advance(face, n)))
                .take_while(|(_, p)| world.contains(p))
                .last();
            if let Some((n, p)) = found {
                result.push((turn, n));
                here = p;
                continue 'outer;
            }
        }
        return result;
    }
}

// Render a subroutine as an ASCII input string
fn render_instructions(instructions: Instructions) -> String {
    instructions
        .iter()
        .map(|&(t, n)| format!("{:?},{}", t, n))
        .collect::<Vec<String>>()
        .join(",")
}


fn program_search<'a>(
    mut target: Instructions<'a>,
) -> Option<(Vec<&'static str>, Vec<Instructions<'a>>)> {
    let mut main_routine = vec![];
    let mut subroutines = vec![];
    let mut n = 0;

    struct History<'a> {
        n: usize,
        target: Instructions<'a>,
        allocated: bool,
    }
    let mut history: Vec<History> = vec![];

    while !target.is_empty() {
        if n >= target.len() {
            match history.pop() {
                None => return None,
                Some(prev) => {
                    n = prev.n;
                    target = prev.target;
                    if prev.allocated {
                        subroutines.pop();
                    }
                    main_routine.pop();
                }
            }
        }
        n += 1;

        let (piece, rest) = target.split_at(n);
        match subroutines.iter().position(|&sub| sub == piece) {
            // This fragment is already named
            Some(i) => {
                main_routine.push(SUBNAMES[i]);

                history.push(History {
                    n,
                    target,
                    allocated: false,
                });
                n = 0;
                target = rest;
            }
            // This fragment is not yet named
            None => {
                if render_instructions(piece).len() > SUBLEN {
                    // The fragment is too long to be named
                    // All future fragments would be too, so skip to the end
                    n = usize::MAX;
                } else if subroutines.len() < SUBNAMES.len() {
                    // Allocate a new named fragment
                    main_routine.push(SUBNAMES[subroutines.len()]);
                    subroutines.push(piece);

                    history.push(History {
                        n,
                        target,
                        allocated: true,
                    });
                    n = 0;
                    target = rest;
                }
            }
        }
    }

    Some((main_routine, subroutines))
}

/*

fn program_search<'a>(
    target: Instructions<'a>,
) -> Option<(Vec<&'static str>, Vec<Instructions<'a>>)> {
    fn go<'a>(
        main_routine: &mut Vec<&'static str>,
        subroutines: &mut Vec<Instructions<'a>>,
        target: &'a [(Turn, i64)],
    ) -> bool {
        if target.is_empty() {
            return true;
        }

        for n in 1..=target.len() {
            let (piece, target) = target.split_at(n);
            match subroutines.iter().position(|&sub| sub == piece) {
                Some(i) => {
                    main_routine.push(SUBNAMES[i]);
                    if go(main_routine, subroutines, target) {
                        return true;
                    }
                    main_routine.pop();
                }
                None => {
                    if render_instructions(piece).len() > SUBLEN {
                        return false;
                    }

                    if subroutines.len() < SUBNAMES.len() {
                        main_routine.push(SUBNAMES[subroutines.len()]);
                        subroutines.push(piece);
                        if go(main_routine, subroutines, target) {
                            return true;
                        }
                        subroutines.pop();
                        main_routine.pop();
                    }
                }
            }
        }
        false
    }
    let mut main_routine = vec![];
    let mut subroutines = vec![];
    if go(&mut main_routine, &mut subroutines, target) {
        Some((main_routine, subroutines))
    } else {
        None
    }
}
*/