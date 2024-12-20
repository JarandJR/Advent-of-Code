use common::io::parse_into_lines_automatic;
use itertools::Itertools;

fn main() {
    dbg!(parse_and_solve(parse_into_lines_automatic("14"), 101, 103));
}

fn parse_and_solve(line_iter: impl Iterator<Item = String>, width: usize, hight: usize) -> usize {
    line_iter
        .filter_map(|line| {
            let (x, y, dx, dy) = line
                .split(&[' ', ',', '=', 'p', 'v'])
                .filter_map(|str| {
                    if str.is_empty() {
                        return None;
                    }
                    Some(str)
                })
                .filter_map(|str| str.parse::<i32>().ok())
                .collect_tuple::<(i32, i32, i32, i32)>()
                .unwrap();

            let x = if dx < 0 {
                (x + 1..width as i32)
                    .chain(0..=x)
                    .rev()
                    .cycle()
                    .step_by(dx.abs() as usize)
                    .take(101)
                    .last()
                    .unwrap()
            } else {
                (x..width as i32)
                    .chain(0..x)
                    .cycle()
                    .step_by(dx as usize)
                    .take(101)
                    .last()
                    .unwrap()
            };
            let y = if dy < 0 {
                (y + 1..hight as i32)
                    .chain(0..=y)
                    .rev()
                    .cycle()
                    .step_by(dy.abs() as usize)
                    .take(101)
                    .last()
                    .unwrap()
            } else {
                (y..hight as i32)
                    .chain(0..y)
                    .cycle()
                    .step_by(dy as usize)
                    .take(101)
                    .last()
                    .unwrap()
            };
            if (0..width / 2)
                .chain((width + 1) / 2..width)
                .contains(&(x as usize))
                && (0..hight / 2)
                    .chain((hight + 1) / 2..hight)
                    .contains(&(y as usize))
            {
                return Some((x, y));
            }
            None
        })
        .fold([0, 0, 0, 0], |mut acc, p| {
            let x = p.0 as usize / (width / 2 + 1);
            let y = p.1 as usize / (hight / 2 + 1);
            let idx = x + 2 * y;
            acc[idx] += 1;
            acc
        })
        .iter()
        .product()
}

#[test]
fn day14_1() {
    let input = "p=0,4 v=3,-3
p=6,3 v=-1,-3
p=10,3 v=-1,2
p=2,0 v=2,-1
p=0,0 v=1,3
p=3,0 v=-2,-2
p=7,6 v=-1,-3
p=3,0 v=-1,-2
p=9,3 v=2,3
p=7,3 v=-1,2
p=2,4 v=2,-3
p=9,5 v=-3,-3";
    let result = parse_and_solve(input.lines().map(|s| s.to_owned()), 11, 7);
    assert_eq!(result, 12);
}
