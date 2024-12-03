use std::{
    fs::File,
    io::{BufRead, BufReader},
};

use itertools::Itertools;

const NEW_LINE: u8 = b'\n';

fn main() {
    dbg!(parse_and_solve("02"));
}

fn parse_and_solve(day: &str) -> usize {
    if let Ok(file) = File::open(format!("inputs/{}.txt", day)) {
        let reader = BufReader::new(file);
        return reader
            .split(NEW_LINE)
            .flat_map(|line| line.ok().into_iter())
            .fold(0, |acc, line| {
                acc + (line
                    .split(|b| b.is_ascii_whitespace())
                    .filter_map(|chunk| {
                        if chunk.is_empty() {
                            return None;
                        }
                        Some(chunk.iter().fold(0, |acc, b| {
                            let num = (b - 48) as u32;
                            if acc > 0 {
                                return acc * 10 + num;
                            }
                            num
                        }))
                    })
                    .tuple_windows()
                    .fold(0_u8, |acc, (a, b)| {
                        // Unsafe bit 8
                        if acc >> 7 == 1 {
                            return acc;
                        }
                        // Started bit 7
                        let started = acc >> 6 == 1;
                        // Increasing bit 6
                        let increasing = (acc >> 5) & 1 == 1;
                        if started {
                            if increasing {
                                if b < a {
                                    // Unsafe
                                    return 0 | (1 << 7);
                                }
                                let diff = b - a;
                                if diff > 0 && diff < 4 {
                                    return acc;
                                }
                                // Unsafe
                                0 | (1 << 7)
                            } else {
                                // Decreasing
                                if a < b {
                                    // Unsafe
                                    return 0 | (1 << 7);
                                }
                                let diff = a - b;
                                if diff > 0 && diff < 4 {
                                    return acc;
                                }
                                // Unsafe
                                0 | (1 << 7)
                            }
                        } else {
                            // Started sequence
                            let status = acc | (1 << 6);
                            if a < b {
                                let diff = b - a;
                                if diff > 0 && diff < 4 {
                                    // Increasing
                                    return status | (1 << 5);
                                }
                                // Unsafe
                                return 1 << 7;
                            }
                            let diff = a - b;
                            if diff > 0 && diff < 4 {
                                // Decreasing
                                return status;
                            }
                            1 << 7
                        }
                    })
                    >> 7
                    ^ 1) as usize
            });
    }
    0
}

#[test]
fn day02_1() {
    use std::fs::{remove_file, File};
    let file_name = "test_02_1";
    let file_path = format!("inputs/{}.txt", file_name);
    {
        // Setup for test
        use std::io::Write;
        let mut file = File::create(&file_path).expect("Could not create file");
        writeln!(
            file,
            "7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9"
        )
        .expect("Could not write to file");
    }
    let result = parse_and_solve(&file_name);
    // Clean up
    remove_file(file_path).expect("Could not remove file");
    assert_eq!(result, 2)
}
