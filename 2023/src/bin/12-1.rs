use aoc2023::read_file_string;

fn main() {
    let data = read_file_string("inputs/11.txt").unwrap();
    println!("Result: {}", solve(data));
}

fn solve(data: String) -> usize {
    data
    .lines()
    .map(|line| get_arrangements(line))
    .collect::<Vec<usize>>()
    .iter()
    .fold(0, |acc, f| acc + f)
}

#[derive(Debug, PartialEq, Eq)]
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
    let records = records.chars().map(|c| Records::from(c)).collect::<Vec<Records>>();
    let parts = parts.split(',').map(|s| s.parse::<usize>().unwrap()).collect::<Vec<usize>>();
    dbg!(records, parts);
    todo!()
}

macro_rules! test_arrangements {
    ($name:ident, $input:expr, $expected:expr) => {
        #[test]
        fn $name() {
            assert_eq!($expected, get_arrangements($input));
        }
    };
}

test_arrangements!(test_12_1a_1, "???.### 1,1,3", 1);// 1 "#.?.###" n = 1, k = 1
test_arrangements!(test_12_1a_2, ".??..??...?##. 1,1,3", 4); // n = 2, k = 1, x2 // n = 4 and k = 1
test_arrangements!(test_12_1a_3, "?#?#?#?#?#?#?#? 1,3,1,6", 1); // "#######.#?#?#?#?" -> "#######.###.#.#."
test_arrangements!(test_12_1a_4, "????.#...#... 4,1,1", 1); // n = k = 1
test_arrangements!(test_12_1a_5, "????.######..#####. 1,6,5", 4);// n = 4, k = 1
test_arrangements!(test_12_1a_6, "?###???????? 3,2,1", 10); // ".###.#.?????" n = 5, k = 2
                                                            // ".###.##.????" n=4, k=1 WRONG
                                                            // Must remove the ones not possible else where and then take the largest

#[test]
fn test_12_1b() {
    assert_eq!(21, solve("???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1".to_string()));
}

