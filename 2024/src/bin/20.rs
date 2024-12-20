use std::collections::BinaryHeap;

use common::{
    datastructs::grid::Grid,
    datatypes::{priority::Priority, vec2::Vec2},
    io::parse_into_lines_automatic,
};

fn main() {
    dbg!(parse_and_solve(parse_into_lines_automatic("20")));
}

fn parse_and_solve(line_iter: impl Iterator<Item = String>) -> usize {
    let grid = Grid::<char>::from_iter(line_iter);
    println!(
        "{:?}",
        grid.four_connectedness(Vec2::new(0, 0), |c| { *c == '#' })
            .iter()
            .map(|p| grid[*p])
            .collect::<Vec<_>>()
    );

    let mut heap = BinaryHeap::new();
    heap.push(Priority::new_min("Max", 0));
    heap.push(Priority::new_min("Max1", 1));
    heap.push(Priority::new_min("Max2", 2));
    heap.push(Priority::new_min("Max0", -1));
    while let Some(v) = heap.pop() {
        println!("{}", v.data);
    }
    0
}

#[test]
fn day20_1() {
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
    let result = parse_and_solve(input.lines().map(|s| s.to_owned()));
    assert_eq!(result, 1);
}
