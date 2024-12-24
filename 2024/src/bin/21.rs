use std::collections::{HashMap, VecDeque};

use common::{datatypes::vec2::Vec2, io::parse_into_lines_automatic};
use itertools::Itertools;

fn main() {
    //dbg!(parse_and_solve(parse_into_lines_automatic("21")));
    dbg!(parse_and_solve_2(parse_into_lines_automatic("21")));
}

fn parse_and_solve_2(line_iter: impl Iterator<Item = String>) -> usize {
    let number_map = vec!['7', '4', '1', 'X', '8', '5', '2', '0', '9', '6', '3', 'A'];
    let numeric_pad = (0..3)
        .flat_map(|x| (0..4).map(move |y| Vec2::new(x, y)))
        .enumerate()
        .map(|(i, p)| (number_map[i], p))
        .collect::<HashMap<char, Vec2>>();
    let arrow_map = vec!['X', '<', '^', 'v', 'A', '>'];
    let arrow_pad = (0..3)
        .flat_map(|x| (0..2).map(move |y| Vec2::new(x, y)))
        .enumerate()
        .map(|(i, p)| (arrow_map[i], p))
        .collect::<HashMap<char, Vec2>>();

    //let mut mem = HashMap::new();
    let mut numeric_robot = numeric_pad.get(&'A').unwrap();
    let mut arrow_robots = vec![arrow_pad.get(&'A').unwrap(), arrow_pad.get(&'A').unwrap()];
    let mut other_arrow_robots = vec![arrow_pad.get(&'A').unwrap(), arrow_pad.get(&'A').unwrap()];

    line_iter.fold(0, |acc, line| {
        let numeric_part = line
            .chars()
            .filter(|c| c.is_numeric())
            .collect::<String>()
            .parse::<usize>()
            .unwrap();

        let mut shortest_path = 0;
        for c in line.chars() {
            println!("AT: {}", c);
            let next_robot_pos = numeric_pad.get(&c).unwrap();
            let distance = *next_robot_pos - *numeric_robot;
            let mut moves = distance_to_moves(distance, &arrow_pad);

            let mut other_moves = VecDeque::new();
            if distance.x < 0 {
                other_moves.extend(vec!['<'; distance.x.abs() as usize]);
            }
            if distance.y < 0 {
                other_moves.extend(vec!['^'; distance.y.abs() as usize]);
            }
            if distance.y >= 0 {
                other_moves.extend(vec!['v'; distance.y as usize]);
            }
            if distance.x >= 0 {
                other_moves.extend(vec!['>'; distance.x as usize]);
            }
            other_moves.push_back('A');
            println!("First moves OTHER: {:?}", other_moves);
            other_arrow_robots.iter_mut().for_each(|r| {
                let mut next_lv = VecDeque::new();
                while let Some(mov) = other_moves.pop_front() {
                    let next_r_pos = arrow_pad.get(&mov).unwrap();
                    let distance = *next_r_pos - **r;
                    next_lv.extend(distance_to_moves(distance, &arrow_pad));
                    next_lv.push_back('A');
                    *r = next_r_pos;
                }
                println!("next lv OTHER: {:?}", next_lv);
                other_moves = next_lv;
            });

            moves.push_back('A');
            println!("First moves: {:?}", moves);

            arrow_robots.iter_mut().for_each(|r| {
                let mut next_lv = VecDeque::new();
                while let Some(mov) = moves.pop_front() {
                    let next_r_pos = arrow_pad.get(&mov).unwrap();
                    let distance = *next_r_pos - **r;
                    next_lv.extend(distance_to_moves(distance, &arrow_pad));
                    next_lv.push_back('A');
                    *r = next_r_pos;
                }
                println!("next lv: {:?}", next_lv);
                moves = next_lv;
            });
            println!("len: {} other: {}", moves.len(), other_moves.len());
            numeric_robot = next_robot_pos;
            shortest_path += moves.len().min(other_moves.len());
        }
        println!("path: {}\n", shortest_path);
        acc + numeric_part * shortest_path
    }) // 265810 to high
       // 251826 to high
}

fn dfs(
    curnt: &str,
    limit: usize,
    arrow_pad: &HashMap<char, Vec2>,
    numeric_pad: &HashMap<char, Vec2>,
    numeric_robot: &mut Vec2,
) -> usize {
    if curnt.is_empty() {
        return 0;
    }
    let c = curnt[0..1].parse::<char>().unwrap();
    let next_robot_pos = numeric_pad.get(&c).unwrap();
    let distance = *next_robot_pos - *numeric_robot;
    // Search for these moves with BFS
    let mut moves = distance_to_moves(distance, arrow_pad);
    while let Some(mov) = moves.pop_front() {}
    0
}

