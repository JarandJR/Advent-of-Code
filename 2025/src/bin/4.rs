use common::{datastructs::grid::Grid, datatypes::vec2::Vec2, io::parse_into_lines_automatic};

fn main() {
    dbg!(parse_and_solve_1(parse_into_lines_automatic("4")));
    dbg!(parse_and_solve_2(parse_into_lines_automatic("4")));
}

fn parse_and_solve_1(line_iter: impl Iterator<Item = String>) -> usize {
    let mut que = Vec::new();
    let grid = line_iter
        .enumerate()
        .map(|(row, line)| {
            line.chars()
                .enumerate()
                .map(|(col, c)| {
                    c.eq(&'@')
                        .then(|| que.push(Vec2::from_row_column(row, col)));
                    c
                })
                .collect::<Vec<char>>()
        })
        .collect::<Grid<char>>();
    que.drain(..)
        .filter(|roll| {
            let neighbor_rolls = grid
                .eight_connectedness(*roll, |c| c.eq(&'@'))
                .iter()
                .count();
            neighbor_rolls < 4
        })
        .count()
}

fn parse_and_solve_2(line_iter: impl Iterator<Item = String>) -> usize {
    let mut que = Vec::new();
    let mut grid = line_iter
        .enumerate()
        .map(|(row, line)| {
            line.chars()
                .enumerate()
                .map(|(col, c)| {
                    c.eq(&'@')
                        .then(|| que.push(Vec2::from_row_column(row, col)));
                    c
                })
                .collect::<Vec<char>>()
        })
        .collect::<Grid<char>>();
    let mut roll_count = 0;
    loop {
        let count = que
            .drain(..)
            .filter(|roll| {
                let neighbor_rolls = grid
                    .eight_connectedness(*roll, |c| c.eq(&'@'))
                    .iter()
                    .count();
                if neighbor_rolls < 4 {
                    grid[*roll] = '.';
                }
                neighbor_rolls < 4
            })
            .count();
        if count == 0 {
            break;
        }
        roll_count += count;
        grid.iter().enumerate().for_each(|(row, row_data)| {
            row_data.iter().enumerate().for_each(|(col, c)| {
                c.eq(&'@')
                    .then(|| que.push(Vec2::from_row_column(row, col)));
            });
        });
    }
    roll_count
}

#[test]
fn day4_1() {
    let input = "..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@.";
    let result = parse_and_solve_1(input.lines().map(|s| s.to_owned()));
    assert_eq!(result, 13);
}

#[test]
fn day4_2() {
    let input = "..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@.";
    let result = parse_and_solve_2(input.lines().map(|s| s.to_owned()));
    assert_eq!(result, 43);
}
