use aoc2024::{parse_into_lines, DIRECTIONS};
use itertools::Itertools;

fn main() {
    dbg!(parse_and_solve("04"));
}

fn parse_and_solve(day: &str) -> usize {
    if let Some(mut line_iter) = parse_into_lines(day) {
        let input = line_iter.join("\n");
        let rows = input.split('\n').count();
        return input.lines().enumerate().fold(0, |acc, (row, line)| {
            let columns = line.chars().count();
            acc + line.chars().enumerate().fold(0, |acc, (column, char)| {
                if char != 'X' {
                    return acc;
                }
                let north = 2 < row;
                let south = row < rows - 3;
                let west = 2 < column;
                let east = column < columns - 3;
                let potential_directions = [
                    north,
                    south,
                    west,
                    east,
                    north && west,
                    north && east,
                    south && west,
                    south && east,
                ];
                let at = (column + row * (columns + 1)) as i32;
                acc + DIRECTIONS.iter().zip(potential_directions).fold(
                    0,
                    |acc, ((dir_row, dir_colmn), valid)| {
                        if !valid {
                            return acc;
                        }
                        let step = (dir_row * (columns as i32 + 1) + dir_colmn).abs();
                        let mut start = at;
                        let mut xmas = vec!['X', 'M', 'A', 'S'];
                        let mut end = at + dir_row * (columns as i32 + 1) * 3 + 3 * dir_colmn;
                        // Swap start and end
                        if end < start {
                            start = end;
                            end = at;
                            xmas = xmas.into_iter().rev().collect::<Vec<char>>();
                        }
                        if input[start as usize..=end as usize]
                            .chars()
                            .step_by(step as usize)
                            .eq(xmas.iter().map(|&c| c))
                        {
                            return acc + 1;
                        }

                        acc
                    },
                )
            })
        });
    }
    0
}

#[test]
fn day04_1() {
    use std::fs::{remove_file, File};
    let file_name = "test_04_1";
    let file_path = format!("inputs/{}.txt", file_name);
    {
        // Setup for test
        use std::io::Write;
        let mut file = File::create(&file_path).expect("Could not create file");
        writeln!(
            file,
            "MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX"
        )
        .expect("Could not write to file");
    }
    let result = parse_and_solve(&file_name);
    // Clean up
    remove_file(file_path).expect("Could not remove file");
    assert_eq!(result, 18);
}