fn parse_and_solve(line_iter: impl Iterator<Item = String>) -> usize {
    /*
    Robot 1: Depressurized
    +---+---+---+
    | 7 | 8 | 9 |
    +---+---+---+
    | 4 | 5 | 6 |
    +---+---+---+
    | 1 | 2 | 3 |
    +---+---+---+
        | 0 | A |
        +---+---+

    Robot 2: Radiation
        +---+---+
        | ^ | A |
    +---+---+---+
    | < | v | > |
    +---+---+---+

    Robot 3: -40 Degrees
        +---+---+
        | ^ | A |
    +---+---+---+
    | < | v | > |
    +---+---+---+

    You
        +---+---+
        | ^ | A |
    +---+---+---+
    | < | v | > |
    +---+---+---+
    */
    let number_map = vec!['7', '4', '1', 'X', '8', '5', '2', '0', '9', '6', '3', 'A'];
    let numeric_pad = (0..3)
        .flat_map(|x| (0..4).map(move |y| Vec2::new(x, y)))
        .enumerate()
        .map(|(i, p)| (number_map[i], p))
        .collect::<HashMap<char, Vec2>>();
    let arrow_map = vec!['X', '<', '^', 'v', 'A', '>'];
    let arrow_pad = (0..3)
        .flat_map(|x| (0..2).map(move |y| Vec2::new(x, y)))
        .enumerate()
        .map(|(i, p)| (arrow_map[i], p))
        .collect::<HashMap<char, Vec2>>();

    let mut numeric_robot = numeric_pad.get(&'A').unwrap();
    let mut arrow_robots = vec![arrow_pad.get(&'A').unwrap(), arrow_pad.get(&'A').unwrap()];
    line_iter.fold(0, |acc, line| {
        let numeric_part = line
            .chars()
            .filter(|c| c.is_numeric())
            .collect::<String>()
            .parse::<usize>()
            .unwrap();
        let shortest_path = line
            .chars()
            .flat_map(|c| {
                println!("AT: {}", c);
                let next_robot_pos = numeric_pad.get(&c).unwrap();
                let distance = *next_robot_pos - *numeric_robot;
                let mut moves = distance_to_moves(distance, &arrow_pad);
                moves.push_back('A');
                println!("First moves: {:?}", moves);

                arrow_robots.iter_mut().for_each(|r| {
                    let mut next_lv = VecDeque::new();
                    while let Some(mov) = moves.pop_front() {
                        let next_r_pos = arrow_pad.get(&mov).unwrap();
                        let distance = *next_r_pos - **r;
                        next_lv.extend(distance_to_moves(distance, &arrow_pad));
                        next_lv.push_back('A');
                        *r = next_r_pos;
                    }
                    println!("next lv: {:?}", next_lv);
                    moves = next_lv;
                });
                println!("len: {}", moves.len());
                numeric_robot = next_robot_pos;
                moves
            })
            .count();
        println!("path: {}\n", shortest_path);
        acc + numeric_part * shortest_path
    }) // 265810 to high
}

fn distance_to_moves(distance: Vec2, arrow_pad: &HashMap<char, Vec2>) -> VecDeque<char> {
    let mut moves = VecDeque::new();
    if distance.x < 0 {
        moves.extend(vec!['<'; distance.x.abs() as usize]);
    } else {
        moves.extend(vec!['>'; distance.x as usize]);
    }
    if distance.y < 0 {
        moves.extend(vec!['^'; distance.y.abs() as usize]);
    } else {
        moves.extend(vec!['v'; distance.y as usize]);
    }

    let pos_a = arrow_pad.get(&'A').unwrap();
    moves
        .into_iter()
        .sorted_by(|c, other| {
            let distance = (*arrow_pad.get(c).unwrap() - *pos_a).abs().sum();
            let other_dist = (*arrow_pad.get(other).unwrap() - *pos_a).abs().sum();
            distance.cmp(&other_dist)
        })
        .collect()
}

#[test]
fn day21_1() {
    let input = "029A
980A
179A
456A
379A";
    let result = parse_and_solve(input.lines().map(|s| s.to_owned()));
    assert_eq!(result, 126384);
}
