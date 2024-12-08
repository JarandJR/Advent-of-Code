use std::collections::HashMap;

use common::{
    datastructs::{double_ended_map::DoubleEndedMap, vec2::Vec2},
    io::parse_into_lines_automatic,
};

fn main() {
    dbg!(parse_and_solve("8"));
}

fn parse_and_solve(day: &str) -> usize {
    if let Some(line_iter) = parse_into_lines_automatic(day) {
        let mut frequencies = HashMap::new(); //DoubleEndedMap::new()
        line_iter.enumerate().for_each(|(row, line)| {
            line.chars().enumerate().for_each(|(column, c)| {
                if c != '.' {
                    frequencies
                        .entry(c)
                        .or_insert(Vec::new())
                        .push(Vec2::from_row_column(row, column));
                }
            });
        });
        frequencies.iter().for_each(|(frequency, positions)| {
            print!("{frequency}: ");
            print!("{:?}", positions);
            println!("\n");
        });
        return 1;
    }
    0
}

#[test]
fn day8_1() {
    use std::fs::{remove_file, File};
    let file_name = "test_8_1";
    let file_path = format!("inputs/{}.txt", file_name);
    {
        // Setup for test
        use std::io::Write;
        let mut file = File::create(&file_path).expect("Could not create file");
        writeln!(
            file,
            "............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............"
        )
        .expect("Could not write to file");
    }
    let result = parse_and_solve(&file_name);
    // Clean up
    remove_file(file_path).expect("Could not remove file");
    assert_eq!(result, 14);
}
