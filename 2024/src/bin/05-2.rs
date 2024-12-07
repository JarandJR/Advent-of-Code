use std::collections::HashMap;

use aoc2024::parse_into_lines;
use itertools::Itertools;

fn main() {
    dbg!(parse_and_solve(2024, "05"));
}

fn parse_and_solve(year: i32, day: &str) -> usize {
    if let Some(line_iter) = parse_into_lines(year, day) {
        let mut page_order = HashMap::new();
        let orders = line_iter
            .filter_map(|line| {
                let numbers = line
                    .split(&['|', ',', '\n'][..])
                    .filter_map(|num_str| num_str.parse::<usize>().ok())
                    .collect::<Vec<usize>>();
                if numbers.is_empty() || numbers.len() == 2 {
                    if !numbers.is_empty() {
                        page_order
                            .entry(numbers[0])
                            .or_insert(Vec::new())
                            .push(numbers[1]);
                    }
                    return None;
                }
                Some(
                    numbers
                        .into_iter()
                        .enumerate()
                        .map(|(i, num)| (num, i))
                        .collect::<HashMap<usize, usize>>(),
                )
            })
            .collect::<Vec<_>>();

        let mut invalids = orders
            .into_iter()
            .filter(|order| {
                order.iter().fold(false, |invalid, (num, pos)| {
                    if let Some(requirements) = page_order.get(num) {
                        if requirements.iter().fold(false, |invalid, n| {
                            if let Some(other_pos) = order.get(n) {
                                if other_pos < pos {
                                    /* Invalid */
                                    return true;
                                }
                            }
                            invalid
                        }) {
                            /* Invalid */
                            return true;
                        }
                    }
                    invalid
                })
            })
            .map(|order| {
                order
                    .into_iter()
                    .map(|(key, value)| (key, value))
                    .sorted_by_key(|k| k.1)
                    .map(|(value, _)| value)
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();

        return invalids.iter_mut().fold(0, |acc, invalid| {
            let mut i = 0;
            let mut safe_order = Vec::new();
            while !invalid.is_empty() {
                i %= invalid.len();
                let current_num = invalid[i];
                // Checks if all dependecies are either safe or not present
                let safe = page_order
                    .get(&current_num)
                    .unwrap_or(&Vec::new())
                    .iter()
                    .all(|n| safe_order.contains(n) || !invalid.contains(n));
                if safe {
                    let num = invalid.remove(i);
                    safe_order.push(num);
                }
                i += 1;
            }
            let mid = safe_order.len() / 2;
            acc + safe_order.get(mid).unwrap()
        });
    }
    0
}

#[test]
fn day05_2() {
    use std::fs::{remove_file, File};
    let file_name = "test_05_2";
    let file_path = format!("inputs/{}.txt", file_name);
    {
        // Setup for test
        use std::io::Write;
        let mut file = File::create(&file_path).expect("Could not create file");
        writeln!(
            file,
            "47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47"
        )
        .expect("Could not write to file");
    }
    let result = parse_and_solve(2024, &file_name);
    // Clean up
    remove_file(file_path).expect("Could not remove file");
    assert_eq!(result, 123);
}
