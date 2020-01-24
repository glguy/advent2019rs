pub mod bfs;
pub mod pos;
pub mod intcode;
use std::fs;

pub fn load_input_file(day: u64) -> String {
    let path = format!("/Users/emertens/Source/advent2019/inputs/input{:02}.txt", day);
    fs::read_to_string(path).unwrap()
}