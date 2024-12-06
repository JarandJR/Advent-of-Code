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

#[derive(Debug)]
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
        Self::SOUTH,
        Self::EAST,
        Self::WEST,
        Self::NORTH_EAST,
        Self::NORTH_WEST,
        Self::SOUTH_EAST,
        Self::SOUTH_WEST,
    ];

    pub const FOUR_CONNECTEDNESS: [Self; 4] = [Self::NORTH, Self::SOUTH, Self::EAST, Self::WEST];

    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    pub fn from_row_column(row: usize, column: usize) -> Self {
        Self::new(column as i32, row as i32)
    }

    pub fn row(&self) -> usize {
        assert!(self.y >= 0, "y must be non-negative");
        self.y as usize
    }

    pub fn column(&self) -> usize {
        assert!(self.x >= 0, "x must be non-negative");
        self.x as usize
    }

    pub fn move_to(self, v: Vector) -> Self{
        self + v.to_position()
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

    pub fn to_position(&self) -> Vec2 {
        Vec2 {
            x: self.direction.x * self.len,
            y: self.direction.y * self.len,
        }
    }

    pub fn scale(&mut self, factor: i32) -> &mut Self {
        self.len *= factor;
        self
    }

    pub fn invert(&mut self) -> &mut Self {
        self.direction =  Vec2::new(-self.direction.x, -self.direction.y);
        self
    }
}

pub fn parse_into_byte_lines(day: &str) -> Option<impl Iterator<Item = Vec<u8>>> {
    let curnt_dir = std::env::current_dir().expect("Could not get current direction").display().to_string();
    let path = if curnt_dir.contains("2024") { "" } else { "2024/" };
    if let Ok(file) = File::open(format!("{}inputs/{}.txt", path, day)) {
        let reader = BufReader::new(file);
        return Some(reader.split(NEW_LINE).flat_map(|line| line.ok()));
    }
    None
}

pub fn parse_into_lines(day: &str) -> Option<impl Iterator<Item = String>> {
    use std::io::BufRead;
    let curnt_dir = std::env::current_dir().expect("Could not get current direction").display().to_string();
    let path = if curnt_dir.contains("2024") { "" } else { "2024/" };
    if let Ok(file) = std::fs::File::open(format!("{}inputs/{}.txt", path, day)) {
        let reader = std::io::BufReader::new(file);
        return Some(reader.lines().flat_map(|line| line.ok()));
    }
    None
}
