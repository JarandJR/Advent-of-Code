use std::collections::HashSet;

use common::{datastructs::string_grid::StrGrid, datatypes::vec2::Vec2, io::parse_into_lines};
use itertools::Itertools;

fn main() {
    dbg!(parse_and_solve(2024, "06"));
}

fn parse_and_solve(year: i32, day: &str) -> usize {
    if let Some(mut line_iter) = parse_into_lines(year, day) {
        let input = line_iter.join("\n");
        let start = Vec2::from(input.lines().enumerate().fold(
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
        let mut at = start.clone();
        let grid = input.lines().collect::<StrGrid>();
        let mut direction_iter = Vec2::FOUR_CONNECTEDNESS.iter().cycle().peekable();
        let mut curnt_dir = direction_iter.next().unwrap();
        let mut memory = HashSet::new();
        loop {
            let movement_iter = grid.slice_end_iter(&(at + *curnt_dir), curnt_dir);
            for mov in movement_iter {
                memory.insert(at);
                if mov == '#' {
                    curnt_dir = direction_iter.next().unwrap();
                    break;
                }
                at += *curnt_dir;
            }
            // The last position
            memory.insert(at);
            if !(0..grid.columns).contains(&((at.x + curnt_dir.x) as usize))
                || !(0..grid.rows).contains(&((at.y + curnt_dir.y) as usize))
            {
                break;
            }
        }
        let mem = memory.clone();
        let mut loop_counter = 0;
        for visited in mem.iter() {
            let mut direction_iter = Vec2::FOUR_CONNECTEDNESS.iter().cycle();
            let mut curnt_dir = direction_iter.next().unwrap();
            let mut memory = HashSet::new();
            at = start.clone();
            loop {
                if !(0..grid.columns - 1).contains(&((at.x + curnt_dir.x) as usize))
                    || !(0..grid.rows - 1).contains(&((at.y + curnt_dir.y) as usize))
                {
                    break;
                }
                for mov in grid.slice_end_iter(&(at + *curnt_dir), curnt_dir) {
                    memory.insert((at, curnt_dir));
                    if mov == '#' || at + *curnt_dir == *visited {
                        curnt_dir = direction_iter.next().unwrap();
                        break;
                    }
                    at += *curnt_dir;
                    if memory.contains(&(at, curnt_dir)) {
                        loop_counter += 1;
                        // Found, break out of both loops
                        at = Vec2::new(0, 0);
                        break;
                    }
                }
            }
        }
        return loop_counter;
    }
    0
}

#[test]
fn day06_2() {
    use std::fs::{remove_file, File};
    let file_name = "test_06_2";
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
    let result = parse_and_solve(2024, &file_name);
    // Clean up
    remove_file(file_path).expect("Could not remove file");
    assert_eq!(result, 6);
}
