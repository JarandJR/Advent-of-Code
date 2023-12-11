use aoc2023::read_file_string;

fn main() {
    println!("Result {}", solve(read_file_string("inputs/03.txt").unwrap()));
}

fn solve(data: String) -> i32 {
    let mut it= data.lines().into_iter();
    let mut prev = it.next();
    let mut at = it.next();
    let mut next = it.next();

    let mut sum = check_line(None, prev.unwrap(), at);
    while at.is_some() {
        sum += check_line(prev, at.unwrap(), next);
        prev = at;
        at = next;
        next = it.next();
    }
    sum
}

fn check_line(p: Option<&str>, at: &str, n: Option<&str>) -> i32 {
    let mut sum = 0;
    let mut num = Vec::new();
    for (i, c) in at.chars().enumerate() {
        if c.is_numeric() {
            num.push(c);
            if i == at.len() - 1 {
                if check_num_is_part(p, at.chars(), n, num.len(), i) {
                    sum += get_number(&num);
                }
            }
        }
        else {
            if num.is_empty() {
                continue;
            }
            if check_num_is_part(p, at.chars(), n, num.len(), i) {
                sum += get_number(&num);
            }
            num.clear();
        }
    }
    sum
}

fn check_num_is_part(p: Option<&str>, a: core::str::Chars, n: Option<&str>, len: usize, end: usize) -> bool {
    let mut is_part = false;
    let mut start = end - len;
    if start != 0 {
        start -= 1;
    }
    for i in start..=end {
        if p.is_some() {
            if is_symbol(p.unwrap().chars().into_iter().nth(i).unwrap()) {
                is_part = true;
            }
        }
        if n.is_some() {
            if is_symbol(n.unwrap().chars().into_iter().nth(i).unwrap()) {
                is_part = true;
            }
        }
        let at = is_symbol(a.clone().into_iter().nth(i).unwrap());
        if at {
            is_part = true;
        }
    }
    is_part
}

fn is_symbol(c: char) -> bool {
    !c.is_numeric() && c != '.'
}

fn get_number(chars: &Vec<char>) -> i32 {
    let mut res = String::new();
    let mut it = chars.iter();
    let mut at = it.next();
    while at.is_some() {
        res.push(*at.unwrap());
        at = it.next();
    }
    res.parse().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_03_1() {
        let input = String::from("467..114..
    ...*......
    ..35..633.
    ......#...
    617*......
    .....+.58.
    ..592.....
    ......755.
    ...$.*....
    .664.598..");
        assert_eq!(4361, solve(input));
    }
}
