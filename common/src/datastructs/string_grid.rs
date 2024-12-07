use super::{vec2::Vec2, vector::Vector};

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
