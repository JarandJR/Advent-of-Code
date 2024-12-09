use chrono::{Datelike, Local};
use std::fs::File;
use std::io::Write;

fn main() {
    let today = Local::now();
    let year = today.year().to_string();
    let day = today.day().to_string();

    // Input file
    let input_file_path = format!("{}/inputs/{}.txt", year, day);
    let file = File::create(&input_file_path).expect("Could not create file");

    // Solution file
    let solution_file_path = format!("{}/src/bin/{}.rs", year, day);
    let mut file = File::create(&solution_file_path).expect("Could not create file");
    let template = include_str!("template.rs");
    let template = template.replace("DAY", &day);
    file.write_all(template.as_bytes())
        .expect("Could not write to file");
}
