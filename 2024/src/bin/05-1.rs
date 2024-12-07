use std::collections::HashMap;

use aoc2024::parse_into_lines;

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
        return orders.iter().fold(0, |acc, order| {
            if order.iter().fold(true, |valid, (num, pos)| {
                if !valid {
                    return valid;
                }
                if let Some(requirements) = page_order.get(num) {
                    if requirements.iter().fold(false, |invalid, n| {
                        if let Some(other_pos) = order.get(n) {
                            if other_pos < pos {
                                /*Invalid*/
                                return true;
                            }
                        }
                        invalid
                    }) {
                        return false;
                    }
                }
                valid
            }) {
                return acc
                    + order
                        .iter()
                        .filter_map(|(num, pos)| {
                            if *pos == order.len() / 2 {
                                return Some(num);
                            }
                            None
                        })
                        .next()
                        .unwrap_or(&0);
            }
            acc
        });
    }
    0
}

#[test]
fn day05_1() {
    use std::fs::{remove_file, File};
    let file_name = "test_05_1";
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
    assert_eq!(result, 143);
}
