use std::{
    collections::{HashMap, HashSet},
    i32,
};

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

    let mut stack = vec![(start, Vec2::EAST, 0, HashSet::new())];
    let mut distance = HashMap::new();
    let mut paths = HashMap::new();
    while let Some((pos, dir, d, mut prevs)) = stack.pop() {
        let at = grid[pos.row()][pos.column()];
        if at == '#' {
            continue;
        }
        let dist = distance.entry(pos).or_insert(i32::MAX);
        if d <= *dist {
            *dist = d;
        } else {
            if *distance.get(&(pos + dir)).unwrap_or(&0) != d + 1 {
                continue;
            }
        }
        prevs.insert(pos);
        if at == 'E' {
            paths.entry(d).or_insert(HashSet::new()).extend(prevs);
            continue;
        }
        let next = pos + dir;
        stack.extend([
            (next, dir, d + 1, prevs.clone()),
            (
                pos + dir.rotate_90_clockwise(),
                dir.rotate_90_clockwise(),
                d + 1001,
                prevs.clone(),
            ),
            (
                pos + dir.rotate_90_counter_clockwise(),
                dir.rotate_90_counter_clockwise(),
                d + 1001,
                prevs,
            ),
        ]);
    }
    paths.get(&distance.get(&end).unwrap()).unwrap().len()
}

#[test]
fn day16_2a() {
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
    assert_eq!(result, 45);
}

#[test]
fn day16_2b() {
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
    assert_eq!(result, 64);
}
