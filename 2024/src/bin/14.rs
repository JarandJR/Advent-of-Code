use common::io::parse_into_lines_automatic;
use itertools::Itertools;

fn main() {
    dbg!(parse_and_solve_part_1("14", 101, 103));
    dbg!(parse_and_solve_part_2("14"));
}

fn parse_and_solve_part_1(day: &str, width: usize, hight: usize) -> usize {
    if let Some(line_iter) = parse_into_lines_automatic(day) {
        return line_iter
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
            .product();
    }
    0
}

fn parse_and_solve_part_2(day: &str) -> usize {
    if let Some(line_iter) = parse_into_lines_automatic(day) {
        return 0;
    }
    0
}

#[test]
fn day14_1() {
    use std::fs::{remove_file, File};
    let file_name = "test_14_1";
    let file_path = format!("inputs/{}.txt", file_name);
    {
        // Setup for test
        use std::io::Write;
        let mut file = File::create(&file_path).expect("Could not create file");
        writeln!(
            file,
            "p=0,4 v=3,-3
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
p=9,5 v=-3,-3"
        )
        .expect("Could not write to file");
    }
    let result = parse_and_solve_part_1(&file_name, 11, 7);
    // Clean up
    remove_file(file_path).expect("Could not remove file");
    assert_eq!(result, 12);
}

#[test]
fn day14_2() {
    use std::fs::{remove_file, File};
    let file_name = "test_14_2";
    let file_path = format!("inputs/{}.txt", file_name);
    {
        // Setup for test
        use std::io::Write;
        let mut file = File::create(&file_path).expect("Could not create file");
        writeln!(
            file,
            "p=0,4 v=3,-3
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
p=9,5 v=-3,-3"
        )
        .expect("Could not write to file");
    }
    let result = parse_and_solve_part_2(&file_name);
    // Clean up
    remove_file(file_path).expect("Could not remove file");
    assert_eq!(result, 1);
}
