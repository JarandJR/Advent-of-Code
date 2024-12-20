use common::io::parse_into_lines_automatic;

fn main() {
    dbg!(parse_and_solve(parse_into_lines_automatic("07"), false));
    dbg!(parse_and_solve(parse_into_lines_automatic("07"), true));
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

fn parse_and_solve(line_iter: impl Iterator<Item = String>, concat: bool) -> usize {
    line_iter.fold(0, |acc, line| {
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
    })
}

#[test]
fn day07_1() {
    let input = "190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20";
    let result = parse_and_solve(input.lines().map(|s| s.to_owned()), false);
    assert_eq!(result, 3749);
}

#[test]
fn day07_2() {
    let input = "190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20";
    let result = parse_and_solve(input.lines().map(|s| s.to_owned()), true);
    assert_eq!(result, 11387);
}
