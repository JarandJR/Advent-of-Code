use std::{
    fs::File,
    io::{BufRead, BufReader},
};

pub const NEW_LINE: u8 = b'\n';
pub const DIRECTIONS: [(i32, i32); 8] = [
    // North
    (-1, 0),
    // South
    (1, 0),
    // West
    (0, -1),
    // East
    (0, 1),
    // North-Vest
    (-1, -1),
    // North-East
    (-1, 1),
    // South-Vest
    (1, -1),
    // South-East
    (1, 1),
];

pub fn parse_into_byte_lines(day: &str) -> Option<impl Iterator<Item = Vec<u8>>> {
    if let Ok(file) = File::open(format!("2024/inputs/{}.txt", day)) {
        let reader = BufReader::new(file);
        return Some(reader.split(NEW_LINE).flat_map(|line| line.ok()));
    }
    None
}

pub fn parse_into_lines(day: &str) -> Option<impl Iterator<Item = String>> {
    use std::io::BufRead;
    if let Ok(file) = std::fs::File::open(format!("2024/inputs/{}.txt", day)) {
        let reader = std::io::BufReader::new(file);
        return Some(reader.lines().flat_map(|line| line.ok()));
    }
    None
}
