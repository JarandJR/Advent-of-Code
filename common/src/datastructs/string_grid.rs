use std::ops::Index;

use super::{vec2::Vec2, vector::Vector};

#[derive(Debug)]
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

/// Helper function for the index implementation as it leakes memory.
/// See index documentation for example.
pub fn reclaime_leaked_memory(leaked: &[char]) {
    let ptr = leaked.as_ptr();
    let len = leaked.len();
    unsafe {
        let ptr = ptr as *mut char;
        let _reclamed_mem = Box::from_raw(std::ptr::slice_from_raw_parts_mut(ptr, len));
    }
}

impl Index<usize> for Grid {
    type Output = [char];

    /// This intenionally leaks memory and is only intende for
    /// small programs like solutions to AOC.
    ///
    /// Consider using the helper function 'reclaime_leaked_memory'
    /// if this is called several times throughout the program.
    ///
    /// /// # Example
    ///
    /// ```rust
    /// // Example where memory is intentionally leaked
    /// // and then reclaimed
    /// let grid = Grid::from("----------
    /// ----x-----
    /// --------y-")
    ///
    /// let leaked_slice: &[char] = &grid[0];
    /// reclaime_leaked_memory(leaked_slice);
    /// ```
    ///
    /// ```rust
    /// // Example where memory is leaked
    /// let grid = Grid::from("----------
    /// ----x-----
    /// --------y-")
    /// // Leaks data unable to be reclaimed
    /// let x: &char = grid[1][4];
    /// ```
    fn index(&self, index: usize) -> &Self::Output {
        let start = index * self.columns;
        let end = (index + 1) * self.columns;

        let row_str = &self.grid[start..end];
        let row_vec = row_str.chars().collect::<Vec<char>>();
        Box::leak(row_vec.into_boxed_slice())
    }
}

impl<'a> FromIterator<&'a str> for Grid {
    fn from_iter<I: IntoIterator<Item = &'a str>>(iter: I) -> Self {
        let mut rows = 0;
        let mut columns = 0;

        let grid_lines = iter
            .into_iter()
            .map(|line| {
                if rows == 0 {
                    columns = line.len();
                }
                rows += 1;
                line.to_string()
            })
            .collect::<Vec<String>>();

        let grid_string = grid_lines.join("");
        Grid::new(grid_string, rows, columns)
    }
}

impl<'a> From<&'a str> for Grid {
    fn from(input: &'a str) -> Self {
        let mut rows = 0;
        let mut columns = 0;
        let grid = input
            .lines()
            .map(|l| {
                if columns == 0 {
                    columns = l.len();
                }
                rows += 1;
                l
            })
            .collect::<String>();

        Grid::new(grid, rows, columns)
    }
}
