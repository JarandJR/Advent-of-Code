use common::{datastructs::vec2::Vec2, io::parse_into_lines_automatic};

const FLOOR: char = '.';
const ROBOT: char = '@';
const BOX_L: char = '[';
const BOX_R: char = ']';
const WALL: char = '#';

fn main() {
    dbg!(parse_and_solve("15"));
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
                        WALL | FLOOR => vec![c; 2],
                        'O' => vec![BOX_L, BOX_R],
                        ROBOT => {
                            start = Vec2::from_row_column(row, column);
                            vec![ROBOT, FLOOR]
                        }
                        _ => vec![],
                    })
                    .collect::<Vec<char>>(),
            );
            row += 1;
        }

        println!();
        for y in 0..grid.len() {
            for x in 0..grid[0].len() {
                print!("{}", grid[y][x]);
            }
            println!();
        }
        println!();

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
            let next = at + mov;
            match grid[next.row()][next.column()] {
                _ => {}
                FLOOR => {
                    grid[at.row()][at.column()] = '.';
                    grid[next.row()][next.column()] = '@';
                    at = next;
                }
                WALL => {}
            }
        }

        return grid
            .iter()
            .enumerate()
            .map(|(y, row)| {
                row.iter().enumerate().fold(0, |acc, (x, t)| match *t {
                    BOX_L => acc + y * 100 + x,
                    _ => acc,
                })
            })
            .sum();
    }
    0
}

#[test]
fn day15_2a() {
    use std::fs::{remove_file, File};
    let file_name = "test_15_2a";
    let file_path = format!("inputs/{}.txt", file_name);
    {
        // Setup for test
        use std::io::Write;
        let mut file = File::create(&file_path).expect("Could not create file");
        writeln!(
            file,
            "#######
#...#.#
#.....#
#..OO@#
#..O..#
#.....#
#######

<vv<<^^<<^^"
        )
        .expect("Could not write to file");
    }
    let result = parse_and_solve(&file_name);
    // Clean up
    remove_file(file_path).expect("Could not remove file");
    assert_eq!(result, 2028);
}
/*
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
}*/
