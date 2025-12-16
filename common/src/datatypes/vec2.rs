use super::vector::Vector;

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

    pub fn move_to(self, v: &Vector) -> Self {
        self + v.as_position()
    }

    pub fn to_index(&self, columns: i32) -> i32 {
        self.x + self.y * columns
    }

    pub fn abs(&self) -> Self {
        Vec2::new(self.x.abs(), self.y.abs())
    }

    pub fn sum(&self) -> i32 {
        self.x + self.y
    }

    pub fn inverse(&self) -> Self {
        Vec2::new(-self.x, -self.y)
    }

    pub fn rotate_90_clockwise(&self) -> Self {
        Vec2::new(-self.y, self.x)
    }

    pub fn rotate_90_counter_clockwise(&self) -> Self {
        Vec2::new(self.y, -self.x)
    }

    pub fn to_tupple(&self) -> (usize, usize) {
        assert!(self.x >= 0, "x can't be negative");
        assert!(self.y >= 0, "y can't be negative");
        (self.x as usize, self.y as usize)
    }
}

impl From<(usize, usize)> for Vec2 {
    fn from(value: (usize, usize)) -> Self {
        Self::new(value.0 as i32, value.1 as i32)
    }
}

impl From<(i32, i32)> for Vec2 {
    fn from(value: (i32, i32)) -> Self {
        Self::new(value.0, value.1)
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

impl std::ops::Sub for Vec2 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self::new(self.x - rhs.x, self.y - rhs.y)
    }
}

impl Default for Vec2 {
    fn default() -> Self {
        Self::new(1, 1)
    }
}
