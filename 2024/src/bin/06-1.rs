use aoc2024::{parse_into_lines, Vec2};
use itertools::Itertools;

fn main() {
    dbg!(parse_and_solve("06"));
}

fn parse_and_solve(day: &str) -> usize {
    if let Some(mut line_iter) = parse_into_lines(day) {
        let grid = line_iter.join("\n");
        let start = Vec2::from(grid.lines().enumerate().fold((0,0), |start: (usize, usize), (y, line)| {
            let potential_start = line.chars().enumerate().fold((0, 0), |start, (x, c)| {
                print!("{}", c);
                match c == '^' {
                    true => (x, y),
                    _ => start
                }
            });
            println!();
            if potential_start != (0, 0) {
                return potential_start;
            }
            start
        }));
        println!("start: {:?}", start);
        return 1;
    }
    0
}

#[test]
fn day06_1() {
    use std::fs::{remove_file, File};
    let file_name = "test_06_1";
    include_str!("../../inputs/06.txt");
    let file_path = format!("inputs/{}.txt", file_name);
    {
        // Setup for test
        use std::io::Write;
        let mut file = File::create(&file_path).expect("Could not create file");
        writeln!(
            file,
            "....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#..."
        )
        .expect("Could not write to file");
    }
    let result = parse_and_solve(&file_name);
    // Clean up
    remove_file(file_path).expect("Could not remove file");
    assert_eq!(result, 41);
}
