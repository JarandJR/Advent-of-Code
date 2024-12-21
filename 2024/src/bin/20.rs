use std::{collections::HashMap, i32};

use common::{datastructs::grid::Grid, datatypes::vec2::Vec2, io::parse_into_lines_automatic};
use itertools::Itertools;

fn main() {
    dbg!(parse_and_solve_part_1(
        parse_into_lines_automatic("20"),
        100
    ));
    dbg!(parse_and_solve_part_2(
        parse_into_lines_automatic("20"),
        100,
        20
    ));
}

fn parse_and_solve_part_1(line_iter: impl Iterator<Item = String>, treshold: usize) -> usize {
    let mut start = Vec2::default();
    let mut end = Vec2::default();
    let grid = line_iter
        .enumerate()
        .map(|(row, line)| {
            line.chars()
                .enumerate()
                .map(|(column, chr)| {
                    if chr == 'S' {
                        start = Vec2::from_row_column(row, column);
                    } else if chr == 'E' {
                        end = Vec2::from_row_column(row, column);
                    }
                    chr
                })
                .collect::<Vec<char>>()
        })
        .collect::<Vec<Vec<char>>>();

    let grid = Grid::from_grid(grid);
    let mut distance = HashMap::new();
    let mut cheats = HashMap::new();
    let mut stack = vec![(start, 0)];
    while let Some((at, d)) = stack.pop() {
        let entry_dist = distance.entry(at).or_insert(i32::MAX);
        if d < *entry_dist {
            *entry_dist = d;
        } else {
            continue;
        }
        if at == end {
            break;
        }
        Vec2::FOUR_CONNECTEDNESS
            .into_iter()
            .map(|d| (d + at, d))
            .filter(|(p, _d)| {
                (0..grid.rows as i32).contains(&p.y) && (0..grid.columns as i32).contains(&p.x)
            })
            .for_each(|(p, dir)| {
                let value_at = grid[p];
                match value_at {
                    '#' => {
                        let next = p + dir;
                        if (0..grid.rows as i32).contains(&next.y)
                            && (0..grid.columns as i32).contains(&next.x)
                        {
                            if grid[next] != '#' && !distance.contains_key(&next) {
                                cheats.insert((p, next), d + 2);
                            }
                        }
                    }
                    _ => {
                        stack.push((p, d + 1));
                    }
                }
            });
    }
    cheats
        .drain()
        .filter(|((_p1, p2), dist)| {
            let saved_seconds = distance.get(p2).unwrap_or(&0) - dist;
            saved_seconds >= treshold as i32
        })
        .count()
}

fn parse_and_solve_part_2(
    line_iter: impl Iterator<Item = String>,
    treshold: usize,
    cheat_secs: i32,
) -> usize {
    let mut start = Vec2::default();
    let mut end = Vec2::default();
    let grid = line_iter
        .enumerate()
        .map(|(row, line)| {
            line.chars()
                .enumerate()
                .map(|(column, chr)| {
                    if chr == 'S' {
                        start = Vec2::from_row_column(row, column);
                    } else if chr == 'E' {
                        end = Vec2::from_row_column(row, column);
                    }
                    chr
                })
                .collect::<Vec<char>>()
        })
        .collect::<Vec<Vec<char>>>();

    let grid = Grid::from_grid(grid);
    let mut distance = HashMap::new();
    let mut stack = vec![(start, 0)];
    while let Some((at, d)) = stack.pop() {
        let entry_dist = distance.entry(at).or_insert(i32::MAX);
        if d < *entry_dist {
            *entry_dist = d;
        } else {
            continue;
        }
        if at == end {
            break;
        }
        stack.extend(
            grid.four_connectedness(at, |c| c != &'#')
                .into_iter()
                .map(|p| (p, d + 1)),
        );
    }

    distance
        .iter()
        .tuple_combinations()
        .filter(|((p1, d1), (p2, d2))| {
            let dist = (**p2 - **p1).abs();
            let cheat = dist.x + dist.y;
            if cheat > cheat_secs {
                return false;
            }
            let saved_seconds = (*d2 - *d1).abs() - cheat;
            saved_seconds >= treshold as i32
        })
        .count()
}

#[test]
fn day20_1a() {
    let input = "###############
#...#...#.....#
#.#.#.#.#.###.#
#S#...#.#.#...#
#######.#.#.###
#######.#.#...#
#######.#.###.#
###..E#...#...#
###.#######.###
#...###...#...#
#.#####.#.###.#
#.#...#.#.#...#
#.#.#.#.#.#.###
#...#...#...###
###############";
    let result = parse_and_solve_part_1(input.lines().map(|s| s.to_owned()), 50);
    assert_eq!(result, 1);
}

#[test]
fn day20_1b() {
    let input = "###############
#...#...#.....#
#.#.#.#.#.###.#
#S#...#.#.#...#
#######.#.#.###
#######.#.#...#
#######.#.###.#
###..E#...#...#
###.#######.###
#...###...#...#
#.#####.#.###.#
#.#...#.#.#...#
#.#.#.#.#.#.###
#...#...#...###
###############";
    let result = parse_and_solve_part_1(input.lines().map(|s| s.to_owned()), 40);
    assert_eq!(result, 2);
}

#[test]
fn day20_1c() {
    let input = "###############
#...#...#.....#
#.#.#.#.#.###.#
#S#...#.#.#...#
#######.#.#.###
#######.#.#...#
#######.#.###.#
###..E#...#...#
###.#######.###
#...###...#...#
#.#####.#.###.#
#.#...#.#.#...#
#.#.#.#.#.#.###
#...#...#...###
###############";
    let result = parse_and_solve_part_1(input.lines().map(|s| s.to_owned()), 20);
    assert_eq!(result, 5);
}

#[test]
fn day20_1d() {
    let input = "###############
#...#...#.....#
#.#.#.#.#.###.#
#S#...#.#.#...#
#######.#.#.###
#######.#.#...#
#######.#.###.#
###..E#...#...#
###.#######.###
#...###...#...#
#.#####.#.###.#
#.#...#.#.#...#
#.#.#.#.#.#.###
#...#...#...###
###############";
    let result = parse_and_solve_part_1(input.lines().map(|s| s.to_owned()), 0);
    assert_eq!(result, 14 * 2 + 2 * 2 + 4 + 3 + 5);
}

#[test]
fn day20_2a() {
    let input = "###############
#...#...#.....#
#.#.#.#.#.###.#
#S#...#.#.#...#
#######.#.#.###
#######.#.#...#
#######.#.###.#
###..E#...#...#
###.#######.###
#...###...#...#
#.#####.#.###.#
#.#...#.#.#...#
#.#.#.#.#.#.###
#...#...#...###
###############";
    let result = parse_and_solve_part_2(input.lines().map(|s| s.to_owned()), 50, 20);
    assert_eq!(
        result,
        32 + 31 + 29 + 39 + 25 + 23 + 20 + 19 + 12 + 14 + 12 + 22 + 4 + 3
    );
}

#[test]
fn day20_2b() {
    let input = "###############
#...#...#.....#
#.#.#.#.#.###.#
#S#...#.#.#...#
#######.#.#.###
#######.#.#...#
#######.#.###.#
###..E#...#...#
###.#######.###
#...###...#...#
#.#####.#.###.#
#.#...#.#.#...#
#.#.#.#.#.#.###
#...#...#...###
###############";
    let result = parse_and_solve_part_2(input.lines().map(|s| s.to_owned()), 73, 20);
    assert_eq!(result, 7);
}
