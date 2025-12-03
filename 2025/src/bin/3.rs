use common::io::parse_into_lines_automatic;

fn main() {
    dbg!(parse_and_solve_1(parse_into_lines_automatic("3")));
    dbg!(parse_and_solve_2(parse_into_lines_automatic("3")));
}

fn parse_and_solve_1(line_iter: impl Iterator<Item = String>) -> usize {
    line_iter.fold(0, |acc, line| {
        let mut max_left = 0;
        let mut max_right = 0;
        line.chars().enumerate().for_each(|(pos, s)| {
            let n = s.to_string().parse::<u32>().unwrap();
            if max_left < n && pos < line.len() - 1 {
                max_left = n;
                max_right = 0;
            } else if max_right < n && pos > 0 {
                max_right = n;
            }
        });
        acc + format!("{}{}", max_left, max_right)
            .parse::<usize>()
            .unwrap()
    })
}

fn parse_and_solve_2(line_iter: impl Iterator<Item = String>) -> usize {
    line_iter.fold(0, |acc, line| {
        let len = line.len();
        let mut max_joltage = 0;
        let mut potential_starts = Vec::new();
        let batteries = line
            .chars()
            .enumerate()
            .map(|(pos, c)| {
                let n = c.to_digit(10).unwrap() as usize;
                if len - pos >= 12 {
                    if n > max_joltage {
                        max_joltage = n;
                        potential_starts = vec![pos];
                    } else if n == max_joltage {
                        potential_starts.push(pos);
                    }
                }
                n
            })
            .collect::<Vec<usize>>();
        let max_joltage = potential_starts
            .iter()
            .map(|pos| {
                let mut max = 0;
                max_k_ordered_permutations(&batteries, 12, *pos, 0, 0, &mut max);
                max
            })
            .max()
            .unwrap();
        acc + max_joltage
    })
}

fn max_k_ordered_permutations(
    batteries: &[usize],
    k: usize,
    start: usize,
    depth: usize,
    joltage: usize,
    max: &mut usize,
) {
    // Prune branches
    let remaining = k - depth;
    if batteries.len() - start < remaining {
        return;
    }
    let shift_factor = 10_usize.pow(remaining as u32);
    let max_potential = joltage * shift_factor + shift_factor - 1;
    if max_potential <= *max {
        return;
    }
    if depth == k {
        if *max < joltage {
            *max = joltage;
        }
        return;
    }
    for i in start..batteries.len() {
        let next_joltage = joltage * 10 + batteries[i];
        max_k_ordered_permutations(batteries, k, i + 1, depth + 1, next_joltage, max);
    }
}

#[test]
fn day3_1_1() {
    let input = "987654321111111
811111111111119
234234234234278
818181911112111";
    let result = parse_and_solve_1(input.lines().map(|s| s.to_owned()));
    assert_eq!(result, 357);
}

#[test]
fn day3_1_2() {
    let input = "987654321111111
811111111111119
234234234234278
818181911112111";
    let result = parse_and_solve_2(input.lines().map(|s| s.to_owned()));
    assert_eq!(result, 3121910778619);
}
