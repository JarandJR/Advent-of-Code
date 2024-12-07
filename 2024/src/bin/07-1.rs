use aoc2024::parse_into_lines_automatic;
use itertools::Itertools;

fn main() {
    dbg!(parse_and_solve("07"));
}

fn parse_and_solve(day: &str) -> usize {
    if let Some(mut line_iter) = parse_into_lines_automatic(day) {
        return line_iter.fold(0, |acc, line| {
            let mut split = line.split(':');
            let result = split.next().unwrap().trim().parse::<u32>().unwrap();
            let numbers = split
                .next()
                .unwrap()
                .split_ascii_whitespace()
                .filter_map(|num| num.parse::<u32>().ok())
                .collect::<Vec<_>>();
            println!("{}, {:?}", result, numbers);
            acc
        });
    }
    0
}

#[test]
fn day07_1() {
    use std::fs::{remove_file, File};
    let file_name = "test_07_1";
    let file_path = format!("inputs/{}.txt", file_name);
    {
        // Setup for test
        use std::io::Write;
        let mut file = File::create(&file_path).expect("Could not create file");
        writeln!(
            file,
            "190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20"
        )
        .expect("Could not write to file");
    }
    let result = parse_and_solve(&file_name);
    // Clean up
    remove_file(file_path).expect("Could not remove file");
    assert_eq!(result, 3749);
}
