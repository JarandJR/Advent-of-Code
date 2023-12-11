use std::collections::HashMap;

use aoc2023::read_file_string ;

fn main() {
    println!("Result: {}", solve(read_file_string("inputs/08.txt").unwrap()));
}

fn solve(data: String) -> usize {
    let (nodes, pattern) = parse(&data);
    let at = nodes
    .iter()
    .filter(|(k, _)| k.ends_with('A'))
    .collect::<Vec<(&String, &Path)>>();

    let mut paths = Vec::new();
    for (k, n) in at.iter() {
        let mut counter = 0;
        let mut node = *n;
        let mut key = *k;
        let mut p = pattern.chars().into_iter().cycle();
        while !key.ends_with('Z') {
            match p.next() {
                Some('L') => {
                    (key, node) = nodes.get_key_value(&node.left).unwrap();
                },
                Some('R') => {
                    (key, node) = nodes.get_key_value(&node.right).unwrap();
                },
                _ => break,
            }
            counter += 1;
        }
        paths.push(counter);
    }
    let mut lcm = 1;
    for p in paths {
        lcm = lcm * p / gcd(lcm, p);
    }
    lcm
}

fn gcd(a: usize, b: usize) -> usize {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
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
