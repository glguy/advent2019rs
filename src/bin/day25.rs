use advent::intcode::iterator::machine;
use advent::intcode::parse_program;
use std::iter::once;

fn main() {
    let input = advent::load_input_file(25);
    let pgm = parse_program(&input).unwrap();
    let base_script = [
        "north",
        "take sand",
        "north",
        "take space heater",
        "east",
        "take semiconductor",
        "west",
        "south",
        "south",
        "east",
        "take ornament",
        "south",
        "take festive hat",
        "east",
        "take asterisk",
        "south",
        "west",
        "take food ration",
        "east",
        "east",
        "take cake",
        "west",
        "north",
        "west",
        "north",
        "west",
        "west",
        "north",
        "north",
    ];

    let items = [
        "asterisk",
        "ornament",
        "cake",
        "space heater",
        "festive hat",
        "semiconductor",
        "food ration",
        "sand",
    ];

    let script: String = base_script
        .iter()
        .map(|x| x.to_string())
        .chain(go(&items, "drop", "take"))
        .collect::<Vec<String>>()
        .join("\n");

    let mut s = String::new();
    for c in machine(pgm, script.chars().map(|x| x as i64)).map(|x| x as u8 as char) {
        if c == '\n' {
            println!("{}", s);
            s.clear();
        } else {
            s.push(c)
        }
    }
}

fn go(items: &[&str], act1: &str, act2: &str) -> Box<dyn DoubleEndedIterator<Item = String>> {
    if items.is_empty() {
        Box::new(once("west".to_string()))
    } else {
        Box::new(
            go(&items[1..], act1, act2)
                .chain(once(format!("{} {}", act1, items[0])))
                .chain(go(&items[1..], act2, act1).rev()),
        )
    }
}
