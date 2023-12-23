use std::{fmt::Debug, f32::INFINITY};

use aoc2023::read_file_string;

fn main() {
    let data = read_file_string("inputs/10.txt").unwrap();
    let (s1, s2) = solve(data);
    println!("Result 1: {}", s1);
    println!("Result 2: {}", s2);
}

fn solve(data: String) -> (i32, usize) {
    let (mut pipes, start) = parse(&data);
    let start = start.unwrap();
    for dir in Direction::directions() {
        let (x, y) = start.to_tuple();
        pipes[y][x].visited = true;
        pipes[y][x].dist = 0;
        let (ox, oy) = get_other(dir, x, y);

        if pipes[oy][ox].ep1.is_none() || pipes[oy][ox].ep2.is_none() {
            continue;
        } else if pipes[oy][ox].ep1.unwrap() != Direction::opposite(&dir) && 
        pipes[oy][ox].ep2.unwrap() != Direction::opposite(&dir) {
            continue;
        }
        let (mut next_x, mut next_y) = pipes[oy][ox].at.to_tuple();

        let mut counter = 1;
        pipes[next_y][next_x].visited = true;
        pipes[next_y][next_x].entered = Some(Direction::opposite(&dir));
        pipes[next_y][next_x].dist = counter;

        while pipes[next_y][next_x].sym != 'S' {
            counter += 1;
            let next_dir = pipes[next_y][next_x].get_next_dir();
            if next_dir.is_none() {
                break;
            }
            let next_dir = next_dir.unwrap();
            let (ox, oy) = get_other(next_dir, next_x, next_y);
            let res = pipes[next_y][next_x].next(next_dir, &pipes[oy][ox]);
            if res.is_err() {
                break;
            }
            (next_x, next_y) = res.unwrap();

            if pipes[next_y][next_x].visited {
                if pipes[next_y][next_x].dist > counter {
                    pipes[next_y][next_x].dist = counter;
                }
            } else {
                pipes[next_y][next_x].visited = true;
                pipes[next_y][next_x].dist = counter;
            }
            
            pipes[next_y][next_x].entered = Some(Direction::opposite(&next_dir));
        }
    }
    let s1 = pipes
        .iter()
        .flat_map(|pipe_vec| pipe_vec.iter())
        .filter(|pipe| pipe.visited)
        .map(|pipe| pipe.dist)
        .max()
        .unwrap_or(0);
    
    let mut count = 0;
    for v in &pipes {
        let mut out = true;
        let last = v.last().unwrap();
        for p in v {
            if p == last {
                continue;
            }
            if p.visited {
                if [
                    '|', '7', 'F', 
                ].contains(&p.sym) {
                    out = match out {
                        true => false,
                        false => true,
                    };
                }  
            } else {
                if !out {
                    count += 1;
                }
            }
        }
    }
    (s1, count)
}

#[derive(Debug, PartialEq, Clone, Copy)]
enum Direction {
    North, 
    South,
    West,
    East,
}

impl Direction {
    fn to_value(dir: &Direction) -> (i32, i32) {
        match dir {
            Direction::North => (0, -1),
            Direction::South => (0, 1),
            Direction::West  => (-1, 0),
            Direction::East  => (1, 0),
        }
    }

    fn directions() -> Vec<Direction> {
        vec![Direction::North, Direction::South, Direction::West, Direction::East]
    }

    fn opposite(dir: &Direction) -> Direction {
        match dir {
            Direction::North => Direction::South,
            Direction::South => Direction::North,
            Direction::West  => Direction::East,
            Direction::East  => Direction::West,
        }
    }
}

#[derive(Debug, PartialEq, Clone, Eq, Hash)]
struct Point {
    x: i32, 
    y: i32,
}

impl Point {
    fn to_tuple(&self) -> (usize, usize) {
        (self.x as usize, self.y as usize)
    }
}

#[derive(Debug, PartialEq)]
struct Pipe {
    sym: char,
    at: Point,
    ep1: Option<Direction>,
    ep2: Option<Direction>,
    visited: bool,
    entered: Option<Direction>,
    dist: i32,
}

impl Pipe {
    fn new(at: Point, sym: char) -> Self {
        let  dirs = Pipe::get_directions(sym);
        let mut ep1 = None;
        let mut ep2 = None;
        if dirs.is_some() {
            let (e1, e2) = dirs.unwrap();
            ep1 = Some(e1);
            ep2 = Some(e2);
        }
        Self { sym, at, ep1, ep2, visited: false , entered: None , dist: INFINITY as i32 }
    }

