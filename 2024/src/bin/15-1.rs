use common::{datatypes::vec2::Vec2, io::parse_into_lines_automatic};

fn main() {
    dbg!(parse_and_solve(parse_into_lines_automatic("15")));
}

enum Tile {
    Box,
    Robot,
    Wall,
    Floor,
}

impl std::fmt::Debug for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Tile::Box => write!(f, "O"),
            Tile::Wall => write!(f, "#"),
            Tile::Floor => write!(f, "."),
            Tile::Robot => write!(f, "@"),
        }
    }
}

fn parse_and_solve(mut line_iter: impl Iterator<Item = String>) -> usize {
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
                .filter_map(|(column, c)| match c {
                    '#' => Some(Tile::Wall),
                    '.' => Some(Tile::Floor),
                    'O' => Some(Tile::Box),
                    '@' => {
                        start = Vec2::from_row_column(row, column);
                        Some(Tile::Robot)
                    }
                    _ => None,
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
        let next = at + mov;
        match grid[next.row()][next.column()] {
            Tile::Box => {
                let mut at_box = next;
                let mut stack = vec![at_box + mov];
                loop {
                    let next = at_box + mov;
                    match grid[next.row()][next.column()] {
                        Tile::Box => {
                            at_box = next;
                            stack.push(at_box + mov);
                        }
                        Tile::Floor | Tile::Robot => {
                            break;
                        }
                        Tile::Wall => {
                            stack.clear();
                            break;
                        }
                    }
                }
                if !stack.is_empty() {
                    grid[at.row()][at.column()] = Tile::Floor;
                    grid[next.row()][next.column()] = Tile::Robot;
                    at = next;
                }
                while let Some(pos) = stack.pop() {
                    grid[pos.row()][pos.column()] = Tile::Box;
                }
            }
            Tile::Floor | Tile::Robot => {
                grid[at.row()][at.column()] = Tile::Floor;
                grid[next.row()][next.column()] = Tile::Robot;
                at = next;
            }
            Tile::Wall => {}
        }
    }
    grid.iter()
        .enumerate()
        .map(|(y, row)| {
            row.iter().enumerate().fold(0, |acc, (x, t)| match t {
                Tile::Box => acc + y * 100 + x,
                _ => acc,
            })
        })
        .sum()
}

#[test]
fn day15_1a() {
    let input = "########
#..O.O.#
##@.O..#
#...O..#
#.#.O..#
#...O..#
#......#
########

<^^>>>vv<v>>v<<";
    let result = parse_and_solve(input.lines().map(|s| s.to_owned()));
    assert_eq!(result, 2028);
}

#[test]
fn day15_1b() {
    let input = "##########
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
v^^>>><<^^<>>^v^<v^vv<>v^<<>^<^v^v><^<<<><<^<v><v<>vv>>v><v^<vv<>v^<<^";
    let result = parse_and_solve(input.lines().map(|s| s.to_owned()));
    assert_eq!(result, 10092);
}
