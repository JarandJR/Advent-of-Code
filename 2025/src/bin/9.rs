use std::collections::BinaryHeap;

use common::{datatypes::priority::Priority, io::parse_into_lines_automatic};

fn main() {
    dbg!(parse_and_solve(parse_into_lines_automatic("9")));
}

fn parse_and_solve(line_iter: impl Iterator<Item = String>) -> usize {
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

fn area(coords: &Vec<(i64, i64)>, i: usize, j: usize) -> i64 {
    let dx = (coords[i].0 - coords[j].0).abs() + 1;
    let dy = (coords[i].1 - coords[j].1).abs() + 1;
    let area = dx * dy;
    area
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
    let result = parse_and_solve(input.lines().map(|s| s.to_owned()));
    assert_eq!(result, 50);
}
