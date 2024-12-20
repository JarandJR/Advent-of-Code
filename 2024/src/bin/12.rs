use std::collections::HashSet;

use common::{datatypes::vec2::Vec2, io::parse_into_lines_automatic};

fn main() {
    dbg!(parse_and_solve_part_1(parse_into_lines_automatic("12")));
    dbg!(parse_and_solve_part_2(parse_into_lines_automatic("12")));
}

fn parse_and_solve_part_1(line_iter: impl Iterator<Item = String>) -> usize {
    let mut rows = 0;
    let mut columns = 0;
    let grid = line_iter
        .map(|line| {
            columns = line.len();
            rows += 1;
            line.chars().collect::<Vec<char>>()
        })
        .collect::<Vec<Vec<char>>>();

    let mut sum = 0;
    let mut visited = HashSet::new();
    for row in 0..rows {
        for column in 0..columns {
            let at = Vec2::from_row_column(row, column);
            if visited.contains(&at) {
                continue;
            }
            let mut edges = 0;
            let mut plants = 0;
            let crnt = grid[row][column];
            let mut stack = vec![at];
            while let Some(at) = stack.pop() {
                if visited.contains(&at) {
                    continue;
                }
                if grid[at.row()][at.column()] == crnt {
                    plants += 1;
                }
                visited.insert(at);
                for n in get_neighbors(&at, rows, columns) {
                    match n {
                        Some(pos) => {
                            let at = grid[pos.row()][pos.column()];
                            if at == crnt {
                                if visited.contains(&pos) {
                                    continue;
                                }
                                stack.push(pos);
                            } else {
                                edges += 1;
                            }
                        }
                        None => edges += 1,
                    };
                }
            }
            sum += plants * edges;
        }
    }
    sum as usize
}

fn parse_and_solve_part_2(line_iter: impl Iterator<Item = String>) -> usize {
    let mut rows = 0;
    let mut columns = 0;
    let grid = line_iter
        .map(|line| {
            columns = line.len();
            rows += 1;
            line.chars().collect::<Vec<char>>()
        })
        .collect::<Vec<Vec<char>>>();

    let corner_directions = [
        (Vec2::NORTH_WEST, Vec2::NORTH, Vec2::WEST),
        (Vec2::NORTH_EAST, Vec2::NORTH, Vec2::EAST),
        (Vec2::SOUTH_WEST, Vec2::SOUTH, Vec2::WEST),
        (Vec2::SOUTH_EAST, Vec2::SOUTH, Vec2::EAST),
    ];
    let mut sum = 0;
    let mut visited = HashSet::new();
    for row in 0..rows {
        for column in 0..columns {
            let at = Vec2::from_row_column(row, column);
            if visited.contains(&at) {
                continue;
            }
            let mut plants = 0;
            let mut corners = 0;
            let crnt = grid[row][column];
            let mut stack = vec![at];
            while let Some(at) = stack.pop() {
                if visited.contains(&at) {
                    continue;
                }
                if grid[at.row()][at.column()] == crnt {
                    plants += 1;
                }
                visited.insert(at);
                // Count corners
                corner_directions
                    .iter()
                    .map(|(corner_dir, vert, horiz)| {
                        let dia = get_value_in_grid(at + *corner_dir, rows, columns, &grid, crnt);
                        let vertical = get_value_in_grid(at + *vert, rows, columns, &grid, crnt);
                        let horizontal = get_value_in_grid(at + *horiz, rows, columns, &grid, crnt);
                        (dia, vertical, horizontal)
                    })
                    .for_each(|(dia, vert, horiz)| match dia {
                        Some(_) => {
                            if vert == None && horiz == None {
                                corners += 1;
                            }
                        }
                        None => {
                            if vert == horiz {
                                corners += 1;
                            }
                        }
                    });
                // Search for the shape
                for n in get_neighbors(&at, rows, columns) {
                    match n {
                        Some(pos) => {
                            let at = grid[pos.row()][pos.column()];
                            if at == crnt {
                                if visited.contains(&pos) {
                                    continue;
                                }
                                stack.push(pos);
                            }
                        }
                        None => {}
                    };
                }
            }
            sum += plants * corners;
        }
    }
    sum as usize
}

fn get_value_in_grid(
    p: Vec2,
    rows: usize,
    columns: usize,
    grid: &Vec<Vec<char>>,
    crnt: char,
) -> Option<char> {
    if !((0_..rows).contains(&(p.y as usize)) && (0..columns).contains(&(p.x as usize))) {
        None
    } else if grid[p.row()][p.column()] == crnt {
        Some(grid[p.row()][p.column()])
    } else {
        None
    }
}

fn get_neighbors(at: &Vec2, rows: usize, columns: usize) -> Vec<Option<Vec2>> {
    Vec2::FOUR_CONNECTEDNESS
        .iter()
        .map(|dir| {
            let next = *at + *dir;
            if !((0_..rows).contains(&(next.y as usize))
                && (0..columns).contains(&(next.x as usize)))
            {
                return None;
            }
            Some(next)
        })
        .collect()
}

#[test]
fn day12_1a() {
    let input = "RRRRIICCFF
RRRRIICCCF
VVRRRCCFFF
VVRCCCJFFF
VVVVCJJCFE
VVIVCCJJEE
VVIIICJJEE
MIIIIIJJEE
MIIISIJEEE
MMMISSJEEE";
    let result = parse_and_solve_part_1(input.lines().map(|s| s.to_owned()));
    assert_eq!(result, 1930);
}

#[test]
fn day12_1b() {
    let input = "OOOOO
OXOXO
OOOOO
OXOXO
OOOOO";
    let result = parse_and_solve_part_1(input.lines().map(|s| s.to_owned()));
    assert_eq!(result, 772);
}

#[test]
fn day12_1c() {
    let input = "AAAA
BBCD
BBCC
EEEC";
    let result = parse_and_solve_part_1(input.lines().map(|s| s.to_owned()));
    assert_eq!(result, 140);
}

#[test]
fn day12_2a() {
    let input = "RRRRIICCFF
RRRRIICCCF
VVRRRCCFFF
VVRCCCJFFF
VVVVCJJCFE
VVIVCCJJEE
VVIIICJJEE
MIIIIIJJEE
MIIISIJEEE
MMMISSJEEE";
    let result = parse_and_solve_part_2(input.lines().map(|s| s.to_owned()));
    assert_eq!(result, 1206);
}

#[test]
fn day12_2b() {
    let input = "OOOOO
OXOXO
OOOOO
OXOXO
OOOOO";
    let result = parse_and_solve_part_2(input.lines().map(|s| s.to_owned()));
    assert_eq!(result, 436);
}

#[test]
fn day12_2c() {
    let input = "AAAA
BBCD
BBCC
EEEC";
    let result = parse_and_solve_part_2(input.lines().map(|s| s.to_owned()));
    assert_eq!(result, 80);
}

#[test]
fn day12_2d() {
    let input = "EEEEE
EXXXX
EEEEE
EXXXX
EEEEE";
    let result = parse_and_solve_part_2(input.lines().map(|s| s.to_owned()));
    assert_eq!(result, 236);
}

#[test]
fn day12_2e() {
    let input = "AAAAAA
AAABBA
AAABBA
ABBAAA
ABBAAA
AAAAAA";
    let result = parse_and_solve_part_2(input.lines().map(|s| s.to_owned()));
    assert_eq!(result, 368);
}
