use std::collections::HashSet;

use common::{datastructs::vec2::Vec2, io::parse_into_lines_automatic};

fn main() {
    dbg!(parse_and_solve_part_1("12"));
    dbg!(parse_and_solve_part_2("12"));
}

fn parse_and_solve_part_1(day: &str) -> usize {
    if let Some(line_iter) = parse_into_lines_automatic(day) {
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
        return sum as usize;
    }
    0
}

fn parse_and_solve_part_2(day: &str) -> usize {
    if let Some(line_iter) = parse_into_lines_automatic(day) {
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
                            let dia =
                                get_value_in_grid(at + *corner_dir, rows, columns, &grid, crnt);
                            let vertical =
                                get_value_in_grid(at + *vert, rows, columns, &grid, crnt);
                            let horizontal =
                                get_value_in_grid(at + *horiz, rows, columns, &grid, crnt);
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
        return sum as usize;
    }
    0
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
    use std::fs::{remove_file, File};
    let file_name = "test_12_1a";
    let file_path = format!("inputs/{}.txt", file_name);
    {
        // Setup for test
        use std::io::Write;
        let mut file = File::create(&file_path).expect("Could not create file");
        writeln!(
            file,
            "RRRRIICCFF
RRRRIICCCF
VVRRRCCFFF
VVRCCCJFFF
VVVVCJJCFE
VVIVCCJJEE
VVIIICJJEE
MIIIIIJJEE
MIIISIJEEE
MMMISSJEEE"
        )
        .expect("Could not write to file");
    }
    let result = parse_and_solve_part_1(&file_name);
    // Clean up
    remove_file(file_path).expect("Could not remove file");
    assert_eq!(result, 1930);
}

#[test]
fn day12_1b() {
    use std::fs::{remove_file, File};
    let file_name = "test_12_1b";
    let file_path = format!("inputs/{}.txt", file_name);
    {
        // Setup for test
        use std::io::Write;
        let mut file = File::create(&file_path).expect("Could not create file");
        writeln!(
            file,
            "OOOOO
OXOXO
OOOOO
OXOXO
OOOOO"
        )
        .expect("Could not write to file");
    }
    let result = parse_and_solve_part_1(&file_name);
    // Clean up
    remove_file(file_path).expect("Could not remove file");
    assert_eq!(result, 772);
}

#[test]
fn day12_1c() {
    use std::fs::{remove_file, File};
    let file_name = "test_12_1c";
    let file_path = format!("inputs/{}.txt", file_name);
    {
        // Setup for test
        use std::io::Write;
        let mut file = File::create(&file_path).expect("Could not create file");
        writeln!(
            file,
            "AAAA
BBCD
BBCC
EEEC"
        )
        .expect("Could not write to file");
    }
    let result = parse_and_solve_part_1(&file_name);
    // Clean up
    remove_file(file_path).expect("Could not remove file");
    assert_eq!(result, 140);
}

#[test]
fn day12_2a() {
    use std::fs::{remove_file, File};
    let file_name = "test_12_2a";
    let file_path = format!("inputs/{}.txt", file_name);
    {
        // Setup for test
        use std::io::Write;
        let mut file = File::create(&file_path).expect("Could not create file");
        writeln!(
            file,
            "RRRRIICCFF
RRRRIICCCF
VVRRRCCFFF
VVRCCCJFFF
VVVVCJJCFE
VVIVCCJJEE
VVIIICJJEE
MIIIIIJJEE
MIIISIJEEE
MMMISSJEEE"
        )
        .expect("Could not write to file");
    }
    let result = parse_and_solve_part_2(&file_name);
    // Clean up
    remove_file(file_path).expect("Could not remove file");
    assert_eq!(result, 1206);
}

#[test]
fn day12_2b() {
    use std::fs::{remove_file, File};
    let file_name = "test_12_2b";
    let file_path = format!("inputs/{}.txt", file_name);
    {
        // Setup for test
        use std::io::Write;
        let mut file = File::create(&file_path).expect("Could not create file");
        writeln!(
            file,
            "OOOOO
OXOXO
OOOOO
OXOXO
OOOOO"
        ) // O at 1, 2 miswsing 2 (left & right)
        // And O at 3, 0 missing down
        .expect("Could not write to file");
    }
    let result = parse_and_solve_part_2(&file_name);
    // Clean up
    remove_file(file_path).expect("Could not remove file");
    assert_eq!(result, 436); // 20 (only 17) * 21 (good) + 16 (good)
}

#[test]
fn day12_2c() {
    use std::fs::{remove_file, File};
    let file_name = "test_12_2c";
    let file_path = format!("inputs/{}.txt", file_name);
    {
        // Setup for test
        use std::io::Write;
        let mut file = File::create(&file_path).expect("Could not create file");
        writeln!(
            file,
            "AAAA
BBCD
BBCC
EEEC"
        )
        .expect("Could not write to file");
    }
    let result = parse_and_solve_part_2(&file_name);
    // Clean up
    remove_file(file_path).expect("Could not remove file");
    assert_eq!(result, 80);
}

#[test]
fn day12_2d() {
    use std::fs::{remove_file, File};
    let file_name = "test_12_2d";
    let file_path = format!("inputs/{}.txt", file_name);
    {
        // Setup for test
        use std::io::Write;
        let mut file = File::create(&file_path).expect("Could not create file");
        writeln!(
            file,
            "EEEEE
EXXXX
EEEEE
EXXXX
EEEEE"
        ) // missing E at x = 4 * 2
        .expect("Could not write to file");
    }
    let result = parse_and_solve_part_2(&file_name);
    // Clean up
    remove_file(file_path).expect("Could not remove file");
    assert_eq!(result, 236);
}

#[test]
fn day12_2e() {
    use std::fs::{remove_file, File};
    let file_name = "test_12_2e";
    let file_path = format!("inputs/{}.txt", file_name);
    {
        // Setup for test
        use std::io::Write;
        let mut file = File::create(&file_path).expect("Could not create file");
        writeln!(
            file,
            "AAAAAA
AAABBA
AAABBA
ABBAAA
ABBAAA
AAAAAA"
        )
        .expect("Could not write to file");
    }
    let result = parse_and_solve_part_2(&file_name);
    // Clean up
    remove_file(file_path).expect("Could not remove file");
    assert_eq!(result, 368);
}
