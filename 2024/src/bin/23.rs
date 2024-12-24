use common::io::parse_into_lines_automatic;

fn main() {
    dbg!(parse_and_solve(parse_into_lines_automatic("23")));
}

fn parse_and_solve(line_iter: impl Iterator<Item = String>) -> usize {
    0
}

#[test]
fn day23_1() {
    let input = "aq,cg,yn
aq,vc,wq
co,de,ka
co,de,ta
co,ka,ta
de,ka,ta
kh,qp,ub
qp,td,wh
tb,vc,wq
tc,td,wh
td,wh,yn
ub,vc,wq";
    let result = parse_and_solve(input.lines().map(|s| s.to_owned()));
    assert_eq!(result, 7);
}
