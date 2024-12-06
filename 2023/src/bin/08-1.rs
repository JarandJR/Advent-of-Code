use std::collections::HashMap;

use aoc2023::{read_file_string, LinkedList};

fn main() {
    println!(
        "Result: {}",
        solve(read_file_string("inputs/08.txt").unwrap())
    );
}

fn solve(data: String) -> usize {
    let (nodes, list) = parse(&data);
    let mut counter = 0;
    let mut key = "AAA";
    let mut at = nodes.get(key).unwrap();
    let mut dir = list.head.as_ref();
    while key != "ZZZ" {
        if dir.unwrap().data == "L" {
            key = &at.left;
        } else {
            key = &at.right;
        }
        at = nodes.get(key).unwrap();
        dir = dir.unwrap().next.as_ref();
        if dir.is_none() {
            dir = list.head.as_ref();
        }
        counter += 1;
    }
    counter
}

fn parse(data: &str) -> (HashMap<String, Path>, LinkedList<String>) {
    let mut map = HashMap::new();
    let mut it = data.lines().into_iter();
    let list = get_directions(it.next().unwrap());
    while let Some(line) = it.next() {
        if line.is_empty() {
            continue;
        }
        let (key, node) = parse_path(line);
        map.insert(key, node);
    }
    (map, list)
}

fn get_directions(line: &str) -> LinkedList<String> {
    let mut list = LinkedList::new();
    let reversed_chars: Vec<String> = line.chars().map(|c| c.to_string()).rev().collect();
    for c in reversed_chars {
        list.push(c);
    }
    list
}

#[derive(Debug)]
struct Path {
    left: String,
    right: String,
}

fn parse_path(line: &str) -> (String, Path) {
    let mut it = line.trim().split(" ").into_iter();
    let key = it.next().unwrap().to_string();
    it.next();
    let left = it
        .next()
        .unwrap()
        .to_string()
        .replace("(", "")
        .replace(",", "")
        .to_string();
    let right = it.next().unwrap().to_string().replace(")", "").to_string();
    (key, Path { left, right })
}

#[test]
fn test_08_1a() {
    assert_eq!(
        2,
        solve(
            "RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)"
                .to_string()
        )
    );
}

#[test]
fn test_08_1b() {
    assert_eq!(
        6,
        solve(
            "LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)"
                .to_string()
        )
    );
}
