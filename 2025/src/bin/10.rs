use std::collections::{HashSet, VecDeque};

use common::io::parse_into_lines_automatic;

fn main() {
    dbg!(parse_and_solve_1(parse_into_lines_automatic("10")));
}

fn parse_and_solve_1(line_iter: impl Iterator<Item = String>) -> usize {
    line_iter.fold(0, |acc, line| {
        let (fasit, buttons, _) = parse(line);
        let buttons = buttons
            .iter()
            .map(|b| b.iter().fold(0, |acc, n| acc | 1 << n))
            .collect::<Vec<u16>>();
        acc + bfs(fasit, buttons)
    })
}

fn bfs(fasit: u16, buttons: Vec<u16>) -> usize {
    let mut visited = HashSet::new();
    let mut queue = VecDeque::new();
    for &b in &buttons {
        visited.insert(b);
        queue.push_back((b, 1));
    }
    while let Some((state, depth)) = queue.pop_front() {
        if state == fasit {
            return depth;
        }
        for &b in &buttons {
            let next_st = state ^ b;
            if visited.insert(next_st) {
                queue.push_back((next_st, depth + 1));
            }
        }
    }
    panic!("No solution");
}

fn parse(line: String) -> (u16, Vec<Vec<usize>>, Vec<usize>) {
    let fasit = line
        .chars()
        .take_while(|c| !c.is_whitespace())
        .filter(|c| !(c.eq(&'[') || c.eq(&']')))
        .enumerate()
        .fold(0_u16, |acc, (i, c)| acc | if c == '#' { 1 << i } else { 0 });
    let mut buttons = line
        .split(" ")
        .skip(1)
        .map(|s| {
            s.split(",")
                .map(|s| {
                    s.chars()
                        .filter(|c| c.is_numeric())
                        .collect::<String>()
                        .parse::<usize>()
                        .unwrap()
                })
                .collect::<Vec<usize>>()
        })
        .collect::<Vec<Vec<usize>>>();
    let joltage_required = buttons.pop().unwrap();
    (fasit, buttons, joltage_required)
}

#[test]
fn day10_1() {
    let input = "[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}
[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}
[.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}";
    let result = parse_and_solve_1(input.lines().map(|s| s.to_owned()));
    assert_eq!(result, 7);
}
