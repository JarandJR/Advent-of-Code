use aoc2023::read_file_string;


fn main() {
    println!("Result {}", solve(read_file_string("inputs/01-2.txt").unwrap()));
}

fn solve(data: String) -> i32 {
    data.lines().into_iter()
    .map(|l| string_num_to_char(l))
    .map(|l| operation(&l))
    .fold(0, |a, b| a + b)
}

fn operation(i: &str) -> i32 {
    let nums = i.chars().filter(|c| c.is_numeric()).collect::<Vec<char>>();
    let first = nums.first().unwrap();
    concat_chars_to_num(first, nums.last().unwrap_or(first))
}

fn string_num_to_char(s: &str) -> String {
    let str_nums = vec!["one", "two", "three", "four", "five", "six", "seven", "eight", "nine"];
    let mut res = String::new();
    
    let mut str_nums_filtered = str_nums.clone();
    for (i, c) in s.chars().enumerate() {
        if c.is_numeric(){
            res.push(c);
            continue;
        }
        if c.is_whitespace() {
            continue;
        }
        str_nums_filtered  = str_nums_filtered
        .iter()
        .filter(|n| n.chars().nth(0).unwrap() == c)
        .map(|c| *c)
        .collect::<Vec<&str>>();

        for j in 0..str_nums_filtered.len(){
            let num_str = str_nums_filtered[j];
            let at = i;
            let to = i+num_str.len();
            if num_str.len() > s[at..].len() {
                continue;
            }
            if num_str == &s[at..to] {
                let number = str_nums.iter().position(|&num| num==num_str).unwrap()+1;
                res.push(number.to_string().chars().nth(0).unwrap());
                str_nums_filtered = str_nums.clone();
            }
        }
        str_nums_filtered = str_nums.clone();
    }
    res
}

fn concat_chars_to_num(a: &char, b: &char) -> i32 {
    format!("{}{}", a, b).parse().unwrap()
}

#[test]
fn test_01_2() {
    let input = String::from("two1nine
    eightwothree
    abcone2threexyz
    xtwone3four
    4nineeightseven2
    zoneight234
    7pqrstsixteen");
    assert_eq!(281, solve(input));
}
