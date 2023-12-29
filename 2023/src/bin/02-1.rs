use aoc2023::read_file_string;

fn main() {
    println!("Result {}", solve(read_file_string("inputs/02.txt").unwrap()));
}

fn solve(data: String) -> i32 {
    data.lines().into_iter()
    .map(|l| operation(&l))
    .filter(|g| g.valid)
    .map(|g| g.id)
    .fold(0, |a, b| a + b)
}

fn operation(i: &str) -> Game {
    let red = 12;
    let green = 13;
    let blue = 14;
    let id: i32 = i.trim()[5..8].replace(":", "").trim().parse().unwrap();
    let data = i.trim()[8..].trim().split(" ").collect::<Vec<&str>>();
    let mut draws = Vec::new();
    let mut counter = 0;
    for _ in data.clone().iter() {
        let mut num = get_data(data[counter]);
        if num.is_empty() {
            counter += 1;
            num = get_data(data[counter]);
        }
        let num: i32 = num.parse().unwrap();
        let color = get_color(&get_data(data[counter+1]), num);
        draws.push(color);
        counter+=2;
        if counter >= data.len() {
            break;
        }
    }
    let mut valid = true;
    for c in draws {
        if c.is_some() {
            let not_valid = match c.unwrap() {
                Color::Blue(val) => val > blue,
                Color::Green(val) => val > green,
                Color::Red(val) => val > red,
            };
            if not_valid {
                valid = false;
            }
        }
    }
    Game { id, valid }
}

fn get_data(data: &str) -> String {
    data.replace(";", "").replace(",", "").replace(":", "")
}

fn get_color(c: &str, n: i32) -> Option<Color> {
    match c {
        "blue" => Some(Color::Blue(n)),
        "red" => Some(Color::Red(n)),
        "green" => Some(Color::Green(n)),
        _ => None
    }
}

struct Game {
    id: i32,
    valid: bool,
}

enum Color {
    Blue(i32),
    Red(i32),
    Green(i32),
}

#[test]
fn test_02_1() {
    let input = String::from("Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
    Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
    Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
    Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
    Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green");
    assert_eq!(8, solve(input));
}
