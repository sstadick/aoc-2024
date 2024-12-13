use std::ops::{Add, AddAssign, Mul, Sub, SubAssign};

use anyhow::{Context, Result};

use itertools::Itertools;

// Box the inner iterator to create a uniform type
type PointIter = Box<dyn Iterator<Item = Point>>;
// A view is just a collection of points in row-wise, column-wise, left-diag, right-diag ordering
type ViewIter<'a> = Box<dyn Iterator<Item = PointIter> + 'a>;

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub struct Point {
    pub x: isize,
    pub y: isize,
}

impl Point {
    pub const fn new(x: isize, y: isize) -> Self {
        Self { x, y }
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

// Grid represented as a slice of bytes with rows delimited by newlines.
//
// The coordinate system for the grid treats the bottom left as (0, 0).
pub struct Grid {
    data: &'static [u8],
    // Number of rows
    height: usize,
    // Number of columns
    width: usize,
}

impl Grid {
    pub fn new(data: &'static [u8]) -> Result<Self> {
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
            data,
        })
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

    pub fn contains(&self, point: Point) -> bool {
        (0..self.width as isize).contains(&point.x) && (0..self.height as isize).contains(&point.y)
    }

    /// Search the grid for any occurrences of xmas (part1)
    pub fn count_occurrences_part1(&self) -> usize {
        let mut count = 0;
        for point in self.rows().flatten() {
            // If the point is an X, look in all directions of it for XMAS
            let value = self.get_point(point);
            if value == b'X' {
                for dir in ALL_DIRECTIONS {
                    count += (self.contains(point + (dir * 3))
                        && self.get_point(point + (dir * 1)) == b'M'
                        && self.get_point(point + (dir * 2)) == b'A'
                        && self.get_point(point + (dir * 3)) == b'S')
                        as usize;
                }
            }
        }

        count
    }

    /// Search the grid for any occurrences of x-mas (part 2)
    pub fn count_occurrences_part2(&self) -> usize {
        let mut count = 0;

        for x in 1..self.width as isize - 1 {
            for y in 1..self.height as isize - 1 {
                // minus to because the grid is special
                let point = Point::new(x, y);

                if self.get_point(point) == b'A' {
                    let upper_left = self.get_point(point + UP_LEFT);
                    let upper_right = self.get_point(point + UP_RIGHT);
                    let lower_right = self.get_point(point + DOWN_RIGHT);
                    let lower_left = self.get_point(point + DOWN_LEFT);

                    // Vertical
                    // M M
                    //  A
                    // S S
                    // Horizontal
                    // M  S
                    //  A
                    // M  S

                    let horizontal = upper_left == lower_left
                        && upper_right == lower_right
                        && upper_left.abs_diff(upper_right) == 6; // abuse fact that grid is only made of XMAS characters
                    let vertical = upper_left == upper_right
                        && lower_left == lower_right
                        && upper_left.abs_diff(lower_left) == 6;
                    count += (horizontal || vertical) as usize
                }
            }
        }

        count
    }
}

pub fn find_all(needle: &[u8], rev_needle: &[u8], haystack: &[u8]) -> usize {
    let mut count = 0;
    for window in haystack.windows(needle.len()) {
        if window == needle || window == rev_needle {
            count += 1;
        }
    }
    count
}

#[tracing::instrument]
pub fn process(input: &'static [u8]) -> anyhow::Result<String> {
    // Find the number of occurances of XMAS in the grid.
    let grid = Grid::new(input)?;
    let count = grid.count_occurrences_part1();
    Ok(count.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    use anyhow::Result;

    #[test]
    fn test_process() -> Result<()> {
        let input = b"MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX";
        assert_eq!("18", process(input)?);
        Ok(())
    }

    #[test]
    fn test_row_search() -> Result<()> {
        let input = b"..XMAS..
........
XMASSAMX
........";
        assert_eq!("3", process(input)?);
        Ok(())
    }

    #[test]
    fn test_col_search() -> Result<()> {
        let input = b".X...S..
.M...A..
.A...M..
.S...X..";
        assert_eq!("2", process(input)?);
        Ok(())
    }

    #[test]
    fn test_left_diag() -> Result<()> {
        let input = b"...S.X
..A.M.
.M.A..
X.S...";
        assert_eq!("2", process(input)?);
        Ok(())
    }

    #[test]
    fn test_right_diag() -> Result<()> {
        let input = b"X.S...
.M.A..
..A.M.
...S.X";
        assert_eq!("2", process(input)?);
        Ok(())
    }

    #[test]
    fn test_smaller() -> Result<()> {
        let input = b"..X...
.SAMX.
.A..A.
XMAS.S
.X....";
        assert_eq!("4", process(input)?);
        Ok(())
    }

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
