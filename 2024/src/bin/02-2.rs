use std::{
    fs::File,
    io::{BufRead, BufReader},
};

const NEW_LINE: u8 = b'\n';

fn main() {
    dbg!(parse_and_solve("02"));
}

fn is_safe<'a>(mut report: impl Iterator<Item = &'a u32>) -> bool {
    let mut increasing = None;
    let mut a = report.next().unwrap();
    for b in report {
        if a == b || a.abs_diff(*b) > 3 {
            return false;
        }
        match increasing {
            None => increasing = Some(a < b),
            // Swap from increasing to decreasing
            Some(true) => {
                if b < a {
                    return false;
                }
            }
            // Swap from decreasing to increasing
            Some(false) => {
                if a < b {
                    return false;
                }
            }
        }
        a = b;
    }
    true
}

fn parse_and_solve(day: &str) -> usize {
    if let Ok(file) = File::open(format!("inputs/{}.txt", day)) {
        let reader = BufReader::new(file);
        return reader
            .split(NEW_LINE)
            .flat_map(|line| line.ok().into_iter())
            .fold(0, |acc, line| {
                let report = line
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
                    .collect::<Vec<u32>>();
                if is_safe(report.iter()) {
                    return acc + 1;
                }
                for i in 0..report.len() {
                    // Removes i
                    let iter = report[..i].iter().chain(report[(i + 1)..].iter());
                    if is_safe(iter) {
                        return acc + 1;
                    }
                }
                acc
            });
    }
    0
}

#[test]
fn day02_2() {
    use std::fs::{remove_file, File};
    let file_name = "test_02_2";
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
    assert_eq!(result, 4);
}
