use std::ops::RangeInclusive;

use common::io::parse_into_lines_automatic;

fn main() {
    dbg!(parse_and_solve_1(parse_into_lines_automatic("5")));
    dbg!(parse_and_solve_2(parse_into_lines_automatic("5")));
}

fn parse_and_solve_1(mut line_iter: impl Iterator<Item = String>) -> usize {
    let ranges = get_unique_ranges(&mut line_iter);
    line_iter
        .filter(|line| {
            ranges
                .iter()
                .any(|range| range.contains(&line.parse::<usize>().unwrap()))
        })
        .count()
}

fn parse_and_solve_2(mut line_iter: impl Iterator<Item = String>) -> usize {
    let ranges = get_unique_ranges(&mut line_iter);
    ranges.into_iter().flat_map(|r| r).count()
}

fn get_unique_ranges(line_iter: &mut impl Iterator<Item = String>) -> Vec<RangeInclusive<usize>> {
    let mut fresh_id_ranges = line_iter
        .by_ref()
        .take_while(|line| !line.is_empty())
        .map(|line| {
            let (start, end) = line
                .split_once("-")
                .map(|(l, r)| {
                    let l = l.parse::<usize>().unwrap();
                    let r = r.parse::<usize>().unwrap();
                    (l, r)
                })
                .unwrap();
            start..=end
        })
        .collect::<Vec<RangeInclusive<usize>>>();
    fresh_id_ranges.sort_by_key(|r| *r.start());
    let mut unique_ranges: Vec<RangeInclusive<usize>> = Vec::new();
    for range in fresh_id_ranges {
        if let Some(last) = unique_ranges.last_mut() {
            let curnt_end = *last.end();
            let end = *range.end().max(&curnt_end);
            if *range.start() <= curnt_end + 1 {
                let start = *last.start();
                *last = start..=end;
            } else {
                unique_ranges.push(range);
            }
        } else {
            unique_ranges.push(range);
        }
    }
    unique_ranges
}

#[test]
fn day5_1() {
    let input = "3-5
10-14
16-20
12-18

1
5
8
11
17
32";
    let result = parse_and_solve_1(input.lines().map(|s| s.to_owned()));
    assert_eq!(result, 3);
}

#[test]
fn day5_2() {
    let input = "3-5
10-14
16-20
12-18

1
5
8
11
17
32";
    let result = parse_and_solve_2(input.lines().map(|s| s.to_owned()));
    assert_eq!(result, 14);
}
