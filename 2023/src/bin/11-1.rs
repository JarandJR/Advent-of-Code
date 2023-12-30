use aoc2023::read_file_string;

fn main() {
    let data = read_file_string("inputs/11.txt").unwrap();
    println!("Result: {}", solve(data));
}

fn solve(data: String) -> usize {
    let space = parse(data);
    let mut map = std::collections::HashMap::new();
    let mut galaxies = 0;
    for y in 0..space.len() {
        for x in 0..space[y].len() {
            if space[y][x] == '#' {
                map.insert(galaxies, (x, y));
                galaxies += 1;
            }
        }
    }

    let mut sum = 0;
    for g in 0..galaxies {
        for g2 in g+1..galaxies {
            let (x, y) = map[&g];
            let (x2, y2) = map[&g2];
            sum += dist(x, y, x2, y2);
        }
    }
    sum
}

fn parse(data: String) -> Vec<Vec<char>> {
    let mut space = data
        .lines()
        .map(|line| line
            .chars()
            .collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let mut y = 0;
    while y < space.len() {
            if space[y].iter().all(|c| *c == '.') {
                space
                .insert(y, vec!['.'; space[y].len()]);
                y += 1;
            }
            y += 1;
        }
    
    let mut x = 0;
    while x < space[0].len() {
            if space.iter().all(|line| line[x] == '.') {
                space
                .iter_mut()
                .for_each(|line| line
                    .insert(x, '.'));
                x += 1;
            }
            x += 1;
        }
    space
}

fn dist(x: usize, y: usize, x2: usize, y2: usize) -> usize {
    ((x as isize - x2 as isize).abs() + (y as isize - y2 as isize).abs()) as usize
}

#[test]
fn test_11_1() {
    assert_eq!(374, solve("...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....".to_string()));
}
