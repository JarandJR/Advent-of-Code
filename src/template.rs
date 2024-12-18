use common::io::parse_into_lines_automatic;

fn main() {
    dbg!(parse_and_solve("DAY"));
}

fn parse_and_solve(day: &str) -> usize {
    if let Some(line_iter) = parse_into_lines_automatic(day) {
        return 0;
    }
    panic!("Failed to read input file")
}

#[test]
fn dayDAY_1() {
    use std::fs::{remove_file, File};
    let file_name = "test_DAY_1";
    let file_path = format!("inputs/{}.txt", file_name);
    {
        // Setup for test
        use std::io::Write;
        let mut file = File::create(&file_path).expect("Could not create file");
        writeln!(
            file,
            "INPUT"
        )
        .expect("Could not write to file");
    }
    let result = parse_and_solve(&file_name);
    // Clean up
    remove_file(file_path).expect("Could not remove file");
    assert_eq!(result, 1);
}
