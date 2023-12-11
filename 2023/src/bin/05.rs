#![allow(dead_code)]

use std::f32::INFINITY;

use aoc2023::{read_file_string, parse_line, Parse, get_numbers_on_line};

fn main() {
    println!("Result {}", solve(read_file_string("inputs/05.txt").unwrap(), false));
}

fn solve(data: String, first: bool) -> i64 {
    let al: Almanac = parse_line(&data);
    if first {
        return task_1(&al);
    }
    task_2(&al)
}

fn task_2(al: &Almanac) -> i64 {
    let mut i = 1;
    let mut lowest_location = INFINITY as i64;
    for s in &al.seeds.chunks(2).collect::<Vec<_>>() {
        println!("Running pair: {i}");
        let start = s[0];
        let end = s[1] + start;
        for i in start..end {
            let loc = get_location(i, &al.maps);
            if loc < lowest_location {
                lowest_location = loc;
            }
        }
        i += 1;
    }
    lowest_location
}

fn task_1(al: &Almanac) -> i64 {
    let mut lowest_location = INFINITY as i64;
    for s in &al.seeds {
        let loc = get_location(*s, &al.maps);
        if loc < lowest_location {
            lowest_location = loc;
        }
    }
    lowest_location
}

fn get_location(seed: i64, maps: &Vec<Vec<Vec<i64>>>) -> i64 {
    let mut at = seed;
    for transelator in maps {
        for map in transelator {
            let start = map[from(Map::SourStart)];
            let range = map[from(Map::Range)];
            if at >= start && at < start + range {
                at = map[from(Map::DestStart)] + (at - start);
                break;
            }
        }
    }
    at
}

struct Almanac {
    seeds: Vec<i64>,
    maps: Vec<Vec<Vec<i64>>>,
}

impl Parse for Almanac {
    fn parse(data: &str) -> Self {
        let mut maps: Vec<Vec<Vec<i64>>> = Vec::new();
        let mut it = data.lines().into_iter();
        let seeds = get_numbers_on_line(it.next().unwrap());
        let mut i = 0;

        maps.push(Vec::new());
        while let Some(l) = it.next() {
            let line_numbers = get_numbers_on_line(l);
            if line_numbers.is_empty() && !maps[i].is_empty(){
                i += 1;
                maps.push(Vec::new());
                continue;
            }
            if !line_numbers.is_empty() {
                maps[i].push(line_numbers);
            }
        }
        Self { seeds, maps }
    }
}

enum Map {
    DestStart,
    SourStart,
    Range,
}

fn from(item: Map) -> usize {
    match item {
        Map::DestStart => 0,
        Map::SourStart => 1,
        Map::Range => 2,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_05_1() {
        assert_eq!(35, solve(TEST_STRING.to_string(), true));
    }

    #[test]
    fn test_05_2() {
        assert_eq!(46, solve(TEST_STRING.to_string(), false));
    }
}

const TEST_STRING: &str = "seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4";
