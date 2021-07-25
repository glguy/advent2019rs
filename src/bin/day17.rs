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
    let (world, start) = scan_map(&output);
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
        .filter(|&&p| is_surrounded(world, p))
        .map(|p| p.x * p.y)
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
    *simple_machine(&pgm, &from_ascii(&input_string))
        .unwrap()
        .last()
        .unwrap()
}

// Compute the ASCII input that solves the robot puzzle
fn build_program(path: Instructions) -> String {
    // Search for a valid main and sub routines
    let (main_routine, subroutines) = program_search(&path).unwrap();

    // Build corresponding text input
    let mut input_string = main_routine
        .into_iter()
        .map(|i| SUBNAMES[i])
        .collect::<Vec<&'static str>>()
        .join(",");
    input_string += "\n";
    for subroutine in subroutines {
        input_string += &render_instructions(&subroutine);
        input_string += "\n";
    }
    input_string += "no\n";
    input_string
}

// Convert intcode output into a starting location and world map
fn scan_map(output: &str) -> (HashSet<Pos>, Pos) {
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
    (world, start)
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

// Find the main routine and list of subroutines that generates a given
// instruction sequence while fitting within the subroutine constraints
fn program_search<'a>(target: Instructions<'a>) -> Option<(Vec<usize>, Vec<Instructions<'a>>)> {
    let mut main_routine: Vec<usize> = vec![];
    let mut subroutines: Vec<(usize, usize)> = vec![];
    let mut lo: usize = 0;
    let mut hi: usize = 0;

    while lo < target.len() {
        if hi >= target.len() {
            // rollback latest subroutine call
            match main_routine.pop() {
                None => return None,
                Some(j) => {
                    hi = lo;
                    let (slo,shi) = subroutines[j];
                    lo -= shi - slo;
                    if lo == slo {
                        subroutines.pop();
                    }
                }
            }
        }
        hi += 1;

        let piece = &target[lo..hi];

        // Check if this piece already has a name
        match subroutines.iter().position(|&(a,b)| &target[a..b] == piece) {
            // This fragment is already named so use that name
            Some(i) => {
                main_routine.push(i);
                lo = hi;
            }
            
            // This fragment is not yet named
            None => {
                if render_instructions(piece).len() > SUBLEN {
                    // The fragment is too long to be named
                    // All future fragments would be too; trigger rollback
                    hi = usize::MAX;
                } else if subroutines.len() < SUBNAMES.len() {
                    // Allocate a new named fragment
                    let s = subroutines.len();
                    subroutines.push((lo,hi));
                    main_routine.push(s);
                    lo = hi;
                }
            }
        }
    }

    Some((main_routine,
    subroutines.into_iter().map(|(a,b)| &target[a..b]).collect()))
}
