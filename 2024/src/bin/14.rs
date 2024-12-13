use common::io::parse_into_lines_automatic;

fn main() {
    dbg!(parse_and_solve_part_1("14"));
    dbg!(parse_and_solve_part_2("14"));
}

fn parse_and_solve_part_1(day: &str) -> usize {
    if let Some(line_iter) = parse_into_lines_automatic(day) {
        return 0;
    }
    0
}

fn parse_and_solve_part_2(day: &str) -> usize {
    if let Some(line_iter) = parse_into_lines_automatic(day) {
        return 0;
    }
    0
}

#[test]
fn day14_1() {
    use std::fs::{remove_file, File};
    let file_name = "test_14_1";
    let file_path = format!("inputs/{}.txt", file_name);
    {
        // Setup for test
        use std::io::Write;
        let mut file = File::create(&file_path).expect("Could not create file");
        writeln!(file, "INPUT").expect("Could not write to file");
    }
    let result = parse_and_solve_part_1(&file_name);
    // Clean up
    remove_file(file_path).expect("Could not remove file");
    assert_eq!(result, 0);
}

#[test]
fn day14_2() {
    use std::fs::{remove_file, File};
    let file_name = "test_14_2";
    let file_path = format!("inputs/{}.txt", file_name);
    {
        // Setup for test
        use std::io::Write;
        let mut file = File::create(&file_path).expect("Could not create file");
        writeln!(file, "INPUT").expect("Could not write to file");
    }
    let result = parse_and_solve_part_2(&file_name);
    // Clean up
    remove_file(file_path).expect("Could not remove file");
    assert_eq!(result, 0);
}
