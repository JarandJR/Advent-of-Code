use common::io::parse_into_lines_automatic;
use itertools::Itertools;

fn main() {
    dbg!(parse_and_solve(parse_into_lines_automatic("25")));
}

fn parse_and_solve(mut line_iter: impl Iterator<Item = String>) -> usize {
    let mut keys = Vec::new();
    let mut locks = Vec::new();
    let mut width = 0;
    loop {
        let lines = line_iter
            .by_ref()
            .take_while(|l| !l.is_empty())
            .collect_vec();
        if lines.is_empty() {
            break;
        }
        width = lines[0].len();
        let key_lock = lines[1..=width]
            .iter()
            .fold(vec![0; width], |mut acc, line| {
                line.chars().enumerate().for_each(|(i, c)| {
                    if c == '#' {
                        acc[i] += 1;
                    }
                });
                acc
            });
        if lines[0].chars().all(|c| c == '#') {
            locks.push(key_lock);
        } else {
            keys.push(key_lock);
        }
    }
    keys.iter().fold(0, |acc, k| {
        acc + locks.iter().fold(0, |acc, l| {
            let overlap = l.iter().zip(k).map(|(l, k)| k + l).any(|s| s > width);
            if !overlap {
                return acc + 1;
            }
            acc
        })
    })
}

#[test]
fn day25_1() {
    let input = "#####
.####
.####
.####
.#.#.
.#...
.....

#####
##.##
.#.##
...##
...#.
...#.
.....

.....
#....
#....
#...#
#.#.#
#.###
#####

.....
.....
#.#..
###..
###.#
###.#
#####

.....
.....
.....
#....
#.#..
#.#.#
#####";
    let result = parse_and_solve(input.lines().map(|s| s.to_owned()));
    assert_eq!(result, 3);
}
