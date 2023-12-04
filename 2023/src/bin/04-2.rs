use aoc2023::{read_file_string, Parse, get_first_number, get_data_list};

fn main() {
    println!("Result {}", solve(read_file_string("inputs/04.txt").unwrap()));
}

fn solve(data: String) -> i32 {
    let cards = get_data_list(data);
    copies_of_cards(cards)
}

fn copies_of_cards(cards: Vec<Card>) -> i32 {
    let mut sum_of_copies = 0;
    for c in cards.clone() {
        sum_of_copies += get_card_sum(&c, &cards);
    }
    sum_of_copies
}

fn get_card_sum(c: &Card, cards: &Vec<Card>) -> i32{
    let mut sum = 1;
    for i in c.id..c.id+c.wins {
        sum += get_card_sum(&cards[i as usize], &cards);
    }
    sum
}

#[derive(Debug, Clone)]
struct Card {
    id: i32,
    wins: i32,
}

impl Parse for Card {
    fn parse(line: &str) -> Self {
        let line = line.replace("Card", "").trim().to_string();
        let id = get_first_number(&line);
        let line = line[3..].trim().to_string();
        let data = line.split("|").into_iter();
        let winnings = data.clone().into_iter().nth(0).unwrap().trim();
        let numbers = data.into_iter().nth(1).unwrap().trim();
        let winnings = winnings.replace(":", "").trim().to_string();
    
        let mut winning = Vec::new();
        for w in winnings.split(" ").filter(|c| !c.is_empty()).into_iter() {
            winning.push(w.parse().unwrap())
        }
        let mut nums = Vec::new();
        for n in numbers.split(" ").filter(|c| !c.is_empty()).into_iter() {
            nums.push(n.parse().unwrap());
        }
        let wins = get_wins(winning, nums);
        Self { id, wins }
    }
}

fn get_wins(winning: Vec<i32>, nums: Vec<i32>) -> i32 {
    let mut matches = 0;
    for n in nums {
        if winning.contains(&n) {
            matches += 1;
        }
    }
    matches
}

#[test]
fn test_04_2() {
    let input = String::from("Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11");
    assert_eq!(30, solve(input));
}