use chrono::{Datelike, Local};
use std::{
    fs::File,
    io::{BufRead, BufReader},
};

pub const NEW_LINE: u8 = b'\n';

pub fn parse_into_byte_lines_automatic(day: &str) -> Option<impl Iterator<Item = Vec<u8>>> {
    let today = Local::now();
    assert!(
        today.month() == 12,
        "This function can only be used in desember"
    );
    parse_into_byte_lines(today.year(), day)
}

pub fn parse_into_byte_lines(year: i32, day: &str) -> Option<impl Iterator<Item = Vec<u8>>> {
    let curnt_dir = std::env::current_dir()
        .expect("Could not get current direction")
        .display()
        .to_string();
    let path = if curnt_dir.contains(year.to_string().as_str()) {
        String::new()
    } else {
        format!("{}/", year)
    };
    if let Ok(file) = File::open(format!("{}inputs/{}.txt", path, day)) {
        let reader = BufReader::new(file);
        return Some(reader.split(NEW_LINE).flat_map(|line| line.ok()));
    }
    None
}

pub fn parse_into_lines_automatic(day: &str) -> Option<impl Iterator<Item = String>> {
    let today = Local::now();
    assert!(
        today.month() == 12,
        "This function can only be used in desember"
    );
    parse_into_lines(today.year(), day)
}

pub fn parse_into_lines(year: i32, day: &str) -> Option<impl Iterator<Item = String>> {
    use std::io::BufRead;
    let curnt_dir = std::env::current_dir()
        .expect("Could not get current direction")
        .display()
        .to_string();
    let path = if curnt_dir.contains(year.to_string().as_str()) {
        String::new()
    } else {
        format!("{}/", year)
    };
    if let Ok(file) = std::fs::File::open(format!("{}inputs/{}.txt", path, day)) {
        let reader = std::io::BufReader::new(file);
        return Some(reader.lines().flat_map(|line| line.ok()));
    }
    None
}
