use aoc2024::parse_into_lines_automatic;

fn main() {
    dbg!(parse_and_solve("07", false));
    dbg!(parse_and_solve("07", true));
}

fn permutations(numbers: &[usize], concat: bool) -> Vec<usize> {
    let mut results = Vec::new();
    permutations_rec(&numbers[1..], &mut results, numbers[0], concat);
    results
}

fn permutations_rec(nums: &[usize], res: &mut Vec<usize>, curnt_res: usize, concat: bool) {
    if nums.is_empty() {
        res.push(curnt_res);
        return;
    }
    permutations_rec(&nums[1..], res, curnt_res + nums[0], concat);
    permutations_rec(&nums[1..], res, curnt_res * nums[0], concat);
    if concat {
        let concatenated = [curnt_res, nums[0]]
            .iter()
            .fold(String::new(), |acc, n| acc + &n.to_string())
            .parse::<usize>()
            .unwrap();
        permutations_rec(&nums[1..], res, concatenated, concat);
    }
}

fn parse_and_solve(day: &str, concat: bool) -> usize {
    if let Some(line_iter) = parse_into_lines_automatic(day) {
        return line_iter.fold(0, |acc, line| {
            let mut split = line.split(':');
            let result = split.next().unwrap().trim().parse::<usize>().unwrap();
            let numbers = split
                .next()
                .unwrap()
                .split_ascii_whitespace()
                .filter_map(|num| num.parse::<usize>().ok())
                .collect::<Vec<_>>();
            if permutations(&numbers, concat).contains(&result) {
                return acc + result;
            }
            acc
        });
    }
    0
}

#[test]
fn day07_1() {
    use std::fs::{remove_file, File};
    let file_name = "test_07_1";
    let file_path = format!("inputs/{}.txt", file_name);
    {
        // Setup for test
        use std::io::Write;
        let mut file = File::create(&file_path).expect("Could not create file");
        writeln!(
            file,
            "190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20"
        )
        .expect("Could not write to file");
    }
    let result = parse_and_solve(&file_name, false);
    // Clean up
    remove_file(file_path).expect("Could not remove file");
    assert_eq!(result, 3749);
}

#[test]
fn day07_2() {
    use std::fs::{remove_file, File};
    let file_name = "test_07_2";
    let file_path = format!("inputs/{}.txt", file_name);
    {
        // Setup for test
        use std::io::Write;
        let mut file = File::create(&file_path).expect("Could not create file");
        writeln!(
            file,
            "190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20"
        )
        .expect("Could not write to file");
    }
    let result = parse_and_solve(&file_name, true);
    // Clean up
    remove_file(file_path).expect("Could not remove file");
    assert_eq!(result, 11387);
}
