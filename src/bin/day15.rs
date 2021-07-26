use advent::intcode::{Machine, Step, parse_program};
use advent::bfs::Bfs;
use advent::pos::{Pos, Dir};

fn main() {
    let (p1,p2) = solve();
    println!("Part 1: {}", p1);
    println!("Part 2: {}", p2);
}

fn solve() -> (u64, u64) {
    let input = advent::load_input_file(15);
    let pgm = parse_program(&input).unwrap();
    let m = Machine::new(pgm);

    let s1 = robot_bfs(m)
        .find(|s| s.found)
        .expect("oxygen not found");
    let s2 = robot_bfs(s1.machine).last().unwrap();


    (s1.distance, s2.distance)
}

#[derive(Clone)]
struct MazeState {
    pos: Pos,
    found: bool,
    distance: u64,
    machine: Machine,
}

fn robot_bfs(machine: Machine) -> impl Iterator<Item=MazeState> {
    fn transition(s: &MazeState) -> Vec<MazeState> {
        Dir::ELEMS.iter().flat_map(
            |&dir| {
                let mut machine = s.machine.clone();
                let found =
                    match move_robot(&mut machine, dir) {
                        Response::Stuck => return None,
                        Response::Moved => false,
                        Response::Found => true,
                    };
                Some(
                    MazeState {
                        pos: s.pos.advance(dir, 1),
                        distance: s.distance + 1,
                        found,
                        machine,
                    })
            }
        ).collect()
    }

    Bfs::new(
        MazeState {
            pos: Pos::ORIGIN,
            found: false,
            distance: 0,
            machine,
        },
        transition,
        |s| s.pos)
}

fn move_robot(machine: &mut Machine, dir: Dir) -> Response {
    match machine.step() {
        Ok(Step::Input(i)) => machine[i] = dir_to_i64(dir),
        _ => panic!("bad machine state"),
    }

    match machine.step() {
        Ok(Step::Output(0)) => Response::Stuck,
        Ok(Step::Output(1)) => Response::Moved,
        Ok(Step::Output(2)) => Response::Found,
        _ => panic!("bad machine state"),
    }
}

#[derive(Debug, Eq, Ord, PartialOrd, PartialEq, Copy, Clone)]
enum Response {
    Moved,
    Stuck,
    Found,
}


fn dir_to_i64(dir: Dir) -> i64 {
    match dir {
        Dir::N => 1,
        Dir::S => 2,
        Dir::W => 3,
        Dir::E => 4,
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn solve_input() {
        let (p1, p2) = solve();
        assert_eq!(p1, 242);
        assert_eq!(p2, 276);
    }
}