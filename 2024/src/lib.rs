use std::{
    fs::File,
    io::{BufRead, BufReader},
};

const NEW_LINE: u8 = b'\n';

pub fn parse_into_byte_lines(day: &str) -> Option<impl Iterator<Item = Vec<u8>>> {
    if let Ok(file) = File::open(format!("inputs/{}.txt", day)) {
        let reader = BufReader::new(file);
        return Some(
            reader
                .split(NEW_LINE)
                .flat_map(|line| line.ok().into_iter()),
        );
    }
    None
}
