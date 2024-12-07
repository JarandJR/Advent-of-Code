use chrono::{Datelike, Local};
use std::{
    fs::File,
    io::{BufRead, BufReader},
};

pub const NEW_LINE: u8 = b'\n';
pub const DIRECTIONS: [(i32, i32); 8] = [
    // North
    (-1, 0),
    // South
    (1, 0),
    // West
    (0, -1),
    // East
    (0, 1),
    // North-Vest
    (-1, -1),
    // North-East
    (-1, 1),
    // South-Vest
    (1, -1),
    // South-East
    (1, 1),
];

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
pub struct Vec2 {
    pub x: i32,
    pub y: i32,
}

impl Vec2 {
    pub const NORTH: Self = Self { x: 0, y: -1 };
    pub const SOUTH: Self = Self { x: 0, y: 1 };
    pub const EAST: Self = Self { x: 1, y: 0 };
    pub const WEST: Self = Self { x: -1, y: 0 };

    pub const NORTH_EAST: Self = Self { x: 1, y: -1 };
    pub const NORTH_WEST: Self = Self { x: -1, y: -1 };
    pub const SOUTH_EAST: Self = Self { x: 1, y: 1 };
    pub const SOUTH_WEST: Self = Self { x: -1, y: 1 };

    pub const EIGHT_CONNECTEDNESS: [Self; 8] = [
        Self::NORTH,
        Self::NORTH_EAST,
        Self::EAST,
        Self::SOUTH_EAST,
        Self::SOUTH,
        Self::SOUTH_WEST,
        Self::WEST,
        Self::NORTH_WEST,
    ];

    pub const FOUR_CONNECTEDNESS: [Self; 4] = [Self::NORTH, Self::EAST, Self::SOUTH, Self::WEST];

    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    pub fn from_point(x: usize, y: usize) -> Self {
        Self::new(x as i32, y as i32)
    }

    pub fn from_row_column(row: usize, column: usize) -> Self {
        Self::new(column as i32, row as i32)
    }

    pub fn row(&self) -> usize {
        assert!(self.y >= 0, "y can't be negative");
        self.y as usize
    }

    pub fn column(&self) -> usize {
        assert!(self.x >= 0, "x can't be negative");
        self.x as usize
    }

    pub fn move_to(self, v: Vector) -> Self {
        self + v.as_position()
    }

    pub fn to_index(&self, columns: i32) -> i32 {
        self.x + self.y * columns
    }
}

impl From<(usize, usize)> for Vec2 {
    fn from(value: (usize, usize)) -> Self {
        Self::new(value.0 as i32, value.1 as i32)
    }
}

impl std::ops::Add for Vec2 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self::new(self.x + rhs.x, self.y + rhs.y)
    }
}

impl std::ops::AddAssign for Vec2 {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl std::ops::SubAssign for Vec2 {
    fn sub_assign(&mut self, rhs: Self) {
        self.x -= rhs.x;
        self.y -= rhs.y;
    }
}

pub struct Vector {
    pub direction: Vec2,
    pub len: i32,
}

impl Vector {
    pub fn new(len: i32, direction: Vec2) -> Self {
        Self { len, direction }
    }

    pub fn north(len: i32) -> Self {
        Self::new(len, Vec2::NORTH)
    }

    pub fn south(len: i32) -> Self {
        Self::new(len, Vec2::SOUTH)
    }

    pub fn east(len: i32) -> Self {
        Self::new(len, Vec2::EAST)
    }

    pub fn west(len: i32) -> Self {
        Self::new(len, Vec2::WEST)
    }

    pub fn north_east(len: i32) -> Self {
        Self::new(len, Vec2::NORTH_EAST)
    }

    pub fn north_west(len: i32) -> Self {
        Self::new(len, Vec2::NORTH_WEST)
    }

    pub fn south_west(len: i32) -> Self {
        Self::new(len, Vec2::SOUTH_WEST)
    }

    pub fn south_east(len: i32) -> Self {
        Self::new(len, Vec2::SOUTH_EAST)
    }

    pub fn as_position(&self) -> Vec2 {
        Vec2 {
            x: self.direction.x * self.len,
            y: self.direction.y * self.len,
        }
    }

    pub fn to_index(&self, columns: i32) -> i32 {
        self.as_position().to_index(columns)
    }

    pub fn scale(&mut self, factor: i32) -> &mut Self {
        self.len *= factor;
        self
    }

    pub fn invert(&mut self) -> &mut Self {
        self.direction = Vec2::new(-self.direction.x, -self.direction.y);
        self
    }
}

pub struct Grid {
    grid: String,
    pub rows: usize,
    pub columns: usize,
}

impl Grid {
    /// Creates a grid of a string
    pub fn new(input: String, rows: usize, columns: usize) -> Self {
        Self {
            grid: input,
            rows,
            columns,
        }
    }

