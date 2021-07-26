use advent::intcode::{parse_program, Machine, Step};
use std::collections::VecDeque;

const NETSIZE: usize = 50;
const NATADDR: i64 = 255;
const NOPACKET: i64 = -1;

fn main() {
    let input = advent::load_input_file(23);
    let pgm = parse_program(&input).unwrap();
    let m = Machine::new(pgm);

    println!("Part 1: {}", part1(&m));
    println!("Part 2: {}", part2(&m));
}

fn make_network(m: &Machine) -> Vec<Machine> {
    let mut network = vec![m.clone(); NETSIZE];

    for (host_id, host) in network.iter_mut().enumerate() {
        let i = host.step().unwrap().input().unwrap();
        host[i] = host_id as i64;
    }

    network
}

fn part1(machine: &Machine) -> i64 {
    let mut network = make_network(&machine);
    let mut packets = vec![VecDeque::new(); network.len()];

    loop {
        for (host_id, host) in network.iter_mut().enumerate() {
            match host.step().unwrap() {
                Step::Halt => panic!("host halted"),
                Step::Input(i) => match packets[host_id].pop_front() {
                    None => host[i] = NOPACKET,
                    Some((x, y)) => {
                        host[i] = x;
                        let j = host.step().unwrap().input().unwrap();
                        host[j] = y
                    }
                },
                Step::Output(d) => {
                    let x = host.step().unwrap().output().unwrap();
                    let y = host.step().unwrap().output().unwrap();
                    if d == NATADDR {
                        return y;
                    } else {
                        packets[d as usize].push_back((x, y));
                    }
                }
            }
        }
    }
}

#[derive(Clone, Debug)]
struct Packet {
    x: i64,
    y: i64,
}

fn part2(m: &Machine) -> i64 {
    let mut network = make_network(&m);
    let mut packets: Vec<VecDeque<Packet>> = vec![VecDeque::new(); network.len()];

    let mut stalls = 0;
    let mut previous = None;
    let mut restart = Packet { x: 0, y: 0 };

    loop {
        stalls += 1;
        for (host_id, host) in network.iter_mut().enumerate() {
            match host.step().unwrap() {
                Step::Halt => panic!("host halted"),
                Step::Input(i) => match packets[host_id].pop_front() {
                    None => host[i] = -1,
                    Some(packet) => {
                        stalls = 0;
                        host[i] = packet.x;
                        let i = host.step().unwrap().input().unwrap();
                        host[i] = packet.y
                    }
                },
                Step::Output(destination) => {
                    stalls = 0;
                    let x = host.step().unwrap().output().unwrap();
                    let y = host.step().unwrap().output().unwrap();
                    let packet = Packet { x, y };
                    if destination == 255 {
                        restart = packet
                    } else {
                        packets[destination as usize].push_back(packet);
                    }
                }
            }
        }

        if stalls > 1 {
            if Some(restart.y) == previous {
                return restart.y;
            }
            previous = Some(restart.y);

            let host = &mut network[0];
            let i = host.step().unwrap().input().unwrap();
            host[i] = restart.x;
            let i = host.step().unwrap().input().unwrap();
            host[i] = restart.y;
        }
    }
}
