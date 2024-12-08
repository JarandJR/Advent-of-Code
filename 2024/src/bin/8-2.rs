use std::collections::{HashMap, HashSet};

use common::{datastructs::vec2::Vec2, io::parse_into_lines_automatic};

fn main() {
    dbg!(parse_and_solve("8"));
}

fn parse_and_solve(day: &str) -> usize {
    if let Some(line_iter) = parse_into_lines_automatic(day) {
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
        return antinodes.len();
    }
    0
}

#[test]
fn day8_2() {
    use std::fs::{remove_file, File};
    let file_name = "test_8_2";
    let file_path = format!("inputs/{}.txt", file_name);
    {
        // Setup for test
        use std::io::Write;
        let mut file = File::create(&file_path).expect("Could not create file");
        writeln!(
            file,
            "............
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
............"
        )
        .expect("Could not write to file");
    }
    let result = parse_and_solve(&file_name);
    // Clean up
    remove_file(file_path).expect("Could not remove file");
    assert_eq!(result, 34);
}