    /// Creates an slice of the grid
    fn slice(&self, start: usize, end: usize) -> &str {
        &self.grid[start..=end]
    }

    /// Slices and returns an iterator from a point in a given direction.
    /// This returns an empty iterator if the slice is outside the grid.
    pub fn slice_iter<'a>(
        &'a self,
        from: &Vec2,
        dir: &Vector,
    ) -> Box<dyn Iterator<Item = char> + 'a> {
        let start = from.to_index(self.columns as i32);
        let step = dir.to_index(self.columns as i32);
        let end = start + step;
        let step = (step.abs().checked_div(dir.len)).unwrap_or(0) as usize;
        if self.check_slice(start, end) {
            return Box::new("".chars());
        }
        let start = start as usize;
        let end = end as usize;
        if end < start {
            let reversed = self.slice(end, start).chars().rev().collect::<Vec<_>>();
            return Box::new(reversed.into_iter().step_by(step));
        }
        Box::new(self.slice(start, end).chars().step_by(step))
    }

    pub fn slice_end_iter<'a>(
        &'a self,
        from: &Vec2,
        dir: &Vec2,
    ) -> Box<dyn Iterator<Item = char> + 'a> {
        let len = if dir.x == 0 {
            // 0, 0
            if dir.y == 0 {
                0
            // 0, -1
            } else if dir.y < 0 {
                from.y
            // 0, 1
            } else {
                self.rows as i32 - from.y - 1
            }
        } else if dir.x < 0 {
            // -1, 0
            if dir.y == 0 {
                from.x
            // -1, -1
            } else if dir.y < 0 {
                std::cmp::min(from.x, from.y)
            // -1, 1
            } else {
                std::cmp::min(self.rows as i32 - from.y - 1, from.x)
            }
        } else {
            // 1, 0
            if dir.y == 0 {
                self.columns as i32 - from.x - 1
            // 1, -1
            } else if dir.y < 0 {
                std::cmp::min(self.columns as i32 - from.x - 1, from.y)
            // 1, 1
            } else {
                std::cmp::min(self.rows as i32 - from.y, self.columns as i32 - from.x) - 1
            }
        };
        self.slice_iter(from, &Vector::new(len as i32, *dir))
    }

    fn check_slice(&self, start: i32, end: i32) -> bool {
        let max = (self.rows * self.columns) as i32;
        start < 0 || max < start || end < 0 || max < end
    }
}

impl<'a, T> From<T> for Grid
where
    T: Iterator<Item = &'a str>,
{
    fn from(value: T) -> Self {
        let mut rows = 0;
        let mut columns = 0;
        let grid = value
            .map(|str| {
                columns = str.len();
                rows += 1;
                str
            })
            .collect::<String>();
        assert!(grid.len() > 0, "Provided empty iterator");
        Self::new(grid, rows, columns)
    }
}

pub fn parse_into_byte_lines_automatic(day: &str) -> Option<impl Iterator<Item = Vec<u8>>> {
    let today = Local::now();
    assert!(
        today.month() == 12,
        "This function can only be used in desember"
    );
    parse_into_byte_lines(today.year(), day)
}

pub fn parse_into_byte_lines(year: i32, day: &str) -> Option<impl Iterator<Item = Vec<u8>>> {
    let curnt_dir = std::env::current_dir()
        .expect("Could not get current direction")
        .display()
        .to_string();
    let path = if curnt_dir.contains(year.to_string().as_str()) {
        String::new()
    } else {
        format!("{}/", year)
    };
    if let Ok(file) = File::open(format!("{}inputs/{}.txt", path, day)) {
        let reader = BufReader::new(file);
        return Some(reader.split(NEW_LINE).flat_map(|line| line.ok()));
    }
    None
}

pub fn parse_into_lines_automatic(day: &str) -> Option<impl Iterator<Item = String>> {
    let today = Local::now();
    assert!(
        today.month() == 12,
        "This function can only be used in desember"
    );
    parse_into_lines(today.year(), day)
}

pub fn parse_into_lines(year: i32, day: &str) -> Option<impl Iterator<Item = String>> {
    use std::io::BufRead;
    let curnt_dir = std::env::current_dir()
        .expect("Could not get current direction")
        .display()
        .to_string();
    let path = if curnt_dir.contains(year.to_string().as_str()) {
        String::new()
    } else {
        format!("{}/", year)
    };
    if let Ok(file) = std::fs::File::open(format!("{}inputs/{}.txt", path, day)) {
        let reader = std::io::BufReader::new(file);
        return Some(reader.lines().flat_map(|line| line.ok()));
    }
    None
}
