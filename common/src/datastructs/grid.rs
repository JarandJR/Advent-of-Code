use std::fmt::{Debug, Display};

use crate::datatypes::vec2::Vec2;

pub struct Grid<T> {
    grid: Vec<Vec<T>>,
    pub rows: usize,
    pub columns: usize,
}

impl<T> Grid<T> {
    pub fn new(grid: Vec<Vec<T>>, rows: usize, columns: usize) -> Self {
        Self {
            grid,
            rows,
            columns,
        }
    }

    pub fn from_grid(grid: Vec<Vec<T>>) -> Self {
        let rows = grid.len();
        let columns = grid.get(0).unwrap_or(&Vec::new()).len();
        Self {
            grid,
            rows,
            columns,
        }
    }

    pub fn four_connectedness<F>(&self, center: Vec2, predicate: F) -> Vec<Vec2>
    where
        F: Fn(&T) -> bool,
    {
        Vec2::FOUR_CONNECTEDNESS
            .into_iter()
            .map(|d| d + center)
            .filter(|p| self.contains_point(p))
            .filter(|p| predicate(&self[*p]))
            .collect::<Vec<Vec2>>()
    }

    pub fn eight_connectedness<F>(&self, center: Vec2, predicate: F) -> Vec<Vec2>
    where
        F: Fn(&T) -> bool,
    {
        Vec2::EIGHT_CONNECTEDNESS
            .into_iter()
            .map(|d| d + center)
            .filter(|p| self.contains_point(p))
            .filter(|p| predicate(&self[*p]))
            .collect::<Vec<Vec2>>()
    }

    pub fn contains_point(&self, point: &Vec2) -> bool {
        (0..self.rows as i32).contains(&point.y) && (0..self.columns as i32).contains(&point.x)
    }

    pub fn iter(&self) -> impl Iterator<Item = &Vec<T>> {
        self.grid.iter()
    }
}

impl<T: Display> Debug for Grid<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in &self.grid {
            for t in row {
                write!(f, "{}", t)?;
            }
            writeln!(f, "")?;
        }
        Ok(())
    }
}

impl<T> std::ops::Index<Vec2> for Grid<T> {
    type Output = T;
    fn index(&self, index: Vec2) -> &Self::Output {
        &self.grid[index.row()][index.column()]
    }
}

impl<T> std::ops::IndexMut<Vec2> for Grid<T> {
    fn index_mut(&mut self, index: Vec2) -> &mut Self::Output {
        &mut self.grid[index.row()][index.column()]
    }
}

impl<T> std::ops::Index<usize> for Grid<T> {
    type Output = Vec<T>;
    fn index(&self, index: usize) -> &Self::Output {
        &self.grid[index]
    }
}

impl<T> std::ops::IndexMut<usize> for Grid<T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.grid[index]
    }
}

impl<T: From<char>> FromIterator<String> for Grid<T> {
    fn from_iter<I: IntoIterator<Item = String>>(iter: I) -> Self {
        let mut rows = 0;
        let mut columns = 0;
        let grid = iter
            .into_iter()
            .map(|line| {
                if rows == 0 {
                    columns = line.len();
                }
                rows += 1;
                line.chars().map(|c| c.into()).collect::<Vec<T>>()
            })
            .collect::<Vec<Vec<T>>>();
        Grid::new(grid, rows, columns)
    }
}

impl FromIterator<Vec<char>> for Grid<char> {
    fn from_iter<I: IntoIterator<Item = Vec<char>>>(iter: I) -> Self {
        let mut rows = 0;
        let mut columns = 0;
        let grid = iter
            .into_iter()
            .map(|row| {
                if rows == 0 {
                    columns = row.len();
                }
                rows += 1;
                row
            })
            .collect::<Vec<Vec<char>>>();
        Grid::new(grid, rows, columns)
    }
}
