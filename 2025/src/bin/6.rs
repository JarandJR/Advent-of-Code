use common::io::parse_into_lines_automatic;

fn main() {
    dbg!(parse_and_solve_1(parse_into_lines_automatic("6")));
    dbg!(parse_and_solve_2(parse_into_lines_automatic("6")));
}

fn parse_and_solve_1(line_iter: impl Iterator<Item = String>) -> usize {
    let mut ops = Vec::new();
    let nums = line_iter
        .map(|line| {
            line.split_whitespace()
                .flat_map(|c| match c.parse::<usize>() {
                    Ok(n) => Some(n),
                    Err(_) => {
                        ops.push(c.to_owned());
                        None
                    }
                })
                .collect::<Vec<usize>>()
        })
        .collect::<Vec<Vec<usize>>>();
    let mut sum = nums[0].clone();
    for row_data in &nums[1..] {
        for (col, num) in row_data.iter().enumerate() {
            match ops[col].as_str() {
                "*" => sum[col] *= num,
                "+" => sum[col] += num,
                _ => unreachable!(),
            }
        }
    }
    sum.iter().sum()
}

fn parse_and_solve_2(line_iter: impl Iterator<Item = String>) -> usize {
    let mut lines = line_iter.collect::<Vec<String>>();
    let ops = lines.pop().unwrap();
    let ops = ops.split_whitespace().collect::<Vec<&str>>();
    let mut column_width = vec![0; ops.len()];
    for line in lines.iter() {
        line.split_whitespace()
            .map(|str| str.len())
            .enumerate()
            .for_each(|(col, width)| {
                column_width[col] = column_width[col].max(width);
            });
    }
    let mut sum = ops
        .iter()
        .map(|o| match *o {
            "*" => 1,
            "+" => 0,
            _ => unreachable!(),
        })
        .collect::<Vec<usize>>();
    let mut columns = vec![Vec::new(); ops.len()];
    for line in lines.iter() {
        let mut start = 0;
        for (col_idx, width) in column_width.iter().enumerate() {
            let value = &line[start..start + *width];
            columns[col_idx].push(value);
            start += *width + 1;
        }
    }
    for (col_idx, op) in ops.iter().enumerate() {
        let width = column_width[col_idx];
        for at in (0..width).rev() {
            let mut num = String::new();
            for col_val in &columns[col_idx] {
                num.push(col_val.chars().nth(at).unwrap());
            }
            let num = num.trim().parse::<usize>().unwrap();
            match *op {
                "*" => sum[col_idx] *= num,
                "+" => sum[col_idx] += num,
                _ => unreachable!(),
            }
        }
    }
    sum.iter().sum()
}

#[test]
fn day6_1() {
    let input = "123 328  51 64 
 45 64  387 23 
  6 98  215 314
*   +   *   +  ";
    let result = parse_and_solve_1(input.lines().map(|s| s.to_owned()));
    assert_eq!(result, 4277556);
}

#[test]
fn day6_2() {
    let input = "123 328  51 64 
 45 64  387 23 
  6 98  215 314
*   +   *   +  ";
    let result = parse_and_solve_2(input.lines().map(|s| s.to_owned()));
    assert_eq!(result, 3263827);
}
