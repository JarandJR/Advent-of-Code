use std::collections::{HashMap, HashSet};

use common::generic_algorithms::dfs::dfs;
use common::{datastructs::grid::Grid, datatypes::vec2::Vec2, io::parse_into_lines_automatic};

fn main() {
    dbg!(parse_and_solve_1(parse_into_lines_automatic("7")));
    dbg!(parse_and_solve_2(parse_into_lines_automatic("7")));
}

fn parse_and_solve_1(line_iter: impl Iterator<Item = String>) -> usize {
    let mut stack = Vec::new();
    let grid = line_iter
        .enumerate()
        .map(|(row, line)| {
            line.chars()
                .enumerate()
                .map(|(col, c)| {
                    c.eq(&'S')
                        .then(|| stack.push(Vec2::from_row_column(row, col)));
                    c
                })
                .collect::<Vec<char>>()
        })
        .collect::<Grid<char>>();
    let mut nodes = 0;
    let mut visited = HashSet::new();
    while let Some(point) = stack.pop() {
        if visited.contains(&point) {
            continue;
        }
        visited.insert(point);
        let next = if &grid[point] == &'^' {
            nodes += 1;
            vec![point + Vec2::EAST, point + Vec2::WEST]
        } else {
            vec![point + Vec2::SOUTH]
        };
        next.into_iter()
            .filter(|n| grid.contains_point(n))
            .for_each(|n| stack.push(n));
    }
    nodes
}

fn parse_and_solve_2(line_iter: impl Iterator<Item = String>) -> usize {
    let mut start = Vec2::new(0, 0);
    let grid = line_iter
        .enumerate()
        .map(|(row, line)| {
            line.chars()
                .enumerate()
                .map(|(col, c)| {
                    c.eq(&'S').then(|| start = Vec2::from_row_column(row, col));
                    c
                })
                .collect::<Vec<char>>()
        })
        .collect::<Grid<char>>();
    let mut mem = HashMap::new();
    dfs(
        start,
        &mut mem,
        &|state| (!grid.contains_point(state)).then(|| 1),
        &|state| {
            (grid[*state] == '^')
                .then(|| vec![*state + Vec2::WEST, *state + Vec2::EAST])
                .unwrap_or_else(|| vec![*state + Vec2::SOUTH])
        },
        &|a, b| a + b,
    )
}

#[test]
fn day7_1() {
    let input = ".......S.......
...............
.......^.......
...............
......^.^......
...............
.....^.^.^.....
...............
....^.^...^....
...............
...^.^...^.^...
...............
..^...^.....^..
...............
.^.^.^.^.^...^.
...............";
    let result = parse_and_solve_1(input.lines().map(|s| s.to_owned()));
    assert_eq!(result, 21);
}

#[test]
fn day7_2() {
    let input = ".......S.......
...............
.......^.......
...............
......^.^......
...............
.....^.^.^.....
...............
....^.^...^....
...............
...^.^...^.^...
...............
..^...^.....^..
...............
.^.^.^.^.^...^.
...............";
    let result = parse_and_solve_2(input.lines().map(|s| s.to_owned()));
    assert_eq!(result, 40);
}
