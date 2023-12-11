use aoc2023::{read_file_string, get_data_list, Parse, get_numbers_on_line};

fn main() {
    let data = read_file_string("inputs/09.txt").unwrap();
    println!("Result 1: {}", solve(data.clone()));
    println!("Result 2: {}", solve2(data.clone()));
}

fn solve(data: String) -> i64 {
    let trees: Vec<Tree> = get_data_list(data);
    trees.into_iter().map(|t| t.get_next()).fold(0, |a, b| a + b)
}

fn solve2(data: String) -> i64 {
    let trees: Vec<Tree> = get_data_list(data);
    trees.into_iter().map(|t| t.get_prev()).fold(0, |a, b| a + b)
}

#[derive(Debug)]
struct Tree {
    nums: Vec<Vec<i64>>,
}

impl Tree {
    fn new(n: Vec<i64>) -> Self {
        let mut nums = vec![n];
        let mut at = 0;
        while nums.last().unwrap().iter().sum::<i64>() != 0 {
            let mut next = Vec::new();
            for (a, n) in nums[at].iter().enumerate() {
                if a == nums[at].len() - 1 {
                    break;
                }
                next.push(nums[at][a + 1] - n);
            }
            nums.push(next);
            at += 1;
        }
        Tree { nums, }
    }

    fn get_next(&self) -> i64 {
        self.nums.iter().map(|n| n.last().unwrap()).fold(0, |a, b| a + b)
    }

    fn get_prev(&self) -> i64 {
        let mut prev = 0;
        for n in self.nums.iter().rev() {
            prev = n.first().unwrap() - prev;
        }
        prev
    }
}

impl Parse for Tree {
    fn parse(line: &str) -> Self {
        Tree::new(get_numbers_on_line(line))
    }
}

#[test]
fn test_10_1a() {
    assert_eq!(8, solve("..F7.
.FJ|.
SJ.L7
|F--J
LJ...".to_string()));
}

#[test]
fn test_10_1b() {
    assert_eq!(4, solve2(".....
.S-7.
.|.|.
.L-J.
.....".to_string()));
}
