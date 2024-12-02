use itertools::Itertools;
use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader},
};

fn main() {
    let (nums, right_heap) = parse("01");
    dbg!(solve(nums, right_heap));
}

fn solve(nums: Vec<usize>, right_set: HashMap<usize, usize>) -> usize {
    nums.iter().fold(0, |acc, left| {
        if let Some(r) = right_set.get(&left) {
            return acc + left * r;
        }
        acc
    })
}

fn parse(day: &str) -> (Vec<usize>, HashMap<usize, usize>) {
    let mut nums = Vec::new();
    let mut rigth_set = HashMap::new();
    if let Ok(file) = File::open(format!("inputs/{}.txt", day)) {
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
                        .map(|s| s.parse::<usize>().unwrap())
                        .collect::<Vec<_>>()
                })
                .fold((0, 0), |_, nums| (nums[0], nums[1]));
            nums.push(left);
            *rigth_set.entry(right).or_insert(0) += 1;
            buffer.clear();
        }
    }
    (nums, rigth_set)
}

#[test]
fn day01_2() {
    use std::fs::remove_file;
    let file_name = "test_01_2";
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
    let (left_heap, right_heap) = parse(&file_name);
    // Clean up
    remove_file(file_path).expect("Could not remove file");
    assert_eq!(solve(left_heap, right_heap), 31);
}
