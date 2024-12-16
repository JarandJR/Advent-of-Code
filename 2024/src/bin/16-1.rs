use std::{collections::HashMap, i32};

use common::{datastructs::vec2::Vec2, io::parse_into_lines_automatic};

fn main() {
    dbg!(parse_and_solve("16"));
}

fn parse_and_solve(day: &str) -> usize {
    if let Some(line_iter) = parse_into_lines_automatic(day) {
        let mut start = Vec2::default();
        let mut end = Vec2::default();
        let mut rows = 0;
        let mut columns = 0;
        let grid = line_iter
            .enumerate()
            .map(|(row, line)| {
                columns = line.len();
                rows += 1;
                line.chars()
                    .enumerate()
                    .map(|(column, c)| {
                        if c == 'S' {
                            start = Vec2::from_row_column(row, column);
                        }
                        if c == 'E' {
                            end = Vec2::from_row_column(row, column);
                        }
                        c
                    })
                    .collect::<Vec<char>>()
            })
            .collect::<Vec<Vec<char>>>();

        let mut stack = Vec::new();
        let mut distance = HashMap::new();
        stack.push((start, Vec2::EAST, 0));
        while let Some((pos, dir, d)) = stack.pop() {
            let at = grid[pos.row()][pos.column()];
            if at == '#' {
                continue;
            }
            let dist = distance.entry(pos).or_insert(i32::MAX);
            if d < *dist {
                *dist = d;
            } else {
                continue;
            }
            if at == 'E' {
                continue;
            }
            let next = pos + dir;
            stack.extend([
                (next, dir, *dist + 1),
                (
                    pos + dir.rotate_90_clockwise(),
                    dir.rotate_90_clockwise(),
                    *dist + 1001,
                ),
                (
                    pos + dir.rotate_90_counter_clockwise(),
                    dir.rotate_90_counter_clockwise(),
                    *dist + 1001,
                ),
            ]);
        }
        return *distance.get(&end).unwrap_or(&0) as usize;
    }
    0
}

#[test]
fn day16_1a() {
    use std::fs::{remove_file, File};
    let file_name = "test_16_1a";
    let file_path = format!("inputs/{}.txt", file_name);
    {
        // Setup for test
        use std::io::Write;
        let mut file = File::create(&file_path).expect("Could not create file");
        writeln!(
            file,
            "###############
#.......#....E#
#.#.###.#.###.#
#.....#.#...#.#
#.###.#####.#.#
#.#.#.......#.#
#.#.#####.###.#
#...........#.#
###.#.#####.#.#
#...#.....#.#.#
#.#.#.###.#.#.#
#.....#...#.#.#
#.###.#.#.#.#.#
#S..#.....#...#
###############"
        )
        .expect("Could not write to file");
    }
    let result = parse_and_solve(&file_name);
    // Clean up
    remove_file(file_path).expect("Could not remove file");
    assert_eq!(result, 7036);
}

#[test]
fn day16_1b() {
    use std::fs::{remove_file, File};
    let file_name = "test_16_1b";
    let file_path = format!("inputs/{}.txt", file_name);
    {
        // Setup for test
        use std::io::Write;
        let mut file = File::create(&file_path).expect("Could not create file");
        writeln!(
            file,
            "#################
#...#...#...#..E#
#.#.#.#.#.#.#.#.#
#.#.#.#...#...#.#
#.#.#.#.###.#.#.#
#...#.#.#.....#.#
#.#.#.#.#.#####.#
#.#...#.#.#.....#
#.#.#####.#.###.#
#.#.#.......#...#
#.#.###.#####.###
#.#.#...#.....#.#
#.#.#.#####.###.#
#.#.#.........#.#
#.#.#.#########.#
#S#.............#
#################"
        )
        .expect("Could not write to file");
    }
    let result = parse_and_solve(&file_name);
    // Clean up
    remove_file(file_path).expect("Could not remove file");
    assert_eq!(result, 11048);
}
