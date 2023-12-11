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
    for (i, c) in at.chars().enumerate() {
        if c == '*' {
            if  let Some(pair) = is_part(p, at, n, i-1, i + 1) {
                sum += pair[0] * pair[1];                    
            } 
        }
    }
    sum
}

fn is_part(p: Option<&str>, a: &str, n: Option<&str>, start: usize, end: usize) -> Option<Vec<i32>> {
    let mut pair: Vec<i32> = Vec::with_capacity(2);
 
    for i in start..=end {
        if p.is_some() {
            if p.unwrap().chars().into_iter().nth(i).unwrap().is_numeric() {
                let num = get_number(&p.unwrap(), i);
                if !pair.contains(&num) {
                    pair.push(num);
                }
            }
        }
        if p.is_some() {
            if n.unwrap().chars().into_iter().nth(i).unwrap().is_numeric() {
                let num = get_number(&n.unwrap(), i);
                if !pair.contains(&num) {
                    pair.push(num);
                }
            }
        }
        if a.chars().into_iter().nth(i).unwrap().is_numeric() {
            let num = get_number(&a, i);
                if !pair.contains(&num) {
                    pair.push(num);
                }
        }
    }
    if pair.len() < 2 {
        return None;
    }
    return Some(pair);
}

fn get_number(l: &str, i: usize) -> i32 {
    let mut i = i;
    let mut at = l.chars().into_iter().nth(i).unwrap();

    while i != 0 {
        if !l.chars().into_iter().nth(i - 1).unwrap().is_numeric() {
            break;
        }
        i -= 1;
        at = l.chars().into_iter().nth(i).unwrap();
    } 

    let mut res = String::new();
    while at.is_numeric() {
        res.push(at);
        if i == l.len() - 1 {
            break;
        }
        i += 1;
        at = l.chars().into_iter().nth(i).unwrap();
    }
    res.parse().unwrap()
}

#[test]
fn test_03_2() {
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
    assert_eq!(467835, solve(input));
}