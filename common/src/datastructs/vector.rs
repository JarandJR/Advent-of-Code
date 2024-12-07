use super::vec2::Vec2;

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
