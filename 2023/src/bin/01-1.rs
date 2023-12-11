use aoc2023::read_file_string;

fn main() {
    println!("Result {}", solve(read_file_string("inputs/01-1.txt").unwrap()));
}

fn solve(data: String) -> i32 {
    data.lines().into_iter().map(|l| operation(l))
    .fold(0, |a, b| a + b)
}

fn operation(i: &str) -> i32 {
    let nums = i.chars().filter(|c| c.is_numeric()).collect::<Vec<char>>();
    let first = nums.first().unwrap();
    concat_chars_to_num(first, nums.last().unwrap_or(first))
}

fn concat_chars_to_num(a: &char, b: &char) -> i32 {
    format!("{}{}", a, b).parse().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_01_1() {
        let input = String::from("1abc2
        pqr3stu8vwx
        a1b2c3d4e5f
        treb7uchet");
        assert_eq!(142, solve(input));
    }
}
