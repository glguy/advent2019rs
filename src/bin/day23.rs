use advent::intcode::{parse_program, Machine, Step};
use std::collections::VecDeque;

const NETSIZE: usize = 50;
const NATADDR: i64 = 255;
const NOPACKET: i64 = -1;

fn main() {
    let input = advent::load_input_file(23);
    let pgm = parse_program(&input).unwrap();
    let m = Machine::new(pgm);

    println!("Part 1: {}", part1(m.clone()).unwrap());
    println!("Part 2: {}", part2(m));
}

#[derive(Copy, Clone, Debug)]
struct Packet {
    x: i64,
    y: i64,
}

fn part1(machine: Machine) -> Option<i64> {
    let mut work: VecDeque<(i64, Packet)> = Default::default();
    let mut network = make_network(machine);

    for host in &mut network {
        deliver(host, [], &mut work)
    }

    while let Some((d, p)) = work.pop_front() {
        if d == NATADDR {
            return Some(p.y)
        } else {
            deliver(&mut network[d as usize], [p], &mut work)
        }
    }
    None
}


fn part2(machine: Machine) -> i64 {
    let mut work: VecDeque<(i64, Packet)> = Default::default();
    let mut network = make_network(machine);

    for host in &mut network {
        deliver(host, [], &mut work)
    }

    let mut prev = None;
    let mut restart = Packet { x: 0, y: 0 };

    loop {
        while let Some((d, p)) = work.pop_front() {
            if d == NATADDR {
                restart = p
            } else {
                deliver(&mut network[d as usize], [p], &mut work)
            }
        }

        if Some(restart.y) == prev {
            return restart.y;
        }
        prev = Some(restart.y);
        deliver(&mut network[0], [restart], &mut work);
    }
}

fn make_network(mut machine: Machine) -> Vec<Machine> {
    
    let i = machine.step().unwrap().input().unwrap();
    let mut network = vec![machine; NETSIZE];

    for (host_id, host) in network.iter_mut().enumerate() {
        host[i] = host_id as i64;
    }

    network
}

fn deliver<I: IntoIterator<Item = Packet>>(
    machine: &mut Machine,
    inputs: I,
    outputs: &mut VecDeque<(i64, Packet)>,
) {
    let mut iter = inputs.into_iter();
    let mut stalled = false;
    loop {
        match machine.step().unwrap() {
            Step::Halt => panic!("halt"),
            Step::Output(d) => {
                outputs.push_back((
                    d,
                    Packet {
                        x: machine.step().unwrap().output().unwrap(),
                        y: machine.step().unwrap().output().unwrap(),
                    },
                ));
                stalled = false;
            }
            Step::Input(i) => match iter.next() {
                Some(packet) => {
                    machine[i] = packet.x;
                    let i = machine.step().unwrap().input().unwrap();
                    machine[i] = packet.y;
                    stalled = false;
                }
                None => {
                    machine[i] = NOPACKET;
                    if stalled { return } else { stalled = true }
                }
            },
        }
    }
}
