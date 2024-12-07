use std::iter::Peekable;

use common::io::parse_into_lines;

fn main() {
    dbg!(parse_and_solve(2024, "03"));
}

fn get_number(
    iter: &mut Peekable<impl Iterator<Item = char>>,
) -> Result<usize, std::num::ParseIntError> {
    let mut number = String::new();
    while let Some(c) = iter.peek() {
        if !c.is_numeric() {
            break;
        }
        number.push(*c);
        _ = iter.next();
    }
    number.chars().rev().collect::<String>().parse::<usize>()
}

fn get_do_dont(iter: &mut impl Iterator<Item = char>) -> Option<bool> {
    let str = iter.collect::<String>();
    if str.starts_with("(od") {
        return Some(true);
    }
    if str.starts_with("(t'nod") {
        return Some(false);
    }
    None
}

fn parse_and_solve(year: i32, day: &str) -> usize {
    if let Some(line_iter) = parse_into_lines(year, day) {
        let mut enabled = true;
        return line_iter.fold(0, |acc, line| {
            acc + line.split(')').fold(0, |acc, str| {
                let mut iter = str.chars().rev().peekable();
                if let Some(enabling) = get_do_dont(&mut iter.clone()) {
                    enabled = enabling;
                }
                if !enabled {
                    return acc;
                }
                let mut multiply = Vec::with_capacity(2);
                let mut expected = vec!['(', ','];
                while let Ok(number) = get_number(&mut iter) {
                    // Skip ',' & '('
                    match iter.next() {
                        Some(skip) => match expected.pop() {
                            Some(ex) => {
                                if skip == ex {
                                    multiply.push(number);
                                }
                            }
                            _ => (),
                        },
                        _ => (),
                    };
                }
                if !multiply.is_empty() && iter.collect::<String>().starts_with("lum") {
                    return acc + multiply.iter().product::<usize>();
                }
                acc
            })
        });
    }
    0
}

#[test]
fn day03_2() {
    use std::fs::{remove_file, File};
    let file_name = "test_03_2";
    let file_path = format!("inputs/{}.txt", file_name);
    {
        // Setup for test
        use std::io::Write;
        let mut file = File::create(&file_path).expect("Could not create file");
        writeln!(
            file,
            "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))"
        )
        .expect("Could not write to file");
    }
    let result = parse_and_solve(2024, &file_name);
    // Clean up
    remove_file(file_path).expect("Could not remove file");
    assert_eq!(result, 48);
}
