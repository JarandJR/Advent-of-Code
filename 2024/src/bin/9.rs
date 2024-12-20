use common::io::parse_into_lines_automatic;

fn main() {
    dbg!(parse_and_solve1(parse_into_lines_automatic("9")));
    dbg!(parse_and_solve2(parse_into_lines_automatic("9")));
}

#[derive(Clone, Copy)]
enum Block {
    File(usize),
    Space,
}

impl std::fmt::Debug for Block {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Block::File(num) => write!(f, "{}", num),
            Block::Space => write!(f, "."),
        }
    }
}

fn parse_and_solve1(line_iter: impl Iterator<Item = String>) -> usize {
    let mut alternate = false;
    let mut file_id = 0;
    let mut disk_map = line_iter
        .flat_map(|line| {
            line.chars()
                .flat_map(|c| {
                    let number = c.to_digit(10).unwrap() as usize;
                    let block = if alternate {
                        file_id += 1;
                        vec![Block::Space; number]
                    } else {
                        vec![Block::File(file_id); number]
                    };
                    alternate = !alternate;
                    block
                })
                .collect::<Vec<Block>>()
        })
        .collect::<Vec<Block>>();
    let mut pos = 0;
    let mut pos_rev = disk_map.len() - 1;
    loop {
        if pos >= pos_rev || pos >= disk_map.len() {
            break;
        }
        match disk_map[pos] {
            Block::File(_) => {
                pos += 1;
                continue;
            }
            _ => {}
        };

        let block = disk_map[pos_rev];
        match block {
            Block::Space => {
                pos_rev -= 1;
                continue;
            }
            _ => {
                disk_map[pos_rev] = Block::Space;
                disk_map[pos] = block;
            }
        }
    }
    disk_map
        .iter()
        .enumerate()
        .fold(0, |acc, (i, block)| match block {
            Block::File(id) => acc + i * id,
            Block::Space => acc,
        })
}

fn parse_and_solve2(line_iter: impl Iterator<Item = String>) -> usize {
    let mut alternate = false;
    let mut file_id = 0;
    let mut disk_map = line_iter
        .flat_map(|line| {
            line.chars()
                .map(|c| {
                    let number = c.to_digit(10).unwrap() as usize;
                    let block = if alternate {
                        file_id += 1;
                        vec![Block::Space; number]
                    } else {
                        vec![Block::File(file_id); number]
                    };
                    alternate = !alternate;
                    block
                })
                .filter(|b| !b.is_empty())
                .collect::<Vec<Vec<Block>>>()
        })
        .collect::<Vec<Vec<Block>>>();
    let mut pos_rev = disk_map.len() - 1;
    let mut start_pos = 0;
    loop {
        if pos_rev < 1 {
            break;
        }
        let block = disk_map[pos_rev][0];
        match block {
            Block::Space => {
                pos_rev -= 1;
                continue;
            }
            _ => {
                let mut pos = start_pos;
                loop {
                    if pos >= pos_rev || pos >= disk_map.len() {
                        // No place to move
                        pos_rev -= 1;
                        break;
                    }
                    let space_to_fill = disk_map[pos]
                        .iter()
                        .filter(|bl| match bl {
                            Block::Space => true,
                            _ => false,
                        })
                        .count();
                    if space_to_fill == 0 && pos == start_pos {
                        // Skip
                        start_pos += 1;
                    }
                    if disk_map[pos_rev].len() <= space_to_fill {
                        let bl = disk_map[pos_rev][0];
                        let mut rev_c = 0;
                        for i in 0..disk_map[pos].len() {
                            if disk_map[pos_rev].len() <= rev_c {
                                break;
                            }
                            match disk_map[pos][i] {
                                Block::Space => {
                                    disk_map[pos_rev][rev_c] = Block::Space;
                                    disk_map[pos][i] = bl;
                                    rev_c += 1;
                                }
                                _ => {
                                    continue;
                                }
                            };
                        }
                        break;
                    }
                    pos += 1;
                }
            }
        }
    }
    disk_map
        .iter()
        .flat_map(|a| a)
        .enumerate()
        .fold(0, |acc, (i, block)| match block {
            Block::File(id) => acc + i * id,
            Block::Space => acc,
        })
}

#[test]
fn day9_1() {
    let input = "2333133121414131402";
    let result = parse_and_solve1(input.lines().map(|s| s.to_owned()));
    assert_eq!(result, 1928);
}

#[test]
fn day9_2() {
    let input = "2333133121414131402";
    let result = parse_and_solve2(input.lines().map(|s| s.to_owned()));
    assert_eq!(result, 2858);
}
