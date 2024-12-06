use aoc2023::read_file_string;

fn main() {
    let data = read_file_string("inputs/12.txt").unwrap();
    println!("Result: {}", solve(data));
}

fn solve(data: String) -> usize {
    data.lines()
        .map(|line| get_arrangements(line))
        .collect::<Vec<usize>>()
        .iter()
        .fold(0, |acc, f| acc + f)
}

#[derive(Debug, PartialEq, Clone)]
enum Records {
    Operational,
    Damaged,
    Unknown,
}

impl From<char> for Records {
    fn from(c: char) -> Self {
        match c {
            '#' => Records::Damaged,
            '.' => Records::Operational,
            '?' => Records::Unknown,
            _ => panic!("Invalid char"),
        }
    }
}

fn get_arrangements(line: &str) -> usize {
    let (records, parts) = line.split_once(' ').unwrap();
    let mut records = records
        .chars()
        .map(|c| Records::from(c))
        .collect::<Vec<Records>>();
    let parts = parts
        .split(',')
        .map(|s| s.parse::<usize>().unwrap())
        .collect::<Vec<usize>>();

    generate_combinations(&mut records)
        .iter()
        .filter(|a| is_valid(a, &parts))
        .count()
}

fn generate_combinations(r: &Vec<Records>) -> Vec<Vec<Records>> {
    let mut combs = Vec::new();

    let mut stack = Vec::new();
    stack.push((0, Vec::new()));
    while let Some((at, mut cur)) = stack.pop() {
        for i in at..r.len() {
            match r[i] {
                Records::Damaged => cur.push(Records::Damaged),
                Records::Operational => cur.push(Records::Operational),
                Records::Unknown => {
                    let mut next = cur.clone();
                    cur.push(Records::Operational);
                    next.push(Records::Damaged);
                    stack.push((i + 1, next));
                }
            }
        }
        if cur.len() == r.len() {
            combs.push(cur);
            continue;
        }
    }
    combs
}

fn is_valid(arr: &Vec<Records>, parts: &Vec<usize>) -> bool {
    let blocks = arr
        .split(|a| a == &Records::Operational)
        .filter(|a| a.len() > 0)
        .collect::<Vec<&[Records]>>();

    if blocks.len() != parts.len() {
        return false;
    }

    let mut i = 0;
    blocks.iter().all(|a| {
        let len = a.len();
        let part = parts[i];
        i += 1;
        len == part
    })
}

macro_rules! test_arrangements {
    ($name:ident, $input:expr, $expected:expr) => {
        #[test]
        fn $name() {
            assert_eq!($expected, get_arrangements($input));
        }
    };
}
test_arrangements!(test_12_1a_1, "???.### 1,1,3", 1);
test_arrangements!(test_12_1a_2, ".??..??...?##. 1,1,3", 4);
test_arrangements!(test_12_1a_3, "?#?#?#?#?#?#?#? 1,3,1,6", 1);
test_arrangements!(test_12_1a_4, "????.#...#... 4,1,1", 1);
test_arrangements!(test_12_1a_5, "????.######..#####. 1,6,5", 4);
test_arrangements!(test_12_1a_6, "?###???????? 3,2,1", 10);

#[test]
fn test_12_1b() {
    assert_eq!(
        21,
        solve(
            "???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1"
                .to_string()
        )
    );
}
