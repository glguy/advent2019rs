use std::collections::HashMap;

fn main() {
    let input = advent::load_input_file(6);

    let mut forward: HashMap<&str, Vec<&str>> = HashMap::new();
    let mut backward: HashMap<&str, &str> = HashMap::new();

    for line in input.lines() {
        let parts: Vec<&str> = line.split(')').collect();
        let k1 = parts[0];
        let k2 = parts[1];
        backward.insert(k2, k1);
        if !forward.contains_key(k1) { forward.insert(k1, vec![]); }
        forward.get_mut(k1).unwrap().push(k2);
    }

    let mut horizon = vec!["COM"];

    let mut p1 = 0;
    let mut distance = 0;
    let z = vec![];
    while !horizon.is_empty() {
        p1 += horizon.len() * distance;
        horizon = horizon.into_iter().flat_map(|x|
            forward.get(x).unwrap_or(&z)).copied().collect();
        distance += 1;
    }

    println!("Part 1: {}", p1);

    let mut p1 = path(&backward, "SAN");
    let mut p2 = path(&backward, "YOU");

    while p1.last() == p2.last() {
        p1.pop();
        p2.pop();
    }

    let part2 = p1.len() + p2.len() - 2;
    println!("Part 2: {}", part2);
}

fn path<'a>(parents: &'a HashMap<&'a str, &'a str>, start: &'a str) -> Vec<&'a str> {
    std::iter::successors(Some(start), |&k| parents.get(&k).copied()).collect()
}
