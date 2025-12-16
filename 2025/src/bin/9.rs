use std::collections::BinaryHeap;

use common::{
    datatypes::{priority::Priority, vec2::Vec2},
    io::parse_into_lines_automatic,
};
use itertools::Itertools;

fn main() {
    dbg!(parse_and_solve_1(parse_into_lines_automatic("9")));
    dbg!(parse_and_solve_2(parse_into_lines_automatic("9")));
}

fn parse_and_solve_1(line_iter: impl Iterator<Item = String>) -> usize {
    let red_tiles = line_iter
        .map(|line| {
            let (x, y) = line.split_once(",").unwrap();
            (x.parse::<i64>().unwrap(), y.parse::<i64>().unwrap())
        })
        .collect::<Vec<(i64, i64)>>();
    let mut heap = BinaryHeap::new();
    for i in 0..red_tiles.len() {
        for j in i + 1..red_tiles.len() {
            let area = area(&red_tiles, i, j);
            heap.push(Priority::new_max(area, area));
        }
    }
    heap.pop().unwrap().data as usize
}

fn parse_and_solve_2(line_iter: impl Iterator<Item = String>) -> usize {
    let red_tiles = line_iter
        .map(|line| {
            let (x, y) = line.split_once(",").unwrap();
            Vec2::new(x.parse::<i32>().unwrap(), y.parse::<i32>().unwrap())
        })
        .collect::<Vec<Vec2>>();
    let lines = red_tiles
        .iter()
        .circular_tuple_windows()
        .collect::<Vec<(&Vec2, &Vec2)>>();

    let mut heap = BinaryHeap::new();
    for i in 0..red_tiles.len() {
        for j in i + 1..red_tiles.len() {
            let a = red_tiles[i];
            let b = red_tiles[j];
            let intersect_line = lines.iter().all(|(line_start, line_end)| {
                let max = Vec2::new(a.x.max(b.x), a.y.max(b.y));
                let min = Vec2::new(a.x.min(b.x), a.y.min(b.y));

                let line_max =
                    Vec2::new(line_start.x.max(line_end.x), line_start.y.max(line_end.y));
                let line_min =
                    Vec2::new(line_start.x.min(line_end.x), line_start.y.min(line_end.y));

                let left = max.x <= line_min.x;
                let right = min.x >= line_max.x;
                let above = max.y <= line_min.y;
                let below = min.y >= line_max.y;

                left || right || above || below
            });
            if intersect_line {
                let area = (a.x.abs_diff(b.x) + 1) * (a.y.abs_diff(b.y) + 1);
                heap.push(Priority::new_max(area, area));
            }
        }
    }
    heap.pop().unwrap().data as usize
}

fn area(coords: &[(i64, i64)], i: usize, j: usize) -> i64 {
    let dx = (coords[i].0 - coords[j].0).abs() + 1;
    let dy = (coords[i].1 - coords[j].1).abs() + 1;
    dx * dy
}

#[test]
fn day9_1() {
    let input = "7,1
11,1
11,7
9,7
9,5
2,5
2,3
7,3";
    let result = parse_and_solve_1(input.lines().map(|s| s.to_owned()));
    assert_eq!(result, 50);
}

#[test]
fn day9_2() {
    let input = "7,1
11,1
11,7
9,7
9,5
2,5
2,3
7,3";
    let result = parse_and_solve_2(input.lines().map(|s| s.to_owned()));
    assert_eq!(result, 24);
}
