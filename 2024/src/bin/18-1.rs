use std::collections::HashSet;

use common::{datatypes::vec2::Vec2, io::parse_into_lines_automatic};
use itertools::Itertools;

fn main() {
    dbg!(parse_and_solve(parse_into_lines_automatic("18"), 70, 1024));
}

fn parse_and_solve(line_iter: impl Iterator<Item = String>, size: usize, bytes: usize) -> usize {
    let corrupted_bytes = line_iter
        .take(bytes)
        .flat_map(|line| {
            line.split(',')
                .filter_map(|str| str.parse::<usize>().ok())
                .tuple_windows()
                .map(|(x, y)| Vec2::from_point(x, y))
                .collect::<HashSet<Vec2>>()
        })
        .collect::<HashSet<Vec2>>();
    let end = Vec2::from_row_column(size, size);
    let start = Vec2::new(0, 0);

    let mut stack = vec![(start, 0)];
    let mut visited = HashSet::new();
    loop {
        // BFS
        let mut next_breadth_search = Vec::new();
        while let Some((p, d)) = stack.pop() {
            if visited.contains(&p) {
                continue;
            }
            if p == end {
                return d;
            }
            visited.insert(p);
            let next = Vec2::FOUR_CONNECTEDNESS
                .iter()
                .filter_map(|dir| {
                    let dir = *dir + p;
                    if !((0_i32..=size as i32).contains(&dir.y)
                        && (0_i32..=size as i32).contains(&dir.x))
                    {
                        return None;
                    }
                    if corrupted_bytes.contains(&dir) {
                        return None;
                    }
                    Some((dir, d + 1))
                })
                .collect::<Vec<(Vec2, usize)>>();
            next_breadth_search.extend(next);
        }
        stack = next_breadth_search;
    }
}

#[test]
fn day18_1() {
    let input = "5,4
4,2
4,5
3,0
2,1
6,3
2,4
1,5
0,6
3,3
2,6
5,1
1,2
5,5
2,5
6,5
1,4
0,4
6,4
1,1
6,1
1,0
0,5
1,6
2,0";
    let result = parse_and_solve(input.lines().map(|s| s.to_owned()), 6, 12);
    assert_eq!(result, 22);
}
