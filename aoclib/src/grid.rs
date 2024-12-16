use std::ops::{Add, AddAssign, Mul, Sub, SubAssign};

use anyhow::{Context, Result};

use itertools::Itertools;

// Box the inner iterator to create a uniform type
type PointIter = Box<dyn Iterator<Item = Point>>;
// A view is just a collection of points in row-wise, column-wise, left-diag, right-diag ordering
type ViewIter<'a> = Box<dyn Iterator<Item = PointIter> + 'a>;

/// A point representation
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub struct Point {
    pub x: isize,
    pub y: isize,
}

impl Point {
    pub const fn new(x: isize, y: isize) -> Self {
        Self { x, y }
    }

    pub fn get_dx_dy(&self, other: &Self) -> Point {
        Point {
            x: other.x - self.x,
            y: other.y - self.y,
        }
    }

    pub fn dist(&self, other: &Self) -> f64 {
        ((other.x - self.x).pow(2) as f64 + (other.y - self.y).pow(2) as f64).sqrt()
    }

    pub fn slope(&self, other: &Self) -> f64 {
        (other.y - self.y) as f64 / (other.x - self.x) as f64
    }
}

impl Add for Point {
    type Output = Self;

    #[inline]
    #[must_use]
    fn add(self, rhs: Self) -> Self {
        Point::new(self.x + rhs.x, self.y + rhs.y)
    }
}

impl AddAssign for Point {
    #[inline]
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl Mul<isize> for Point {
    type Output = Self;

    #[inline]
    #[must_use]
    fn mul(self, rhs: isize) -> Self {
        Point::new(self.x * rhs, self.y * rhs)
    }
}

impl Sub for Point {
    type Output = Self;

    #[inline]
    #[must_use]
    fn sub(self, rhs: Self) -> Self {
        Point::new(self.x - rhs.x, self.y - rhs.y)
    }
}

impl SubAssign for Point {
    #[inline]
    fn sub_assign(&mut self, rhs: Self) {
        self.x -= rhs.x;
        self.y -= rhs.y;
    }
}

pub const MOVE_UP: u8 = b'^';
pub const MOVE_LEFT: u8 = b'<';
pub const MOVE_RIGHT: u8 = b'>';
pub const MOVE_DOWN: u8 = b'v';

pub const fn is_move(c: u8) -> bool {
    matches!(c, MOVE_UP | MOVE_DOWN | MOVE_LEFT | MOVE_RIGHT)
}

pub const ORIGIN: Point = Point::new(0, 0);
pub const DOWN: Point = Point::new(0, -1);
pub const UP: Point = Point::new(0, 1);
pub const LEFT: Point = Point::new(-1, 0);
pub const RIGHT: Point = Point::new(1, 0);
pub const UP_LEFT: Point = Point::new(-1, 1);
pub const UP_RIGHT: Point = Point::new(1, 1);
pub const DOWN_LEFT: Point = Point::new(-1, -1);
pub const DOWN_RIGHT: Point = Point::new(1, -1);
/// Left to right and top to bottom.
pub const ORTHOGONAL: [Point; 4] = [UP, DOWN, LEFT, RIGHT];
/// N, NE, E, SE, S, SW, W, NW
pub const ALL_DIRECTIONS: [Point; 8] = [
    DOWN_LEFT, UP, DOWN_RIGHT, LEFT, RIGHT, UP_LEFT, DOWN, UP_RIGHT,
];

/// Grid represented as a slice of bytes with rows delimited by newlines.
///
/// The coordinate system for the grid treats the bottom left as (0, 0).
#[derive(Debug, Clone)]
pub struct Grid {
    data: Vec<u8>,
    // Number of rows
    height: usize,
    // Number of columns
    width: usize,
}

impl Grid {
    /// Create a grid from a string repr of a grid.
    pub fn new(data: &[u8]) -> Result<Self> {
        // width, not including newline
        let width = data
            .iter()
            .find_position(|c| **c == b'\n')
            .context("Can't find a newline")?
            .0;
        // handle no trailing newline in input
        let input_len = if data[data.len() - 1] != b'\n' {
            data.len() + 1
        } else {
            data.len()
        };
        let height = input_len / (width + 1);
        Ok(Self {
            width,
            height,
            data: data.to_vec(),
        })
    }

    pub fn num_rows(&self) -> usize {
        self.height
    }

    pub fn num_cols(&self) -> usize {
        self.width
    }

    pub fn get_data(&self) -> &[u8] {
        &self.data
    }

    // "01234\n56789\nabcde\n"
    // [0, (0, 2), index  0], [1, (1, 2),  1], [2, (2, 2),  2], [3, (3, 2),  3], [4, (4, 2),  4]
    // [5, (0, 1), index  6], [6, (1, 1),  7], [7, (2, 1),  8], [8, (3, 1),  9], [9, (4, 1), 10]
    // [a, (0, 0), index 12], [b, (1, 0), 13], [c, (2, 0), 14], [d, (3, 0), 15], [e, (4, 0), 16]
    pub fn get_point(&self, point: Point) -> u8 {
        // Width + 1 accounts for newline character
        let row_length = self.width + 1;

        // Convert y from bottom-up to top-down counting
        let adjusted_y = self.height as isize - 1 - point.y;

        // Calculate index: (row * row_length) + x position
        let index = (adjusted_y * row_length as isize) + point.x;

        assert!(index >= 0);
        self.data[index as usize]
    }

    pub fn get_point_mut(&mut self, point: Point) -> &mut u8 {
        // Width + 1 accounts for newline character
        let row_length = self.width + 1;

        // Convert y from bottom-up to top-down counting
        let adjusted_y = self.height as isize - 1 - point.y;

        // Calculate index: (row * row_length) + x position
        let index = (adjusted_y * row_length as isize) + point.x;

        if index < 0 {
            println!("Invalid point: {:?}", point);
        }
        assert!(index >= 0);
        &mut self.data[index as usize]
    }

    pub fn rows(&self) -> ViewIter {
        Box::new((0..self.height as isize).map(move |y| {
            Box::new((0..self.width as isize).map(move |x| Point { x, y })) as PointIter
        }))
    }

    pub fn columns(&self) -> ViewIter {
        Box::new((0..self.width as isize).map(move |x| {
            Box::new((0..self.height as isize).map(move |y| Point { x, y })) as PointIter
        }))
    }

    pub fn points(&self) -> impl Iterator<Item = Point> + '_ {
        (0..self.height as isize)
            .flat_map(move |y| (0..self.width as isize).map(move |x| Point { x, y }))
    }

    pub fn contains(&self, point: Point) -> bool {
        (0..self.width as isize).contains(&point.x) && (0..self.height as isize).contains(&point.y)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use anyhow::Result;

    #[test]
    fn test_grid() -> Result<()> {
        let input = b"01234
56789
abcde";
        let grid = Grid::new(input)?;
        // Test corners
        assert_eq!(grid.get_point(Point::new(0, 0)), b'a', "get a");
        assert_eq!(grid.get_point(Point::new(4, 0)), b'e', "get e");
        assert_eq!(grid.get_point(Point::new(0, 2)), b'0', "get 0");
        assert_eq!(grid.get_point(Point::new(4, 2)), b'4', "get 4");
        // Test inner
        assert_eq!(grid.get_point(Point::new(2, 1)), b'7', "get 7");

        println!("Rows:");
        for row in grid.rows() {
            let points: Vec<_> = row.collect();
            println!("{:?}", points);
        }

        println!("\nColumns:");
        for col in grid.columns() {
            let points: Vec<_> = col.collect();
            println!("{:?}", points);
        }

        Ok(())
    }
}
