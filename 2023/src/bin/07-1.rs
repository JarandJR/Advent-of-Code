use aoc2023::{get_data_list, get_numbers_on_line, read_file_string, Parse};

fn main() {
    println!(
        "Result: {}",
        solve(read_file_string("inputs/07.txt").unwrap())
    );
}

fn solve(data: String) -> usize {
    let mut hands: Vec<PokerHand> = get_data_list(data);
    hands.sort_by(
        |a, b| match a.poker_type.partial_cmp(&b.poker_type).unwrap() {
            std::cmp::Ordering::Equal => highest_first_cards(&a.cards, &b.cards),
            a => a,
        },
    );
    hands
        .iter()
        .enumerate()
        .map(|(i, h)| h.bid * (i + 1))
        .fold(0, |a, b| a + b)
}

fn highest_first_cards(a: &Vec<i32>, b: &Vec<i32>) -> std::cmp::Ordering {
    for (a, b) in a.iter().zip(b.iter()) {
        match a.partial_cmp(&b).unwrap() {
            std::cmp::Ordering::Equal => continue,
            a => return a,
        }
    }
    std::cmp::Ordering::Equal
}

#[derive(Debug, PartialEq, PartialOrd, Ord, Eq)]
enum PokerType {
    HighCard = 1,
    OnePair = 2,
    TwoPairs = 3,
    ThreeOfAKind = 4,
    FullHouse = 5,
    FourOfAKind = 6,
    FiveOfAKind = 7,
}

#[derive(Debug)]
struct PokerHand {
    cards: Vec<i32>,
    bid: usize,
    poker_type: PokerType,
}

impl Parse for PokerHand {
    fn parse(line: &str) -> Self {
        let mut it = line.split(" ").into_iter();
        let mut l = String::new();
        for c in it.next().unwrap().chars() {
            if c.is_numeric() {
                l.push_str(&format!("{},", c));
            } else {
                l.push_str(&format!("{},", get_card_number_string(c)));
            }
        }
        l.push_str(it.next().unwrap());
        let numbers = get_numbers_on_line(l.as_str());
        let cards = numbers[..numbers.len() - 1].to_vec();
        let bid = *numbers.last().unwrap() as usize;
        let poker_type = get_poker_type(&cards);
        PokerHand {
            cards,
            bid,
            poker_type: poker_type,
        }
    }
}

fn get_poker_type(cards: &Vec<i32>) -> PokerType {
    let counts = number_of_a_kind(cards);
    let highest_count = counts.last().unwrap();
    match highest_count {
        5 => PokerType::FiveOfAKind,
        4 => PokerType::FourOfAKind,
        3 => {
            let lowest_count = counts.first().unwrap();
            match lowest_count {
                2 => PokerType::FullHouse,
                _ => PokerType::ThreeOfAKind,
            }
        }
        2 => match counts.iter().filter(|&x| *x == 1).count() {
            1 => PokerType::TwoPairs,
            _ => PokerType::OnePair,
        },
        _ => PokerType::HighCard,
    }
}

fn number_of_a_kind(cards: &Vec<i32>) -> Vec<i32> {
    let mut counts = Vec::new();
    for c in cards {
        let count = cards.iter().filter(|&x| x == c).count() as i32;
        counts.push(count);
    }
    counts.sort();
    counts
}

fn get_card_number_string(c: char) -> String {
    match c {
        'T' => "10".to_string(),
        'J' => "11".to_string(),
        'Q' => "12".to_string(),
        'K' => "13".to_string(),
        'A' => "14".to_string(),
        _ => c.to_string(),
    }
}

#[test]
fn test_07_1() {
    assert_eq!(
        6440,
        solve(
            "32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483"
                .to_string()
        )
    );
}
