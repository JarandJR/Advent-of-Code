use std::collections::HashMap;

use aoc2023::read_file_string ;

fn main() {
    println!("Result: {}", solve(read_file_string("inputs/08.txt").unwrap()));
}

fn solve(data: String) -> usize {
    let (nodes, pattern) = parse(&data);
    let mut at = nodes.clone()
    .into_iter()
    .filter(|(k, _)| k.ends_with('A'))
    .collect::<Vec<(String, Path)>>();
    let mut pattern = pattern.chars().into_iter().cycle();
    let mut counter = 0;
    loop {
        if at.clone().into_iter().all(|(k, _)| k.ends_with('Z')) {
            break;
        };
        if counter % 1_000_000 == 0 {
            println!("At: {}", counter);
        }

        match pattern.next() {
            Some('L') => {
                at = at.clone().into_iter()
                .map(|(_, n)| {
                let (k, node) = nodes.get_key_value(&n.left).unwrap();
                (k.clone(), node.clone())
                }
            ).collect();
            },
            Some('R') => {
                at = at.clone().into_iter()
                .map(|(_, n)| {
                let (k, node) = nodes.get_key_value(&n.right).unwrap();
                (k.clone(), node.clone())
                }
            ).collect();
            },
            _ => break,
        }
        counter += 1;
    }
    counter
}

fn parse(data: &str) -> (HashMap<String, Path>, String) {
    let mut map = HashMap::new();
    let mut it = data.lines().into_iter();
    let pattern = it.next().unwrap().to_string();
    while let Some(line) = it.next() {
        if line.is_empty() {
            continue;
        }
        let (key, node) = parse_path(line);
        map.insert(key, node);
    }
    (map, pattern)
}

#[derive(Debug, Clone)]
struct Path {
    left: String,
    right: String,
}

fn parse_path(line: &str) -> (String, Path) {
    let mut it = line.trim().split(" ").into_iter();
    let key = it.next().unwrap().to_string();
    it.next();
    let left = it.next().unwrap().to_string()
    .replace("(", "").replace(",", "")
    .to_string();
    let right = it.next().unwrap().to_string()
    .replace(")", "").to_string();
    (key, Path { left, right,})
}

#[test]
fn test_08_2() {
    assert_eq!(6, solve("LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)".to_string()));
}
