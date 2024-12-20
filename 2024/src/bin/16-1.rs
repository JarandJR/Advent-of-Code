use std::{collections::HashMap, i32};

use common::{datatypes::vec2::Vec2, io::parse_into_lines_automatic};

fn main() {
    dbg!(parse_and_solve(parse_into_lines_automatic("16")));
}

fn parse_and_solve(line_iter: impl Iterator<Item = String>) -> usize {
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
    *distance.get(&end).unwrap_or(&0) as usize
}

#[test]
fn day16_1a() {
    let input = "###############
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
###############";
    let result = parse_and_solve(input.lines().map(|s| s.to_owned()));
    assert_eq!(result, 7036);
}

#[test]
fn day16_1b() {
    let input = "#################
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
#################";
    let result = parse_and_solve(input.lines().map(|s| s.to_owned()));
    assert_eq!(result, 11048);
}
