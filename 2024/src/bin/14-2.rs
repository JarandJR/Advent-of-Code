use std::collections::HashSet;

use common::io::parse_into_lines_automatic;
use itertools::Itertools;

fn main() {
    dbg!(parse_and_solve(parse_into_lines_automatic("14"), 101, 103));
}

fn parse_and_solve(line_iter: impl Iterator<Item = String>, width: usize, hight: usize) -> usize {
    let mut robots = line_iter
        .map(|line| {
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

            let x: Box<dyn Iterator<Item = i32>> = if dx < 0 {
                Box::new(
                    (x + 1..width as i32)
                        .chain(0..=x)
                        .rev()
                        .cycle()
                        .step_by(dx.abs() as usize),
                )
            } else {
                Box::new((x..width as i32).chain(0..x).cycle().step_by(dx as usize))
            };
            let y: Box<dyn Iterator<Item = i32>> = if dy < 0 {
                Box::new(
                    (y + 1..hight as i32)
                        .chain(0..=y)
                        .rev()
                        .cycle()
                        .step_by(dy.abs() as usize),
                )
            } else {
                Box::new((y..hight as i32).chain(0..y).cycle().step_by(dy as usize))
            };
            x.zip(y)
        })
        .collect::<Vec<_>>();

    let mut max_unique_positions = 0;
    let mut tree = 0;
    for i in 0..10_000 {
        let mut v = vec![vec!['.'; width]; hight];
        let mut crnt = HashSet::new();
        robots.iter_mut().for_each(|r| {
            let (x, y) = r.next().unwrap();
            v[y as usize][x as usize] = 'A';
            crnt.insert((x, y));
        });
        if crnt.len() > max_unique_positions {
            println!("i {} ->", i);
            max_unique_positions = crnt.len();
            tree = i;
            for row in 0..v.len() {
                for column in 0..v[0].len() {
                    print!("{}", v[row][column])
                }
                println!();
            }
        }
    }
    tree
}
