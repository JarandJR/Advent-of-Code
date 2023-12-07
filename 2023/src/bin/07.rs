use aoc2023::{read_file_string, parse_line, Parse, get_numbers_on_line};

fn main() {
    println!("Result {}", solve(read_file_string("inputs/06.txt").unwrap()));
}

fn solve(data: String) -> i64 {
    let hands = data.lines().into_iter()
        .map(|l| parse_line(l))
        .collect::<Vec<Hand>>();
    println!("{:?}", hands);
    0
}

#[derive(Debug)]
struct Hand {
    hand: String,
    number: i64,
    rand: i32,
}

impl Parse for Hand {
    fn parse(line: &str) -> Self {
        todo!("Parse: {}", line);
    }
}

#[test]
fn test_07_1() {
    assert_eq!(6440, solve("32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483".to_string()));
}
