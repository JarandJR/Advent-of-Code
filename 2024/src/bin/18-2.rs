use std::collections::HashSet;

use common::{datastructs::vec2::Vec2, io::parse_into_lines_automatic};
use itertools::Itertools;

fn main() {
    dbg!(parse_and_solve("18", 70, 1024));
}

fn parse_and_solve(day: &str, size: usize, bytes: usize) -> (usize, usize) {
    if let Some(line_iter) = parse_into_lines_automatic(day) {
        let corrupted_bytes = line_iter
            .flat_map(|line| {
                line.split(',')
                    .filter_map(|str| str.parse::<usize>().ok())
                    .tuple_windows()
                    .map(|(x, y)| Vec2::from_point(x, y))
                    .collect::<HashSet<Vec2>>()
            })
            .collect::<Vec<Vec2>>();
        let end = Vec2::from_row_column(size, size);
        let start = Vec2::new(0, 0);

        let mut bytes = bytes;
        loop {
            let curnt_corrupted_bytes = corrupted_bytes.iter().take(bytes).collect::<HashSet<_>>();
            let mut stack = vec![(start, 0)];
            let mut visited = HashSet::new();
            let mut exit_reached = false;
            while !stack.is_empty() {
                // BFS
                let mut next_breadth_search = Vec::new();
                while let Some((p, d)) = stack.pop() {
                    if visited.contains(&p) {
                        continue;
                    }
                    if p == end {
                        exit_reached = true;
                        next_breadth_search.clear();
                        break;
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
                            if curnt_corrupted_bytes.contains(&dir) {
                                return None;
                            }
                            Some((dir, d + 1))
                        })
                        .collect::<Vec<(Vec2, usize)>>();
                    next_breadth_search.extend(next);
                }
                stack = next_breadth_search;
            }
            if !exit_reached {
                return corrupted_bytes.get(bytes - 1).unwrap().to_tupple();
            }
            bytes += 1;
        }
    }
    panic!("Failed to read input file")
}

#[test]
fn day18_2() {
    use std::fs::{remove_file, File};
    let file_name = "test_18_2";
    let file_path = format!("inputs/{}.txt", file_name);
    {
        // Setup for test
        use std::io::Write;
        let mut file = File::create(&file_path).expect("Could not create file");
        writeln!(
            file,
            "5,4
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
2,0"
        )
        .expect("Could not write to file");
    }
    let result = parse_and_solve(&file_name, 6, 12);
    // Clean up
    remove_file(file_path).expect("Could not remove file");
    assert_eq!(result, (6, 1));
}
