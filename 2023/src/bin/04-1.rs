use aoc2023::read_file_string;


fn main() {
    println!("Result {}", solve(read_file_string("inputs/04.txt").unwrap()));
}

fn solve(data: String) -> i32 {
    data.lines().into_iter()
    .map(|l| parse(&l))
    .map(|c| points(c))
    .fold(0, |a, b| a + b)
}

fn parse(line: &str) -> Card {
    let line = line[9..].trim().to_string();
    let data = line.split("|").into_iter();
    let winnings = data.clone().into_iter().nth(0).unwrap().trim();
    let numbers = data.into_iter().nth(1).unwrap().trim();
    
    let mut winning = Vec::new();
    for w in winnings.split(" ").filter(|c| !c.is_empty()).into_iter() {
        winning.push(w.parse().unwrap())
    }
    let mut nums = Vec::new();
    for n in numbers.split(" ").filter(|c| !c.is_empty()).into_iter() {
        nums.push(n.parse().unwrap());
    }

    Card { winning, nums }
}

fn points(card: Card) -> i32 {
    let mut matches = -1;
    for n in card.nums {
        if card.winning.contains(&n) {
            matches += 1;
        }
    }
    if matches == -1 {
        return 0;
    }
    (2 as i32).pow(matches as u32)
}

struct Card {
    winning: Vec<i32>,
    nums: Vec<i32>,
}

#[test]
fn test_01_2() {
    let input = String::from("Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11");
    assert_eq!(13, solve(input));
}