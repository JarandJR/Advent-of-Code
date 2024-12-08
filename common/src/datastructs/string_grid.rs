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
        let step = dir.dir_to_step(self.columns as i32).abs() as usize;
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
        let abs_dir = dir.abs();
        let len = Vec2::from(if dir.x == 0 {
            // 0, 0
            if dir.y == 0 {
                (0, 0)
            // 0, -1
            } else if dir.y < 0 {
                (0, from.y / abs_dir.y)
            // 0, 1
            } else {
                (0, (self.rows as i32 - from.y - 1) / abs_dir.y)
            }
        } else if dir.x < 0 {
            // -1, 0
            if dir.y == 0 {
                (from.x / abs_dir.x, 0)
            // -1, -1
            } else if dir.y < 0 {
                (from.x / abs_dir.x, from.y / abs_dir.y)
            // -1, 1
            } else {
                (
                    (self.rows as i32 - from.y - 1) / abs_dir.y,
                    from.x / abs_dir.x,
                )
            }
        } else {
            // 1, 0
            if dir.y == 0 {
                ((self.columns as i32 - from.x - 1) / abs_dir.x, 0)
            // 1, -1
            } else if dir.y < 0 {
                (
                    (self.columns as i32 - from.x - 1) / abs_dir.x,
                    from.y / abs_dir.y,
                )
            // 1, 1
            } else {
                (
                    (self.rows as i32 - from.y - 1) / abs_dir.y,
                    (self.columns as i32 - from.x - 1) / abs_dir.x,
                )
            }
        });
        self.slice_iter(from, &Vector::new(len, *dir))
    }

    fn check_slice(&self, start: i32, end: i32) -> bool {
        let max = (self.rows * self.columns) as i32;
        start < 0 || max < start || end < 0 || max < end
    }
}

impl<'a, T> From<T> for Grid
where
    T: IntoIterator,
    T::Item: Into<String>,
{
    fn from(value: T) -> Self {
        let mut rows = 0;
        let mut columns = 0;
        let grid = value
            .into_iter()
            .map(|str| {
                let str = str.into();
                columns = str.len();
                rows += 1;
                str
            })
            .collect::<String>();
        assert!(grid.len() > 0, "Provided empty iterator");
        Self::new(grid, rows, columns)
    }
}
