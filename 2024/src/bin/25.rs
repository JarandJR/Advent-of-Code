use common::io::parse_into_lines_automatic;

fn main() {
    dbg!(parse_and_solve(parse_into_lines_automatic("25")));
}

fn parse_and_solve(line_iter: impl Iterator<Item = String>) -> usize {
    0
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
