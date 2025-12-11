use std::collections::HashMap;

use common::{generic_algorithms::dfs::dfs, io::parse_into_lines_automatic};

fn main() {
    dbg!(parse_and_solve_1(parse_into_lines_automatic("11")));
    dbg!(parse_and_solve_2(parse_into_lines_automatic("11")));
}

fn parse_and_solve_1(line_iter: impl Iterator<Item = String>) -> usize {
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
        "you".to_string(),
        &mut memo,
        &|state| state.eq("out").then(|| 1),
        &|state| paths.get(state).unwrap().to_vec(),
        &|a, b| a + b,
    )
}

fn parse_and_solve_2(line_iter: impl Iterator<Item = String>) -> usize {
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
        ("svr".to_string(), false, false),
        &mut memo,
        &|(state, fft, dac)| {
            state
                .as_str()
                .eq("out")
                .then(|| (*fft && *dac).then(|| 1).unwrap_or_else(|| 0))
        },
        &|(state, fft, dac)| {
            paths
                .get(&state.to_string())
                .unwrap()
                .iter()
                .map(|n| {
                    let mut next_fft = *fft;
                    let mut next_dac = *dac;
                    n.eq("fft").then(|| next_fft = true);
                    n.eq("dac").then(|| next_dac = true);
                    (n.clone(), next_fft, next_dac)
                })
                .collect()
        },
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
    let result = parse_and_solve_1(input.lines().map(|s| s.to_owned()));
    assert_eq!(result, 5);
}

#[test]
fn day11_2() {
    let input = "svr: aaa bbb
aaa: fft
fft: ccc
bbb: tty
tty: ccc
ccc: ddd eee
ddd: hub
hub: fff
eee: dac
dac: fff
fff: ggg hhh
ggg: out
hhh: out";
    let result = parse_and_solve_2(input.lines().map(|s| s.to_owned()));
    assert_eq!(result, 2);
}
