use aoc2023::read_file_string;

fn main() {
    println!("Result {}", solve(read_file_string("inputs/02.txt").unwrap()));
}

fn solve(data: String) -> i32 {
    data.lines().into_iter()
    .map(|l| operation(&l))
    .fold(0, |a, b| a + b)
}

fn operation(i: &str) -> i32 {
    let mut red = 1;
    let mut blue = 1;
    let mut green = 1;

    let data = i.trim()[8..].trim().split(" ").collect::<Vec<&str>>();
    let mut counter = 0;
    for _ in data.clone().iter() {
        let mut num = get_data(data[counter]);
        if num.is_empty() {
            counter += 1;
            num = get_data(data[counter]);
        }
        let num: i32 = num.parse().unwrap();
        let color = get_color(&get_data(data[counter+1])).unwrap();

        match color {
            Color::Blue => blue = get_color_count(blue, num),
            Color::Red => red = get_color_count(red, num),
            Color::Green => green = get_color_count(green, num),
        }
        counter+=2;
        if counter >= data.len() {
            break;
        }
    }
    red * blue * green
}

fn get_color_count(current: i32, new: i32) -> i32 {
    if current < new {
        return new;
    }
    current
}

fn get_data(data: &str) -> String {
    data.replace(";", "").replace(",", "").replace(":", "")
}

fn get_color(c: &str) -> Option<Color> {
    match c {
        "blue" => Some(Color::Blue),
        "red" => Some(Color::Red),
        "green" => Some(Color::Green),
        _ => None
    }
}

enum Color {
    Blue,
    Red,
    Green,
}

#[test]
fn test_02_1() {
    let input = String::from("Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
    Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
    Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
    Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
    Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green");
    assert_eq!(2286, solve(input));
}
