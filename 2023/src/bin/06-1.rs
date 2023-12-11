use aoc2023::{read_file_string, get_numbers_on_line};

fn main() {
    println!("Result {}", solve(read_file_string("inputs/06.txt").unwrap()));
}

fn solve(data: String) -> i32 {
    let games: Vec<Game> = parse(&data);
    get_multiply_of_wins(&games)
}

fn get_multiply_of_wins(games: &Vec<Game>) -> i32 {
    let mut sum = 0;
    for g in games {
        let mut wins = 0;
        for hold in 1..g.time {
            let time_left = g.time - hold;
            if time_left * hold > g.record {
                wins += 1;
            }
        }
        if sum == 0 {
            sum = wins;
        } else {
            sum *= wins;
        }
    }
    sum
}

#[derive(Debug, Clone)]
struct Game {
    time: i32,
    record: i32,
}

fn parse(data: &str) -> Vec<Game> {
    let mut it = data.lines().into_iter();
    let times = get_numbers_on_line(it.next().unwrap());
    let records = get_numbers_on_line(it.next().unwrap());
    let mut games = Vec::new();

    for i in 0..times.len() {
        games.push(Game { time: times[i], record: records[i] });
    }
    games
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_06_1() {
        let input = String::from("Time:      7  15   30
    Distance:  9  40  200");
        assert_eq!(288, solve(input));
    }
}
