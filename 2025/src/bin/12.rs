use common::{datastructs::grid::Grid, io::parse_into_lines_automatic};

fn main() {
    dbg!(parse_and_solve(parse_into_lines_automatic("12")));
}

fn parse_and_solve(line_iter: impl Iterator<Item = String>) -> usize {
    let (presents, regions) = parse(line_iter);
    // This works for some reason ¯\_(ツ)_/¯
    regions
        .iter()
        .filter(|r| {
            r.quantity
                .iter()
                .enumerate()
                .map(|(i, num_present)| {
                    presents[i]
                        .grid
                        .iter()
                        .map(|row| row.iter().sum::<usize>())
                        .sum::<usize>()
                        * num_present
                })
                .sum::<usize>()
                < r.height * r.width
        })
        .count()
}

fn parse(mut line_iter: impl Iterator<Item = String>) -> (Vec<Present>, Vec<Region>) {
    let mut presents = Vec::new();
    let mut regions = Vec::new();
    while let Some(line) = line_iter.next() {
        if line.is_empty() {
            continue;
        }
        if line.contains("x") {
            regions.push(line.into());
        } else {
            let mut grid = Vec::new();
            for _ in 0..3 {
                grid.push(
                    line_iter
                        .next()
                        .unwrap()
                        .chars()
                        .map(|c| match c {
                            '#' => 1,
                            _ => 0,
                        })
                        .collect::<Vec<usize>>(),
                );
            }
            presents.push(Present::new(grid, presents.len()));
        }
    }
    (presents, regions)
}

struct Region {
    width: usize,
    height: usize,
    quantity: Vec<usize>,
}

impl From<String> for Region {
    fn from(value: String) -> Self {
        let (size, quantity) = value.split_once(" ").unwrap();
        let (width, height) = size
            .split_once("x")
            .map(|(width, height)| {
                let width = width.parse::<usize>().unwrap();
                let height = height[..height.len() - 1].parse::<usize>().unwrap();
                (width, height)
            })
            .unwrap();
        let quantity = quantity
            .split(" ")
            .filter(|s| !s.is_empty())
            .map(|n| n.parse::<usize>().unwrap())
            .collect::<Vec<usize>>();
        Self {
            width,
            height,
            quantity,
        }
    }
}

struct Present {
    id: usize,
    grid: Grid<usize>,
}

impl Present {
    fn new(grid: Vec<Vec<usize>>, id: usize) -> Self {
        let grid = Grid::from_grid(grid);
        Self { id, grid }
    }
}

impl std::fmt::Debug for Present {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Present ID: {}", self.id)?;
        writeln!(f, "{:?}", self.grid)
    }
}

impl std::fmt::Debug for Region {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}x{} [", self.width, self.height)?;
        for q in &self.quantity {
            write!(f, "{}, ", q)?;
        }
        write!(f, "]")
    }
}

#[test]
fn day12_1() {
    let input = "0:
###
##.
##.

1:
###
##.
.##

2:
.##
###
##.

3:
##.
###
##.

4:
###
#..
###

5:
###
.#.
###

4x4: 0 0 0 0 2 0
12x5: 1 0 1 0 2 2
12x5: 1 0 1 0 3 2";
    let result = parse_and_solve(input.lines().map(|s| s.to_owned()));
    assert_eq!(result, 2);
}
