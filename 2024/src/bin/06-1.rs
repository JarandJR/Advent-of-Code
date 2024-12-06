use std::collections::HashSet;

use aoc2024::{parse_into_lines, Grid, Vec2};
use itertools::Itertools;

fn main() {
    dbg!(parse_and_solve("06"));
}

fn parse_and_solve(day: &str) -> usize {
    if let Some(mut line_iter) = parse_into_lines(day) {
        let input = line_iter.join("\n");
        let mut at = Vec2::from(input.lines().enumerate().fold(
            (0, 0),
            |start: (usize, usize), (y, line)| {
                let potential_start =
                    line.chars()
                        .enumerate()
                        .fold((0, 0), |start, (x, c)| match c == '^' {
                            true => (x, y),
                            _ => start,
                        });
                if potential_start != (0, 0) {
                    return potential_start;
                }
                start
            },
        ));
        let grid = Grid::from(input.lines());
        let mut direction_iter = Vec2::FOUR_CONNECTEDNESS.iter().cycle();
        let mut curnt_dir = direction_iter.next().unwrap();
        let mut memory = HashSet::new();
        while at.x < grid.columns as i32 - 1 && 0 < at.x && 0 < at.y && at.y < grid.rows as i32 - 1
        {
            let mut movement_iter = grid.slice_end_iter(&at, curnt_dir);
            let _ = movement_iter.next();
            for mov in movement_iter {
                if mov == '#' {
                    curnt_dir = direction_iter.next().unwrap();
                    break;
                }
                at += *curnt_dir;
                if !memory.contains(&at) {
                    memory.insert(at);
                }
            }
        }
        return memory.len();
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
