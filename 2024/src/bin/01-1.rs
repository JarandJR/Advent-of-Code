use itertools::Itertools;
use std::{
    cmp::Reverse,
    collections::BinaryHeap,
    fs::File,
    io::{BufRead, BufReader},
};

fn main() {
    let (left_heap, right_heap) = parse("2024/", "01");
    dbg!(solve(left_heap, right_heap));
}

fn solve(
    mut left_heap: BinaryHeap<Reverse<i32>>,
    mut right_heap: BinaryHeap<Reverse<i32>>,
) -> usize {
    let mut acc = 0;
    while let (Some(Reverse(l)), Some(Reverse(r))) = (left_heap.pop(), right_heap.pop()) {
        acc += (l - r).abs() as usize;
    }
    acc
}

fn parse(year: &str, day: &str) -> (BinaryHeap<Reverse<i32>>, BinaryHeap<Reverse<i32>>) {
    let mut left_heap = BinaryHeap::new();
    let mut rigth_heap = BinaryHeap::new();
    if let Ok(file) = File::open(format!("{}inputs/{}.txt", year, day)) {
        let mut reader = BufReader::new(file);
        let mut buffer = String::new();
        while {
            if let Ok(bytes_read) = reader.read_line(&mut buffer) {
                bytes_read != 0
            } else {
                false
            }
        } {
            let (left, right) = buffer
                .split_ascii_whitespace()
                .chunks(2)
                .into_iter()
                .map(|chunk| {
                    chunk
                        .into_iter()
                        .map(|s| s.parse::<i32>().unwrap())
                        .collect::<Vec<_>>()
                })
                .fold((0, 0), |_, nums| (nums[0], nums[1]));
            left_heap.push(Reverse(left));
            rigth_heap.push(Reverse(right));
            buffer.clear();
        }
    }
    (left_heap, rigth_heap)
}

#[test]
fn day01_1() {
    use std::fs::remove_file;
    let file_name = "test_01_1";
    let file_path = format!("inputs/{}.txt", file_name);
    {
        // Setup for test
        use std::io::Write;
        let mut file = File::create(&file_path).expect("Could not create file");
        writeln!(
            file,
            "3   4
4   3
2   5
1   3
3   9
3   3"
        )
        .expect("Could not write to file");
    }
    let (left_heap, right_heap) = parse("", &file_name);
    // Clean up
    remove_file(file_path).expect("Could not remove file");
    assert_eq!(solve(left_heap, right_heap), 11);
}
