use std::collections::{HashMap, HashSet};

use common::{datatypes::vec2::Vec2, io::parse_into_lines_automatic};

fn main() {
    dbg!(parse_and_solve(parse_into_lines_automatic("8")));
}

fn parse_and_solve(line_iter: impl Iterator<Item = String>) -> usize {
    let mut rows = 0;
    let mut columns = 0;
    let mut frequencies = HashMap::new();
    line_iter.enumerate().for_each(|(row, line)| {
        columns = line.len();
        line.chars().enumerate().for_each(|(column, c)| {
            if c != '.' {
                frequencies
                    .entry(c)
                    .or_insert(Vec::new())
                    .push(Vec2::from_row_column(row, column));
            }
        });
        rows += 1;
    });

    let mut antinodes = HashSet::new();
    for (_, signals) in frequencies {
        // Frequency antennas all in line with at least two antennas
        if signals.len() > 2 {
            signals.iter().for_each(|p| {
                antinodes.insert(*p);
            });
        }
        let mut stack = vec![(signals[0], &signals[1..])];
        while let Some((a, positions)) = stack.pop() {
            if positions.is_empty() {
                continue;
            }
            let b = positions[0];
            let dx = a.x - b.x;
            let dy = a.y - b.y;
            let dir = Vec2::new(dx, dy);
            [(a, dir), (b, dir.inverse())]
                .into_iter()
                .for_each(|(p, dir)| {
                    let mut antinode = p + dir;
                    while (0..rows as i32).contains(&antinode.y)
                        && (0..columns as i32).contains(&antinode.x)
                    {
                        antinodes.insert(antinode);
                        antinode += dir;
                    }
                });

            stack.push((b, &positions[1..]));
            stack.push((a, &positions[1..]));
        }
    }
    antinodes.len()
}

#[test]
fn day8_2() {
    let input = "............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............";
    let result = parse_and_solve(input.lines().map(|s| s.to_owned()));
    assert_eq!(result, 34);
}
