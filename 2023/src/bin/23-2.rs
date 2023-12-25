use aoc2023::read_file_string;

fn main() {
    let data = read_file_string("inputs/23.txt").unwrap();
    println!("Result: {}", solve(data));
}

fn solve(data: String) -> usize {
    let (map, start, end) = parse(&data);
    let mut max = 0;
    let mut paths = Vec::new();
    paths.push(Path { tiles: std::collections::HashSet::new(), length: 0, at: start });

    while let Some(mut path) = paths.pop() {
        path.tiles.insert(path.at);
        let mut prev = Some(path.at);

        loop {
            let mut neighbors: Vec<(usize, usize)> = get_neighbors(&map, &path.at, prev);
            let mut next = neighbors.pop();
            if next.is_none() {
                break;
            }
            if path.tiles.contains(&next.unwrap()) {
                while let Some(n) = neighbors.pop() {
                    next = Some(n);
                    if !path.tiles.contains(&next.unwrap()) {
                        break;
                    }
                }
            }
            if path.tiles.contains(&next.unwrap()) {
                break;
            }

            path.length += 1;
            prev = Some(path.at);
            path.at = next.unwrap();
            for n in neighbors {
                if path.tiles.contains(&n) {
                    continue;
                }
                let mut neighbor = Path { tiles: path.tiles.clone(), length: path.length, at: n };
                neighbor.tiles.insert(neighbor.at);
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

fn get_neighbors(map: &Vec<Vec<Terrain>>, at: &(usize, usize), prev: Option<(usize, usize)>) -> Vec<(usize, usize)> {
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
}

impl From<char> for Terrain {
    fn from(c: char) -> Self {
        match c {
            '#' => Terrain::Forrest,
            _ => Terrain::Path,
        }
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
fn test_23_2() {
    assert_eq!(154, solve("#.#####################
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
#####################.#".to_string()));
}
