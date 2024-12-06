use aoc2023::read_file_string;

fn main() {
    let data = read_file_string("inputs/23.txt").unwrap();
    println!("Result: {}", solve(data));
}

fn solve(data: String) -> usize {
    let (map, start, end) = parse(&data);
    let mut max = 0;
    let mut paths = Vec::new();
    paths.push(Path {
        tiles: std::collections::HashSet::new(),
        length: 0,
        at: start,
    });

    while let Some(mut path) = paths.pop() {
        path.tiles.insert(path.at);
        let mut prev = Some(path.at);

        loop {
            let mut neighbors = get_neighbors(&map, &path.at, prev);
            let mut next = neighbors.pop();
            if path.tiles.contains(&next.unwrap()) {
                while neighbors.len() > 0 {
                    next = neighbors.pop();
                    if next.is_none() {
                        break;
                    }
                    if !path.tiles.contains(&next.unwrap()) {
                        break;
                    }
                }
            }

            path.length += 1;
            prev = Some(path.at);
            path.at = next.unwrap();
            while let Terrain::Slope(slope) = map[path.at.1][path.at.0] {
                let next = (
                    (path.at.0 as isize + slope.0) as usize,
                    (path.at.1 as isize + slope.1) as usize,
                );
                if path.tiles.contains(&next) {
                    break;
                }
                path.length += 1;
                prev = Some(path.at);
                path.at = next;
                path.tiles.insert(path.at);
                neighbors = get_neighbors(&map, &path.at, prev);
                neighbors.pop();
            }

            for n in neighbors {
                let mut neighbor = Path {
                    tiles: path.tiles.clone(),
                    length: path.length + 1,
                    at: n,
                };
                neighbor.tiles.insert(neighbor.at);
                let mut invalid_slope = false;
                while let Terrain::Slope(slope) = map[neighbor.at.1][neighbor.at.0] {
                    let next = (
                        (neighbor.at.0 as isize + slope.0) as usize,
                        (neighbor.at.1 as isize + slope.1) as usize,
                    );
                    if neighbor.tiles.contains(&next) {
                        invalid_slope = true;
                        break;
                    }
                    neighbor.length += 1;
                    neighbor.at = next;
                    neighbor.tiles.insert(neighbor.at);
                }
                if invalid_slope {
                    continue;
                }
                paths.push(neighbor);
            }
            path.tiles.insert(path.at);
            if path.at == end {
                if path.length > max {
                    max = path.length;
                }
                break;
            }
        }
    }
    max
}

fn get_neighbors(
    map: &Vec<Vec<Terrain>>,
    at: &(usize, usize),
    prev: Option<(usize, usize)>,
) -> Vec<(usize, usize)> {
    let mut neighbors = Vec::new();
    let (x, y) = at;
    for dir in [(-1, 0), (0, -1), (1, 0), (0, 1)].iter() {
        let next_x = *x as isize + dir.0;
        let next_y = *y as isize + dir.1;
        if next_x < 0 || next_y < 0 {
            continue;
        }
        if next_x >= map.len() as isize || next_y >= map[0].len() as isize {
            continue;
        }
        let next = (next_x as usize, next_y as usize);
        if prev.is_some() {
            if next == prev.unwrap() {
                continue;
            }
        }
        if map[next.1][next.0] == Terrain::Forrest {
            continue;
        }
        neighbors.push(next);
    }
    neighbors
}

#[derive(Debug)]
struct Path {
    tiles: std::collections::HashSet<(usize, usize)>,
    length: usize,
    at: (usize, usize),
}

#[derive(Debug, PartialEq)]
enum Terrain {
    Path,
    Forrest,
    Slope((isize, isize)),
}

impl From<char> for Terrain {
    fn from(c: char) -> Self {
        match c {
            '#' => Terrain::Forrest,
            '.' => Terrain::Path,
            _ => Terrain::Slope(from_slope(&c)),
        }
    }
}

fn from_slope(s: &char) -> (isize, isize) {
    match s {
        '^' => (0, -1),
        'v' => (0, 1),
        '<' => (-1, 0),
        '>' => (1, 0),
        _ => panic!("Invalid slope"),
    }
}

fn parse(data: &str) -> (Vec<Vec<Terrain>>, (usize, usize), (usize, usize)) {
    let mut map = Vec::new();
    let mut start = (0, 0);
    let mut end = (0, 0);

    data.lines().enumerate().for_each(|(y, line)| {
        let mut row = Vec::new();
        line.chars().enumerate().for_each(|(x, c)| {
            let terrain = Terrain::from(c);
            if terrain == Terrain::Path && start == (0, 0) {
                start = (x, y);
            } else if terrain == Terrain::Path {
                end = (x, y);
            }
            row.push(terrain);
        });
        map.push(row);
    });
    (map, start, end)
}

#[test]
fn test_23_1() {
    assert_eq!(
        94,
        solve(
            "#.#####################
#.......#########...###
#######.#########.#.###
###.....#.>.>.###.#.###
###v#####.#v#.###.#.###
###.>...#.#.#.....#...#
###v###.#.#.#########.#
###...#.#.#.......#...#
#####.#.#.#######.#.###
#.....#.#.#.......#...#
#.#####.#.#.#########v#
#.#...#...#...###...>.#
#.#.#v#######v###.###v#
#...#.>.#...>.>.#.###.#
#####v#.#.###v#.#.###.#
#.....#...#...#.#.#...#
#.#########.###.#.#.###
#...###...#...#...#.###
###.###.#.###v#####v###
#...#...#.#.>.>.#.>.###
#.###.###.#.###.#.#v###
#.....###...###...#...#
#####################.#"
                .to_string()
        )
    );
}