    fn get_directions(sym: char) -> Option<(Direction, Direction)>{
        match sym {
            '|' => Some((Direction::North, Direction::South)),
            '-' => Some((Direction::West, Direction::East)),
            'L' => Some((Direction::North, Direction::East)),
            'J' => Some((Direction::West, Direction::North)),
            '7' => Some((Direction::West, Direction::South)),
            'F' => Some((Direction::South, Direction::East)),
            _ => None,
        }
    }

    fn next(&self, dir: Direction, other: &Pipe) -> Result<(usize, usize), &str> {
        if other == self {
            return Err("Same pipe");
        }
        if self.is_valid_move(&dir, other) {
            return Ok(other.at.to_tuple());
        }
        Err("Not a valid move")
    }

    fn is_valid_move(&self, dir: &Direction, other: &Pipe) -> bool {
        if self.ep1.is_none() || 
        self.ep2.is_none() {
            return false;
        }
        if other.ep1.is_none() || 
        other.ep2.is_none() {
            if other.sym == 'S' {
                return true;
            }
            return false;
        }
        if dir != self.ep1.as_ref().unwrap() && 
        dir != self.ep2.as_ref().unwrap() {
            return false;
        }
        let (x, y) = Direction::to_value(dir);
        dir == &Direction::opposite(other.ep1.as_ref().unwrap()) || 
        dir == &Direction::opposite(other.ep2.as_ref().unwrap()) && 
        self.at.x + x == other.at.x && 
        self.at.y + y == other.at.y
    }

    fn get_next_dir(&self) -> Option<Direction> {
        let dir = &self.entered.unwrap();
        if self.ep1.is_none() || 
        self.ep2.is_none() {
            return None;
        }
        if dir != self.ep1.as_ref().unwrap() && 
        dir != self.ep2.as_ref().unwrap() {
            return None;
        }
        if dir == self.ep1.as_ref().unwrap() {
            return self.ep2;
        }
        self.ep1
    }
}

fn parse(data: &str) -> (Vec<Vec<Pipe>>, Option<Point>) {
    let mut pipes = Vec::new();
    let mut start_point = None;

    let mut y = 0;
    for l in data.lines() {
        let mut x = 0;
        let mut row = Vec::new();
        for c in l.chars().into_iter() {
            if c == 'S' {
                start_point = Some(Point { x, y });
            }
            row.push(Pipe::new(Point { x, y }, c));
            x += 1;
        }
        pipes.push(row);
        y += 1;
    }
    (pipes, start_point)
}

fn get_other(dir: Direction, x: usize, y: usize) -> (usize, usize) {
    let (dx, dy) = Direction::to_value(&dir);
    let (mut x, mut y) = (x as i32 + dx, y as i32 + dy);
    if x < 0 {
        x = 0;
    } else if y < 0 {
        y = 0;
    }
    (x as usize, y as usize)
}

#[test]
fn test_10_1a() {
    assert_eq!(4, solve(".....
.S-7.
.|.|.
.L-J.
.....".to_string()).0);
}

#[test]
fn test_10_1b() {
    assert_eq!(8, solve("..F7.
.FJ|.
SJ.L7
|F--J
LJ...".to_string()).0);
}

#[test]
fn test_10_2a() {
    assert_eq!(8, solve(".F----7F7F7F7F-7....
.|F--7||||||||FJ....
.||.FJ||||||||L7....
FJL7L7LJLJ||LJ.L-7..
L--J.L7...LJS7F-7L7.
....F-J..F7FJ|L7L7L7
....L7.F7||L7|.L7L7|
.....|FJLJ|FJ|F7|.LJ
....FJL-7.||.||||...
....L---J.LJ.LJLJ...".to_string()).1);
}

#[test]
fn test_10_2b() {
    assert_eq!(10, solve("FF7FSF7F7F7F7F7F---7
L|LJ||||||||||||F--J
FL-7LJLJ||||||LJL-77
F--JF--7||LJLJ7F7FJ-
L---JF-JLJ.||-FJLJJ7
|F|F-JF---7F7-L7L|7|
|FFJF7L7F-JF7|JL---7
7-L-JL7||F7|L7F-7F7|
L.L7LFJ|||||FJL7||LJ
L7JLJL-JLJLJL--JLJ.L".to_string()).1);
}

#[test]
fn test_10_2c() {
    assert_eq!(4, solve("...........
.S-------7.
.|F-----7|.
.||.....||.
.||.....||.
.|L-7.F-J|.
.|..|.|..|.
.L--J.L--J.
...........".to_string()).1);
}
