use std::collections::HashMap;

use common::io::parse_into_lines_automatic;

fn main() {
    dbg!(parse_and_solve(parse_into_lines_automatic("11"), 25));
    dbg!(parse_and_solve(parse_into_lines_automatic("11"), 75));
}

fn parse_and_solve(line_iter: impl Iterator<Item = String>, blinks: usize) -> usize {
    let stones = line_iter
        .flat_map(|line| {
            line.split_ascii_whitespace()
                .filter_map(|str| str.parse().ok())
                .collect::<Vec<u128>>()
        })
        .collect::<Vec<_>>();

    let mut sum = 0;
    let mut breadths = HashMap::new();
    for stone in stones {
        // DFS
        let mut stack = vec![(stone, blinks, Vec::new())];
        while let Some((num, blinks_left, mut prevs)) = stack.pop() {
            if breadths.contains_key(&(num, blinks_left)) {
                let breadth = *breadths.get(&(num, blinks_left)).unwrap();
                // Update memory
                let mut blinks = blinks_left;
                while let Some(num) = prevs.pop() {
                    blinks += 1;
                    *breadths.entry((num, blinks)).or_insert(0) += breadth;
                }
                continue;
            }
            if blinks_left == 0 {
                // Update memory
                let mut blinks = 0;
                while let Some(num) = prevs.pop() {
                    blinks += 1;
                    *breadths.entry((num, blinks)).or_insert(0) += 1;
                }
                continue;
            }

            prevs.push(num);
            if num == 0 {
                // Case 1
                stack.push((1, blinks_left - 1, prevs));
                continue;
            }
            let str = num.to_string();
            if str.len() % 2 == 0 {
                // Case 2
                let mid = str.len() / 2;
                let num1 = str[0..mid].parse::<u128>().unwrap();
                let num2 = str[mid..].parse::<u128>().unwrap();
                stack.extend([
                    (num1, blinks_left - 1, prevs.clone()),
                    (num2, blinks_left - 1, prevs),
                ]);
            } else {
                // Case 3
                stack.push((num * 2024, blinks_left - 1, prevs));
            }
        }
        sum += breadths.get(&(stone, blinks)).unwrap_or(&0);
    }
    sum
}

#[test]
fn day11_1() {
    let input = "125 17";
    let result = parse_and_solve(input.lines().map(|s| s.to_owned()), 25);
    assert_eq!(result, 55312);
}
