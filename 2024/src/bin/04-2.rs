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
                if char != 'A' {
                    return acc;
                }
                let north = 0 < (row);
                let south = row < rows - 1;
                let west = 0 < column;
                let east = column < columns - 1;
                let potential_directions = [
                    north && west && south && east,
                    north && east && south && west,
                ];
                let mas = vec!['M', 'A', 'S'];
                let at = (column + row * (columns + 1)) as i32;
                let cross = DIRECTIONS.iter().rev().zip(potential_directions).fold(
                    0,
                    |acc, ((dir_row, dir_colmn), valid)| {
                        if !valid {
                            return acc;
                        }

                        let step = (dir_row * (columns as i32 + 1) + dir_colmn).abs();
                        let mut start = at + dir_row * (columns as i32 + 1) + dir_colmn;
                        let mut end = at + (dir_row * (columns as i32 + 1) + dir_colmn) * -1;
                        // Swap start and end
                        if end < start {
                            let tmp = start;
                            start = end;
                            end = tmp;
                        }
                        let slice = &input[start as usize..=end as usize];
                        if slice
                            .chars()
                            .step_by(step as usize)
                            .eq(mas.iter().map(|&c| c))
                            || slice
                                .chars()
                                .step_by(step as usize)
                                .eq(mas.iter().rev().map(|&c| c))
                        {
                            return acc + 1;
                        }
                        acc
                    },
                );
                acc + cross / 2
            })
        });
    }
    0
}

#[test]
fn day04_2() {
    use std::fs::{remove_file, File};
    let file_name = "test_04_2";
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
    assert_eq!(result, 9);
}
