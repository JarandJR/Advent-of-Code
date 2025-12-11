use std::collections::HashMap;

use common::{generic_algorithms::dfs::dfs, io::parse_into_lines_automatic};

fn main() {
    dbg!(parse_and_solve(parse_into_lines_automatic("11")));
}

fn parse_and_solve(line_iter: impl Iterator<Item = String>) -> usize {
    let paths = line_iter
        .map(|line| {
            let key = line.chars().take_while(|c| !c.eq(&':')).collect::<String>();
            let paths = line
                .split_whitespace()
                .skip(1)
                .map(|s| s.to_owned())
                .collect::<Vec<String>>();
            (key, paths)
        })
        .collect::<HashMap<String, Vec<String>>>();
    let mut memo = HashMap::new();
    dfs(
        String::from("you"),
        &mut memo,
        &|state| state.eq(&String::from("out")).then(|| 1),
        &|state| paths.get(state).unwrap().to_vec(),
        &|a, b| a + b,
    )
}

#[test]
fn day11_1() {
    let input = "aaa: you hhh
you: bbb ccc
bbb: ddd eee
ccc: ddd eee fff
ddd: ggg
eee: out
fff: out
ggg: out
hhh: ccc fff iii
iii: out";
    let result = parse_and_solve(input.lines().map(|s| s.to_owned()));
    assert_eq!(result, 5);
}
