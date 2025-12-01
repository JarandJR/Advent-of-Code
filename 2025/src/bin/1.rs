use common::io::parse_into_lines_automatic;

fn main() {
    dbg!(parse_and_solve_1(parse_into_lines_automatic("1")));
    dbg!(parse_and_solve_2(parse_into_lines_automatic("1")));
}

fn parse_and_solve_1(line_iter: impl Iterator<Item = String>) -> usize {
    line_iter
        .fold((0, 50), |acc, line| {
            let (mut count, dial) = acc;
            let rotation = line
                .chars()
                .take_while(|c| c.is_ascii_alphabetic())
                .collect::<String>();
            let digit = line
                .chars()
                .skip(1)
                .collect::<String>()
                .parse::<i32>()
                .unwrap();
            let dial = match rotation.as_str() {
                "L" => (dial - digit).rem_euclid(100),
                "R" => (dial + digit).rem_euclid(100),
                _ => unreachable!(),
            };
            dial.eq(&0).then(|| count += 1);
            (count, dial)
        })
        .0
}

fn parse_and_solve_2(line_iter: impl Iterator<Item = String>) -> usize {
    line_iter
        .fold((0, Dial::new()), |acc, line| {
            let (mut count, mut dial) = acc;
            let rotation = line
                .chars()
                .take_while(|c| c.is_ascii_alphabetic())
                .collect::<String>();
            let digit = line
                .chars()
                .skip(1)
                .collect::<String>()
                .parse::<usize>()
                .unwrap();
            match rotation.as_str() {
                "L" => dial.reverse().take(digit),
                "R" => dial.forward().take(digit),
                _ => unreachable!(),
            }
            .for_each(|d| {
                d.eq(&0).then(|| count += 1);
            });
            (count, dial)
        })
        .0
}

pub struct Dial {
    len: i32,
    pointing_at: i32,
    forward: bool,
}

impl Dial {
    pub fn new() -> Self {
        Self {
            len: 100,
            pointing_at: 50,
            forward: true,
        }
    }
    pub fn reverse(&mut self) -> &mut Self {
        self.forward = false;
        self
    }

    pub fn forward(&mut self) -> &mut Self {
        self.forward = true;
        self
    }
}

impl Iterator for Dial {
    type Item = i32;
    fn next(&mut self) -> Option<Self::Item> {
        if self.forward {
            self.pointing_at = (self.pointing_at + 1).rem_euclid(self.len);
        } else {
            self.pointing_at = (self.pointing_at - 1).rem_euclid(self.len);
        }
        Some(self.pointing_at)
    }
}

#[test]
fn day1_1_1() {
    let input = "L68
L30
R48
L5
R60
L55
L1
L99
R14
L82";
    let result = parse_and_solve_1(input.lines().map(|s| s.to_owned()));
    assert_eq!(result, 3);
}

#[test]
fn day1_1_2() {
    let input = "L68
L30
R48
L5
R60
L55
L1
L99
R14
L82";
    let result = parse_and_solve_2(input.lines().map(|s| s.to_owned()));
    assert_eq!(result, 6);
}
