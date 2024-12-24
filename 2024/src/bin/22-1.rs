use common::io::parse_into_lines_automatic;

fn main() {
    dbg!(parse_and_solve(parse_into_lines_automatic("22")));
}

const PRUNE: usize = 16_777_216;

fn parse_and_solve(line_iter: impl Iterator<Item = String>) -> usize {
    line_iter
        .filter_map(|line| line.parse::<usize>().ok())
        .map(|mut num| {
            for _ in 0..2000 {
                num = ((num * 64) ^ num) % PRUNE;
                num = ((num / 32) ^ num) % PRUNE;
                num = ((num * 2048) ^ num) % PRUNE;
            }
            num
        })
        .sum()
}

#[test]
fn day22_1() {
    let input = "1
10
100
2024";
    let result = parse_and_solve(input.lines().map(|s| s.to_owned()));
    assert_eq!(result, 37327623);
}
