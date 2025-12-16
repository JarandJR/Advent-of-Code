use common::io::parse_into_lines_automatic;

fn main() {
    dbg!(parse_and_solve_1(parse_into_lines_automatic("2")));
    dbg!(parse_and_solve_2(parse_into_lines_automatic("2")));
}

fn parse_and_solve_1(line_iter: impl Iterator<Item = String>) -> usize {
    line_iter.fold(0, |acc, line| {
        acc + line.split(",").fold(0, |acc, range| {
            let (left, right) = range
                .split_once("-")
                .map(|(l, r)| {
                    let l = l.parse::<usize>().unwrap();
                    let r = r.parse::<usize>().unwrap();
                    (l, r)
                })
                .unwrap();
            acc + (left..=right).fold(0, |acc, n| {
                let n_str = n.to_string();
                let split = n_str.len().div_ceil(2);
                if n_str[..split].eq(&n_str[split..]) {
                    return acc + n;
                }
                acc
            })
        })
    })
}

fn parse_and_solve_2(line_iter: impl Iterator<Item = String>) -> usize {
    line_iter.fold(0, |acc, line| {
        acc + line.split(",").fold(0, |acc, range| {
            let (left, right) = range
                .split_once("-")
                .map(|(l, r)| {
                    let l = l.parse::<usize>().unwrap();
                    let r = r.parse::<usize>().unwrap();
                    (l, r)
                })
                .unwrap();

            acc + (left..=right).fold(0, |acc, n| {
                let n_str = n.to_string();
                let len = n_str.len();
                if (1..=len / 2).any(|divisor| {
                    let pattern = &n_str[..divisor];
                    pattern.repeat(len / divisor).eq(&n_str)
                }) {
                    return acc + n;
                }
                acc
            })
        })
    })
}

#[test]
fn day_2_1() {
    let input = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124";
    let result = parse_and_solve_1(input.lines().map(|s| s.to_owned()));
    assert_eq!(result, 1227775554);
}

#[test]
fn day_2_2() {
    let input = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124";
    let result = parse_and_solve_2(input.lines().map(|s| s.to_owned()));
    assert_eq!(result, 4174379265);
}
