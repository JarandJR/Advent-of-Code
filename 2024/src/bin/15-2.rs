use std::collections::HashSet;

use common::{datastructs::vec2::Vec2, io::parse_into_lines_automatic};

fn main() {
    dbg!(parse_and_solve("15"));
}

#[derive(Clone, Copy, Hash, PartialEq, Eq)]
enum Tile {
    BoxL,
    BoxR,
    Robot,
    Wall,
    Floor,
}

impl std::fmt::Debug for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Tile::BoxL => write!(f, "["),
            Tile::BoxR => write!(f, "]"),
            Tile::Wall => write!(f, "#"),
            Tile::Floor => write!(f, "."),
            Tile::Robot => write!(f, "@"),
        }
    }
}

fn parse_and_solve(day: &str) -> usize {
    if let Some(mut line_iter) = parse_into_lines_automatic(day) {
        let mut row = 0;
        let mut start = Vec2::default();
        let mut grid = Vec::new();
        while let Some(line) = line_iter.next() {
            if line.is_empty() {
                break;
            }
            grid.push(
                line.chars()
                    .enumerate()
                    .flat_map(|(column, c)| match c {
                        '#' => vec![Tile::Wall; 2],
                        '.' => vec![Tile::Floor; 2],
                        'O' => vec![Tile::BoxL, Tile::BoxR],
                        '@' => {
                            start = Vec2::from_row_column(row, column * 2);
                            vec![Tile::Robot, Tile::Floor]
                        }
                        _ => vec![],
                    })
                    .collect::<Vec<Tile>>(),
            );
            row += 1;
        }
        let moves = line_iter
            .flat_map(|line| {
                line.chars()
                    .filter_map(|c| match c {
                        '<' => Some(Vec2::WEST),
                        '^' => Some(Vec2::NORTH),
                        '>' => Some(Vec2::EAST),
                        'v' => Some(Vec2::SOUTH),
                        _ => None,
                    })
                    .collect::<Vec<Vec2>>()
            })
            .collect::<Vec<Vec2>>();

        let mut at = start;
        for mov in moves {
            let mut objects = HashSet::new();
            let mut stack = vec![at];
            while let Some(pos) = stack.pop() {
                let curnt = grid[pos.row()][pos.column()];
                if objects.contains(&(pos, curnt)) {
                    continue;
                }
                match curnt {
                    Tile::Wall => {
                        objects.clear();
                        break;
                    }
                    Tile::Floor => {
                        continue;
                    }
                    Tile::BoxL => {
                        stack.push(pos + Vec2::from_row_column(0, 1));
                    }
                    Tile::BoxR => {
                        stack.push(pos - Vec2::from_row_column(0, 1));
                    }
                    Tile::Robot => {}
                }
                objects.insert((pos, curnt));
                stack.push(pos + mov);
            }
            if !objects.is_empty() {
                at += mov;
            }
            // Clear grid
            for (pos, _) in objects.iter() {
                grid[pos.row()][pos.column()] = Tile::Floor;
            }
            // Paint new grid
            for (pos, c) in objects.iter() {
                let moved = *pos + mov;
                grid[moved.row()][moved.column()] = *c;
            }
        }
        return grid
            .iter()
            .enumerate()
            .map(|(y, row)| {
                row.iter().enumerate().fold(0, |acc, (x, t)| match *t {
                    Tile::BoxL => acc + y * 100 + x,
                    _ => acc,
                })
            })
            .sum();
    }
    0
}

#[test]
fn day15_2() {
    use std::fs::{remove_file, File};
    let file_name = "test_15_2";
    let file_path = format!("inputs/{}.txt", file_name);
    {
        // Setup for test
        use std::io::Write;
        let mut file = File::create(&file_path).expect("Could not create file");
        writeln!(
            file,
            "##########
#..O..O.O#
#......O.#
#.OO..O.O#
#..O@..O.#
#O#..O...#
#O..O..O.#
#.OO.O.OO#
#....O...#
##########

<vv>^<v^>v>^vv^v>v<>v^v<v<^vv<<<^><<><>>v<vvv<>^v^>^<<<><<v<<<v^vv^v>^
vvv<<^>^v^^><<>>><>^<<><^vv^^<>vvv<>><^^v>^>vv<>v<<<<v<^v>^<^^>>>^<v<v
><>vv>v^v^<>><>>>><^^>vv>v<^^^>>v^v^<^^>v^^>v^<^v>v<>>v^v^<v>v^^<^^vv<
<<v<^>>^^^^>>>v^<>vvv^><v<<<>^^^vv^<vvv>^>v<^^^^v<>^>vvvv><>>v^<<^^^^^
^><^><>>><>^^<<^^v>>><^<v>^<vv>>v>>>^v><>^v><<<<v>>v<v<v>vvv>^<><<>^><
^>><>^v<><^vvv<^^<><v<<<<<><^v<<<><<<^^<v<^^^><^>>^<v^><<<^>>^v<v^v<v^
>^>>^v>vv>^<<^v<>><<><<v<<v><>v<^vv<<<>^^v^>^^>>><<^v>>v^v><^^>>^<>vv^
<><^^>^^^<><vvvvv^v<v<<>^v<v>v<<^><<><<><<<^^<<<^<<>><<><^^^>^^<>^>v<>
^^>vv<^v^v<vv>^<><v<^v>^^^>>>^^vvv^>vvv<>>>^<^>>>>>^<<^v>^vvv<>^<><<v>
v^^>>><<^^<>>^v^<v^vv<>v^<<>^<^v^v><^<<<><<^<v><v<>vv>>v><v^<vv<>v^<<^"
        )
        .expect("Could not write to file");
    }
    let result = parse_and_solve(&file_name);
    // Clean up
    remove_file(file_path).expect("Could not remove file");
    assert_eq!(result, 9021);
}
