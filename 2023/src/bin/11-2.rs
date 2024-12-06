use aoc2023::read_file_string;

fn main() {
    let data = read_file_string("inputs/11.txt").unwrap();
    println!("Result: {}", solve(data, 1_000_000));
}

fn solve(data: String, multi: usize) -> usize {
    let space = parse(data, multi);
    let mut map = std::collections::HashMap::new();
    let mut galaxies = 0;

    let mut y_pos = 0;
    for y in 0..space.len() {
        let mut x_pos = 0;
        for x in 0..space[y].len() {
            if space[y][x] == Space::Galaxy {
                map.insert(galaxies, (x_pos, y_pos));
                galaxies += 1;
            }
            if space[0][x] == Space::Empty(multi) {
                x_pos += multi;
            } else {
                x_pos += 1;
            }
        }
        if space[y][0] == Space::Empty(multi) {
            y_pos += multi;
        } else {
            y_pos += 1;
        }
    }

    let mut sum = 0;
    for g in 0..galaxies {
        for g2 in g + 1..galaxies {
            let (x, y) = map[&g];
            let (x2, y2) = map[&g2];
            sum += dist(x, y, x2, y2);
        }
    }
    sum
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Space {
    Empty(usize),
    Galaxy,
}

impl From<char> for Space {
    fn from(c: char) -> Self {
        match c {
            '.' => Space::Empty(1),
            '#' => Space::Galaxy,
            _ => panic!("Unknown space type: {}", c),
        }
    }
}

fn parse(data: String, m: usize) -> Vec<Vec<Space>> {
    let mut space = data
        .lines()
        .map(|line| line.chars().map(|c| Space::from(c)).collect::<Vec<_>>())
        .collect::<Vec<_>>();
    let mut y = 0;
    while y < space.len() {
        if space[y].iter().all(|c| *c == Space::Empty(1)) {
            space.remove(y);
            space.insert(y, vec![Space::Empty(m); space[y].len()]);
        } else {
        }
        y += 1;
    }

    let mut x = 0;
    while x < space[0].len() {
        if space
            .iter()
            .all(|line| line[x] == Space::Empty(1) || line[x] == Space::Empty(m))
        {
            space.iter_mut().for_each(|line| {
                line[x] = Space::Empty(m);
            });
        }
        x += 1;
    }
    space
}

fn dist(x: usize, y: usize, x2: usize, y2: usize) -> usize {
    ((x as isize - x2 as isize).abs() + (y as isize - y2 as isize).abs()) as usize
}

#[test]
fn test_11_2a() {
    assert_eq!(
        1030,
        solve(
            "...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#....."
                .to_string(),
            10
        )
    );
}

#[test]
fn test_11_2b() {
    assert_eq!(
        8410,
        solve(
            "...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#....."
                .to_string(),
            100
        )
    );
}
