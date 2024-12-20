use common::io::parse_into_lines_automatic;
use itertools::Itertools;
use num_rational::Rational64;

type Equation = (
    num_rational::Ratio<i64>,
    num_rational::Ratio<i64>,
    num_rational::Ratio<i64>,
);

type Tupple = (num_rational::Ratio<i64>, num_rational::Ratio<i64>);

const TOKEN_COST_A: i64 = 3;
const TOKEN_COST_B: i64 = 1;
const PRICE_ADD: i64 = 10_000_000_000_000;

fn main() {
    dbg!(parse_and_solve_part_1(parse_into_lines_automatic("13")));
    dbg!(parse_and_solve_part_2(parse_into_lines_automatic("13")));
}

fn parse_and_solve_part_1(line_iter: impl Iterator<Item = String>) -> usize {
    let mut claw_machines = parse(line_iter);
    let mut sum = 0;
    while let Some((eq1, eq2)) = claw_machines.pop() {
        if let Some((press_a, press_b)) = cramer(eq1, eq2) {
            let a_is_int = press_a.denom() == &1;
            let b_is_int = press_b.denom() == &1;
            if (0..100).contains(press_a.numer())
                && a_is_int
                && (0..100).contains(press_b.numer())
                && b_is_int
            {
                sum += press_a.to_integer() * TOKEN_COST_A + press_b.to_integer() * TOKEN_COST_B;
            }
        }
    }
    sum as usize
}

fn parse_and_solve_part_2(line_iter: impl Iterator<Item = String>) -> usize {
    let mut claw_machines = parse(line_iter);
    let mut sum = 0;
    while let Some((mut eq1, mut eq2)) = claw_machines.pop() {
        eq1.2 += PRICE_ADD;
        eq2.2 += PRICE_ADD;
        if let Some((press_a, press_b)) = cramer(eq1, eq2) {
            let a_is_int = press_a.denom() == &1;
            let b_is_int = press_b.denom() == &1;
            if a_is_int && b_is_int {
                sum += press_a.to_integer() * TOKEN_COST_A + press_b.to_integer() * TOKEN_COST_B;
            }
        }
    }
    sum as usize
}

/// Parses the input
fn parse(mut line_iter: impl Iterator<Item = String>) -> Vec<(Equation, Equation)> {
    let mut claw_machines = Vec::new();
    let mut crnt_machine = Vec::new();
    while let Some(line) = line_iter.next() {
        if line.is_empty() {
            let eqs = eqs_from_machine(&crnt_machine);
            claw_machines.push(eqs);
            crnt_machine.clear();
            continue;
        }
        let line_nums: Tupple = line
            .split(',')
            .filter_map(|str| {
                str.chars()
                    .filter(|&c| c.is_numeric())
                    .collect::<String>()
                    .parse::<i64>()
                    .ok()
                    .map(|n| Rational64::from_integer(n))
            })
            .collect_tuple()
            .unwrap();
        crnt_machine.push(line_nums);
    }
    // The last machine
    let eqs = eqs_from_machine(&crnt_machine);
    claw_machines.push(eqs);
    claw_machines
}

/// Constructs the equations for a claw machine
fn eqs_from_machine(crnt_machine: &Vec<Tupple>) -> (Equation, Equation) {
    let a = crnt_machine[0];
    let b = crnt_machine[1];
    let prize = crnt_machine[2];
    let eq1 = (a.0, b.0, prize.0);
    let eq2 = (a.1, b.1, prize.1);
    (eq1, eq2)
}

/// Cramer's rule for solution of a linear equation system
fn cramer(eq1: Equation, eq2: Equation) -> Option<Tupple> {
    /*
        2x2 system of linear equation
        a*x + b*y = e
        c*x + d*y = f

        Matrix Form:
        [a  b] [x]   = [e]
        [c  d] [y]     [f]
    */
    let (a, b, e) = eq1;
    let (c, d, f) = eq2;
    let determinant = a * d - b * c;
    if determinant != Rational64::from(0) {
        let x = (e * d - b * f) / determinant;
        let y = (a * f - e * c) / determinant;
        return Some((x, y));
    }
    // Determinant is zero: there are no solutions to the equation system
    None
}

#[test]
fn day13_1() {
    let input = "Button A: X+94, Y+34
Button B: X+22, Y+67
Prize: X=8400, Y=5400

Button A: X+26, Y+66
Button B: X+67, Y+21
Prize: X=12748, Y=12176

Button A: X+17, Y+86
Button B: X+84, Y+37
Prize: X=7870, Y=6450

Button A: X+69, Y+23
Button B: X+27, Y+71
Prize: X=18641, Y=10279";
    let result = parse_and_solve_part_1(input.lines().map(|s| s.to_owned()));
    assert_eq!(result, 480);
}

#[test]
fn day13_2() {
    let input = "Button A: X+94, Y+34
Button B: X+22, Y+67
Prize: X=8400, Y=5400

Button A: X+26, Y+66
Button B: X+67, Y+21
Prize: X=12748, Y=12176

Button A: X+17, Y+86
Button B: X+84, Y+37
Prize: X=7870, Y=6450

Button A: X+69, Y+23
Button B: X+27, Y+71
Prize: X=18641, Y=10279";
    let result = parse_and_solve_part_2(input.lines().map(|s| s.to_owned()));
    assert_eq!(result, 875318608908);
}
