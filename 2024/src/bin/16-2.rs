use std::{
    collections::{HashMap, HashSet},
    i32,
};

use common::{datastructs::vec2::Vec2, io::parse_into_lines_automatic};

fn main() {
    dbg!(parse_and_solve("16"));
}

fn parse_and_solve(day: &str) -> usize {
    if let Some(line_iter) = parse_into_lines_automatic(day) {
        let mut start = Vec2::default();
        let mut end = Vec2::default();
        let mut rows = 0;
        let mut columns = 0;
        let mut grid = line_iter
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
        for pos in paths.get(&distance.get(&end).unwrap()).unwrap() {
            grid[pos.row()][pos.column()] = 'O';
        }

        return paths.get(&distance.get(&end).unwrap()).unwrap().len();
    }
    0
}

#[test]
fn day16_2a() {
    use std::fs::{remove_file, File};
    let file_name = "test_16_2a";
    let file_path = format!("inputs/{}.txt", file_name);
    {
        // Setup for test
        use std::io::Write;
        let mut file = File::create(&file_path).expect("Could not create file");
        writeln!(
            file,
            "###############
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
###############"
        )
        .expect("Could not write to file");
    }
    let result = parse_and_solve(&file_name);
    // Clean up
    remove_file(file_path).expect("Could not remove file");
    assert_eq!(result, 45);
}

#[test]
fn day16_2b() {
    use std::fs::{remove_file, File};
    let file_name = "test_16_2b";
    let file_path = format!("inputs/{}.txt", file_name);
    {
        // Setup for test
        use std::io::Write;
        let mut file = File::create(&file_path).expect("Could not create file");
        writeln!(
            file,
            "#################
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
#################"
        )
        .expect("Could not write to file");
    }
    let result = parse_and_solve(&file_name);
    // Clean up
    remove_file(file_path).expect("Could not remove file");
    assert_eq!(result, 64);
}
