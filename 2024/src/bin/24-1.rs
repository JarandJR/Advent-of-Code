use std::collections::HashMap;

use common::io::parse_into_lines_automatic;
use itertools::Itertools;

fn main() {
    dbg!(parse_and_solve(parse_into_lines_automatic("24")));
}

fn parse_and_solve(mut line_iter: impl Iterator<Item = String>) -> usize {
    let mut wires = line_iter
        .by_ref()
        .take_while(|line| !line.is_empty())
        .map(|line| {
            let tmp = line
                .split(&[' ', ':'])
                .filter(|s| !s.is_empty())
                .collect_vec();
            (tmp[0].to_owned(), tmp[1].parse::<u64>().unwrap())
        })
        .collect::<HashMap<String, u64>>();

    let instructions = line_iter.collect_vec();
    let mut processed = vec![false; instructions.len()];
    for (i, line) in instructions.iter().enumerate().cycle() {
        if processed.iter().all(|p| *p) {
            break;
        }
        let (source, instruction, operand, destination) = line
            .split(&[' ', '-', '>'])
            .filter(|str| !str.is_empty())
            .collect_tuple::<(&str, &str, &str, &str)>()
            .unwrap();
        if wires.contains_key(source) && wires.contains_key(operand) {
            let instruction = match instruction {
                "AND" => |a: u64, b: u64| a & b,
                "XOR" => |a: u64, b: u64| a ^ b,
                "OR" => |a: u64, b: u64| a | b,
                _ => panic!(),
            };
            let source = wires.get(source).unwrap();
            let operand = wires.get(operand).unwrap();
            wires.insert(destination.to_owned(), instruction(*source, *operand));
            processed[i] = true;
        }
    }
    wires
        .into_iter()
        .filter(|(k, _v)| k.starts_with("z"))
        .fold(0, |acc, (k, v)| {
            let bit_pos = k[1..].parse::<usize>().unwrap();
            acc | (v << bit_pos)
        }) as usize
}

#[test]
fn day24_1a() {
    let input = "x00: 1
x01: 1
x02: 1
y00: 0
y01: 1
y02: 0

x00 AND y00 -> z00
x01 XOR y01 -> z01
x02 OR y02 -> z02";
    let result = parse_and_solve(input.lines().map(|s| s.to_owned()));
    assert_eq!(result, 4);
}

#[test]
fn day24_1b() {
    let input = "x00: 1
x01: 0
x02: 1
x03: 1
x04: 0
y00: 1
y01: 1
y02: 1
y03: 1
y04: 1

ntg XOR fgs -> mjb
y02 OR x01 -> tnw
kwq OR kpj -> z05
x00 OR x03 -> fst
tgd XOR rvg -> z01
vdt OR tnw -> bfw
bfw AND frj -> z10
ffh OR nrd -> bqk
y00 AND y03 -> djm
y03 OR y00 -> psh
bqk OR frj -> z08
tnw OR fst -> frj
gnj AND tgd -> z11
bfw XOR mjb -> z00
x03 OR x00 -> vdt
gnj AND wpb -> z02
x04 AND y00 -> kjc
djm OR pbm -> qhw
nrd AND vdt -> hwm
kjc AND fst -> rvg
y04 OR y02 -> fgs
y01 AND x02 -> pbm
ntg OR kjc -> kwq
psh XOR fgs -> tgd
qhw XOR tgd -> z09
pbm OR djm -> kpj
x03 XOR y03 -> ffh
x00 XOR y04 -> ntg
bfw OR bqk -> z06
nrd XOR fgs -> wpb
frj XOR qhw -> z04
bqk OR frj -> z07
y03 OR x01 -> nrd
hwm AND bqk -> z03
tgd XOR rvg -> z12
tnw OR pbm -> gnj";
    let result = parse_and_solve(input.lines().map(|s| s.to_owned()));
    assert_eq!(result, 2024);
}
