use std::collections::{BinaryHeap, HashSet};

use common::{datatypes::priority::Priority, io::parse_into_lines_automatic};

fn main() {
    dbg!(parse_and_solve_1(parse_into_lines_automatic("8"), 1_000));
    dbg!(parse_and_solve_2(parse_into_lines_automatic("8")));
}

fn parse_and_solve_1(line_iter: impl Iterator<Item = String>, k: usize) -> usize {
    let (_, mut heap) = parse(line_iter);
    let mut graph: Vec<HashSet<usize>> = Vec::new();
    for _ in 1..=k {
        if let Some(data) = heap.pop() {
            let data = *data;
            extend_graph(&mut graph, data);
        }
    }
    let mut top_paths = graph.iter().map(|c| c.len()).collect::<BinaryHeap<usize>>();
    (0..3).flat_map(|_| top_paths.pop()).product()
}

fn parse_and_solve_2(line_iter: impl Iterator<Item = String>) -> i64 {
    let (points, mut heap) = parse(line_iter);
    let mut graph: Vec<HashSet<usize>> = Vec::new();
    while let Some(data) = heap.pop() {
        let data = *data;
        extend_graph(&mut graph, data);
        if graph
            .iter()
            .all(|c| (0..points.len()).all(|i| c.contains(&i)))
        {
            return points[data.0][0] * points[data.1][0];
        }
    }
    panic!()
}

fn extend_graph(graph: &mut Vec<HashSet<usize>>, data: (usize, usize)) {
    let mut paths_contains = graph
        .iter()
        .enumerate()
        .filter(|(_, path)| path.contains(&data.0) || path.contains(&data.1))
        .map(|(i, _)| i)
        .collect::<Vec<usize>>();
    if !paths_contains.is_empty() {
        paths_contains.sort_unstable_by(|a, b| b.cmp(a));
        let first = paths_contains.pop().unwrap();
        paths_contains.iter().for_each(|con| {
            let other = graph.remove(*con);
            graph[first].extend(other);
        });
        graph[first].extend([data.0, data.1]);
    } else {
        graph.push(HashSet::from([data.0, data.1]));
    }
}

fn parse(
    line_iter: impl Iterator<Item = String>,
) -> (Vec<Vec<i64>>, BinaryHeap<Priority<i64, (usize, usize)>>) {
    let points = line_iter
        .map(|line| {
            line.split(",")
                .flat_map(|n| n.parse::<i64>())
                .collect::<Vec<i64>>()
        })
        .collect::<Vec<Vec<i64>>>();
    let mut heap = BinaryHeap::new();
    for i in 0..points.len() {
        for j in i + 1..points.len() {
            let d = euclidean_squared_distance(&points[i], &points[j]);
            heap.push(Priority::new_min((i, j), d));
        }
    }
    (points, heap)
}

fn euclidean_squared_distance(p1: &Vec<i64>, p2: &Vec<i64>) -> i64 {
    let dx = p1[0] - p2[0];
    let dy = p1[1] - p2[1];
    let dz = p1[2] - p2[2];
    dx * dx + dy * dy + dz * dz
}

#[test]
fn day8_1() {
    let input = "162,817,812
57,618,57
906,360,560
592,479,940
352,342,300
466,668,158
542,29,236
431,825,988
739,650,466
52,470,668
216,146,977
819,987,18
117,168,530
805,96,715
346,949,466
970,615,88
941,993,340
862,61,35
984,92,344
425,690,689";
    let result = parse_and_solve_1(input.lines().map(|s| s.to_owned()), 10);
    assert_eq!(result, 40);
}

#[test]
fn day8_2() {
    let input = "162,817,812
57,618,57
906,360,560
592,479,940
352,342,300
466,668,158
542,29,236
431,825,988
739,650,466
52,470,668
216,146,977
819,987,18
117,168,530
805,96,715
346,949,466
970,615,88
941,993,340
862,61,35
984,92,344
425,690,689";
    let result = parse_and_solve_2(input.lines().map(|s| s.to_owned()));
    assert_eq!(result, 25272);
}
