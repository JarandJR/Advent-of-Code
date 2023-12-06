use aoc2023::{read_file_string, parse_line, Parse, get_numbers_on_line};

fn main() {
    println!("Result {}", solve(read_file_string("inputs/06.txt").unwrap()));
}

fn solve(data: String) -> i64 {
    let game: Game = parse_line(&data);
    let mut wins = 0;
    for time_held in 1..game.time {
        let time_left = game.time - time_held;
        if time_held * time_left > game.record{
            wins += 1;
        }
    }
    wins
}

struct Game {
    time: i64,
    record: i64,
}

impl Parse for Game {
    fn parse(line: &str) -> Self {
        let mut it = line.lines().into_iter();
        let times_list = get_numbers_on_line(it.next().unwrap());
        let records_list = get_numbers_on_line(it.next().unwrap());
        Self { time: concat(times_list), record: concat(records_list) }
    }
}

fn concat(list: Vec<i64>) -> i64 {
    let mut res = String::new();
    for i in list {
        i.to_string().chars().into_iter().for_each(|c| res.push(c));
    }
    res.parse().unwrap()
}

#[test]
fn test_06_2() {
    assert_eq!(71503, solve("Time:      7  15   30
    Distance:  9  40  200".to_string()));
}
