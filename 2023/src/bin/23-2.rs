use std::sync::{Arc, Mutex, Condvar};
use aoc2023::read_file_string;

fn main() {
    let data = read_file_string("inputs/23.txt").unwrap();
    println!("Result: {}", solve(data));
}

fn solve(data: String) -> usize {
    let (map, start, end) = parse(&data);
    let map = Arc::new(map);
    let max = Arc::new(Mutex::new(0));
    let paths = Arc::new(Mutex::new(vec![Path {
        tiles: std::collections::HashSet::new(),
        length: 0,
        at: start,
    }]));
    let mut threads = Vec::new();
    let cond = Arc::new((Mutex::new(false), Condvar::new()));


    loop {
        let cond_copy = Arc::clone(&cond);
        let (lock, c) = &*cond;
        {
            let mut wait = lock.lock().unwrap();
            while *wait {
                wait = c.wait(wait).unwrap();
                if threads.is_empty() {
                    break;
                }
            }
        }
        let path = paths.lock().unwrap().pop();
        if path.is_none() {
            break;
        }
        let mut path = path.unwrap();
        path.tiles.insert(path.at);
        let mut prev = Some(path.at);

        let max_clone = Arc::clone(&max);
        let paths_clone = Arc::clone(&paths);
        let map_clone = Arc::clone(&map);
        threads.push(std::thread::spawn(move || {
            loop {
                let mut neighbors: Vec<(usize, usize)> = get_neighbors(&map_clone, &path.at, prev);
                let mut next = neighbors.pop();
                if next.is_none() {
                    let (lock, c) = &*cond_copy;
                    *lock.lock().unwrap() = false;
                    c.notify_all();
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
                    let (lock, c) = &*cond_copy;
                    *lock.lock().unwrap() = false;
                    c.notify_all();
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
                    paths_clone.lock().unwrap().push(neighbor);
                    let (lock, c) = &*cond_copy;
                    *lock.lock().unwrap() = false;
                    c.notify_all();
                }
                path.tiles.insert(path.at);
                if path.at == end {
                    let (lock, c) = &*cond_copy;
                    *lock.lock().unwrap() = false;
                    c.notify_all();
                    let mut m = max_clone.lock().unwrap();
                    if path.length > *m {
                        println!("New max: {}", path.length);
                        *m = path.length;
                    }
                    break;
                }
            }
        }));
        let (lock, _) = &*cond;
        *lock.lock().unwrap() = true;
    }
    threads.into_iter().for_each(|t| t.join().unwrap());
    let m = max.lock().unwrap();
    *m
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
