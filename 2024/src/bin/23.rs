use common::{datastructs::bidirectional_map::BiMap, io::parse_into_lines_automatic};
use itertools::Itertools;

fn main() {
    dbg!(parse_and_solve(parse_into_lines_automatic("23")));
}

fn parse_and_solve(line_iter: impl Iterator<Item = String>) -> usize {
    let mut map = BiMap::new();
    line_iter.for_each(|line| {
        let (from, to) = line.split('-').collect_tuple().unwrap();
        map.insert(from.to_owned(), to.to_owned());
    });
    let mut count = 0;
    // Needs to create a graph with edges
    for (left, right) in map.forward_iter() {}
    count
}

#[test]
fn day23_1() {
    let input = "kh-tc
qp-kh
de-cg
ka-co
yn-aq
qp-ub
cg-tb
vc-aq
tb-ka
wh-tc
yn-cg
kh-ub
ta-co
de-co
tc-td
tb-wq
wh-td
ta-ka
td-qp
aq-cg
wq-ub
ub-vc
de-ta
wq-aq
wq-vc
wh-yn
ka-de
kh-ta
co-tc
wh-qp
tb-vc
td-yn";
    let result = parse_and_solve(input.lines().map(|s| s.to_owned()));
    assert_eq!(result, 7);
}
