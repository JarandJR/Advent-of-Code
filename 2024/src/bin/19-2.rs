use std::collections::HashMap;

use common::io::parse_into_lines_automatic;

fn main() {
    dbg!(parse_and_solve("19"));
}

fn parse_and_solve(day: &str) -> usize {
    if let Some(mut line_iter) = parse_into_lines_automatic(day) {
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
        let mut permutations = HashMap::new();
        return wanted_designs.iter().fold(0, |acc, wanted| {
            acc + dfs_sum(&wanted, &patterns, &mut permutations)
        });
    }
    panic!("Failed to read input file")
}

fn dfs_sum(curnt: &str, patterns: &[String], processed: &mut HashMap<String, usize>) -> usize {
    if let Some(processed) = processed.get(curnt) {
        return *processed;
    }
    if curnt.is_empty() {
        return 1;
    }
    let mut sum = 0;
    for p in patterns {
        if let Some(stripped) = curnt.strip_prefix(p) {
            sum += dfs_sum(stripped, patterns, processed);
        }
    }
    processed.insert(curnt.to_string(), sum);
    sum
}

#[test]
fn day19_2() {
    use std::fs::{remove_file, File};
    let file_name = "test_19_2";
    let file_path = format!("inputs/{}.txt", file_name);
    {
        // Setup for test
        use std::io::Write;
        let mut file = File::create(&file_path).expect("Could not create file");
        writeln!(
            file,
            "r, wr, b, g, bwu, rb, gb, br

brwrr
bggr
gbbr
rrbgbr
ubwu
bwurrg
brgr
bbrgwb"
        )
        .expect("Could not write to file");
    }
    let result = parse_and_solve(&file_name);
    // Clean up
    remove_file(file_path).expect("Could not remove file");
    assert_eq!(result, 16);
}
