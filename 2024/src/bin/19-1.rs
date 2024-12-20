use std::collections::HashMap;

use common::io::parse_into_lines_automatic;

fn main() {
    dbg!(parse_and_solve(parse_into_lines_automatic("19")));
}

fn parse_and_solve(mut line_iter: impl Iterator<Item = String>) -> usize {
    let patterns = line_iter
        .by_ref()
        .take_while(|line| !line.is_empty())
        .flat_map(|s| {
            s.split(&[',', ' '])
                .filter(|s| !s.is_empty())
                .map(|s| s.to_owned())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<String>>();
    let wanted_designs = line_iter.collect::<Vec<_>>();
    let mut processed = HashMap::new();
    wanted_designs
        .iter()
        .filter(|w| dfs(w, &patterns, &mut processed))
        .count()
}

fn dfs(curnt: &str, patterns: &[String], processed: &mut HashMap<String, bool>) -> bool {
    if let Some(processed) = processed.get(curnt) {
        return *processed;
    }
    if curnt.is_empty() {
        return true;
    }
    for p in patterns {
        if let Some(stripped) = curnt.strip_prefix(p) {
            let dfs = dfs(stripped, patterns, processed);
            processed.insert(curnt.to_string(), dfs);
            if dfs {
                return dfs;
            }
        }
    }
    return false;
}

#[test]
fn day19_1() {
    let input = "r, wr, b, g, bwu, rb, gb, br

brwrr
bggr
gbbr
rrbgbr
ubwu
bwurrg
brgr
bbrgwb";
    let result = parse_and_solve(input.lines().map(|s| s.to_owned()));
    assert_eq!(result, 6);
}
