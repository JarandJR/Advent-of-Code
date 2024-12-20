use std::collections::HashSet;

use common::{datatypes::vec2::Vec2, io::parse_into_lines_automatic};

fn main() {
    dbg!(parse_and_solve_1(parse_into_lines_automatic("10")));
    dbg!(parse_and_solve_2(parse_into_lines_automatic("10")));
}

const ASCII_ZERO: u8 = 48;

fn parse_and_solve_1(line_iter: impl Iterator<Item = String>) -> usize {
    let mut trailheads = HashSet::new();
    let mut rows = 0;
    let mut columns = 0;

    let grid = line_iter
        .enumerate()
        .map(|(row, line)| {
            columns = line.len() as i32;
            rows += 1;
            line.chars()
                .enumerate()
                .map(|(column, c)| {
                    let num = c as u8 - ASCII_ZERO;
                    if num == 0 {
                        trailheads.insert(Vec2::from_row_column(row, column));
                    }
                    num
                })
                .collect::<Vec<u8>>()
        })
        .collect::<Vec<Vec<u8>>>();

    let mut sum = 0;
    for trailhead in trailheads.iter() {
        let mut trails = HashSet::new();
        let mut visited = HashSet::new();
        let neighbours = get_valid_neighbours(trailhead, rows, columns, &grid);
        let mut stack = neighbours;
        while let Some(neigh) = stack.pop() {
            if visited.contains(&neigh) {
                // Already visited
                continue;
            }
            visited.insert(neigh);
            stack.extend(get_valid_neighbours(&neigh, rows, columns, &grid));
            if grid[neigh.y as usize][neigh.x as usize] == 9 {
                // Found trail
                trails.insert(neigh);
            }
        }
        sum += trails.len();
    }
    sum
}

fn get_valid_neighbours(pos: &Vec2, rows: i32, columns: i32, grid: &Vec<Vec<u8>>) -> Vec<Vec2> {
    let neighbours = Vec2::FOUR_CONNECTEDNESS
        .iter()
        .filter_map(|dir| {
            let next = *pos + *dir;
            if !((0..rows).contains(&next.y) && (0..columns).contains(&next.x)) {
                return None;
            }
            let prev = grid[pos.y as usize][pos.x as usize];
            if grid[next.y as usize][next.x as usize] != prev + 1 {
                return None;
            }
            Some(next)
        })
        .collect::<Vec<Vec2>>();
    neighbours
}

fn parse_and_solve_2(line_iter: impl Iterator<Item = String>) -> usize {
    let mut trailheads = HashSet::new();
    let mut rows = 0;
    let mut columns = 0;

    let grid = line_iter
        .enumerate()
        .map(|(row, line)| {
            columns = line.len() as i32;
            rows += 1;
            line.chars()
                .enumerate()
                .map(|(column, c)| {
                    let num = c as u8 - ASCII_ZERO;
                    if num == 0 {
                        trailheads.insert(Vec2::from_row_column(row, column));
                    }
                    num
                })
                .collect::<Vec<u8>>()
        })
        .collect::<Vec<Vec<u8>>>();

    let mut sum = 0;
    for trailhead in trailheads.iter() {
        let neighbours = get_valid_neighbours(trailhead, rows, columns, &grid);
        let mut stack = neighbours;
        while let Some(neigh) = stack.pop() {
            stack.extend(get_valid_neighbours(&neigh, rows, columns, &grid));
            if grid[neigh.y as usize][neigh.x as usize] == 9 {
                // Found trail
                sum += 1;
            }
        }
    }
    sum
}

#[test]
fn day10_1() {
    let input = "89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732";
    let result = parse_and_solve_1(input.lines().map(|s| s.to_owned()));
    assert_eq!(result, 36);
}

#[test]
fn day10_2() {
    let input = "89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732";
    let result = parse_and_solve_2(input.lines().map(|s| s.to_owned()));
    assert_eq!(result, 81);
}
