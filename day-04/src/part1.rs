use anyhow::{Context, Result};

use itertools::Itertools;

// Box the inner iterator to create a uniform type
type PointIter = Box<dyn Iterator<Item = Point>>;
// A view is just a collection of points in row-wise, column-wise, left-diag, right-diag ordering
type ViewIter<'a> = Box<dyn Iterator<Item = PointIter> + 'a>;

#[derive(Debug, Clone, Copy)]
pub struct Point {
    pub x: usize,
    pub y: usize,
}

// Grid represented as a slice of bytes with rows delmited by newlines.
//
// The coordinate system for the grid treates the bottom left as (0, 0).
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
    pub fn get_point(&self, x: usize, y: usize) -> u8 {
        assert!(x <= self.width, "x value outside of grid width");
        assert!(y <= self.height, "y value outside of grid height");
        // Width + 1 accounts for newline character
        let row_length = self.width + 1;

        // Convert y from bottom-up to top-down counting
        let adjusted_y = self.height - 1 - y;

        // Calculate index: (row * row_length) + x position
        let index = (adjusted_y * row_length) + x;

        self.data[index]
    }

    pub fn rows(&self) -> ViewIter {
        Box::new(
            (0..self.height)
                .map(move |y| Box::new((0..self.width).map(move |x| Point { x, y })) as PointIter),
        )
    }

    pub fn columns(&self) -> ViewIter {
        Box::new(
            (0..self.width)
                .map(move |x| Box::new((0..self.height).map(move |y| Point { x, y })) as PointIter),
        )
    }

    pub fn left_diagonals(&self) -> ViewIter {
        let width = self.width;
        let height = self.height;

        // Start from rows
        let from_rows = (0..height).map(move |start_y| {
            Box::new(
                (0..width)
                    .map(move |dx| {
                        let x = dx;
                        let y = match start_y.checked_add(dx) {
                            Some(y) => y,
                            None => height, // This will be filtered out by take_while
                        };
                        (x, y)
                    })
                    .take_while(move |(_, y)| *y < height)
                    .map(|(x, y)| Point { x, y }),
            ) as PointIter
        });

        // Start from columns (except first column)
        let from_cols = (1..width).map(move |start_x| {
            Box::new(
                (0..height)
                    .map(move |dy| {
                        let x = match start_x.checked_add(dy) {
                            Some(x) => x,
                            None => width, // This will be filtered out by take_while
                        };
                        let y = dy;
                        (x, y)
                    })
                    .take_while(move |(x, _)| *x < width)
                    .map(|(x, y)| Point { x, y }),
            ) as PointIter
        });

        Box::new(from_rows.chain(from_cols))
    }

    pub fn right_diagonals(&self) -> ViewIter {
        let width = self.width;
        let height = self.height;

        // Start from rows
        let from_rows = (0..height).map(move |start_y| {
            Box::new(
                (0..width)
                    .map(move |dx| {
                        let x = width.saturating_sub(1 + dx);
                        let y = start_y + dx;
                        (x, y)
                    })
                    .take_while(move |(_, y)| *y < height)
                    .map(|(x, y)| Point { x, y }),
            ) as PointIter
        });

        // Start from columns (except last column)
        let from_cols = (0..width - 1).rev().map(move |start_x| {
            Box::new(
                (0..height)
                    .map(move |dy| {
                        let x = start_x.saturating_sub(dy);
                        let y = dy;
                        (x, y)
                    })
                    .take_while(move |&(x, y)| x < width && y < height)
                    .map(|(x, y)| Point { x, y }),
            ) as PointIter
        });

        Box::new(from_rows.chain(from_cols))
    }

    pub fn all_views(&self) -> ViewIter {
        Box::new(
            self.rows()
                .chain(self.columns())
                .chain(self.left_diagonals())
                .chain(self.right_diagonals()),
        )
    }

    /// Search the grid for anny occurances of the needle
    pub fn count_occurances(&self, needle: &[u8]) -> usize {
        let mut buffer = Vec::with_capacity(self.height * self.width);
        let rev_needle = needle.iter().copied().rev().collect::<Vec<_>>();
        let mut count = 0;
        for view in self.all_views() {
            buffer.clear();

            // copy the searchable rows into the buffer
            for point in view {
                buffer.push(self.get_point(point.x, point.y));
            }
            count += find_all(needle, &rev_needle, &buffer);
            // count += find_all(&rev_needle, &buffer);
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
    let count = grid.count_occurances(b"XMAS");
    Ok(count.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    use anyhow::Result;
    use std::io::Write;
    use std::path::Path;

    fn write_to_file<P: AsRef<Path>>(data: &str, file: P) {
        let mut file = std::fs::File::create(file).unwrap();
        file.write_all(data.as_bytes()).unwrap();
    }

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
        // let grid = Grid::new(input)?;
        // let count = grid.count_occurances(b"XMAS");
        // assert_eq!(18, count);
        assert_eq!("18", process(input)?);
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
        assert_eq!(grid.get_point(0, 0), b'a', "get a");
        assert_eq!(grid.get_point(4, 0), b'e', "get e");
        assert_eq!(grid.get_point(0, 2), b'0', "get 0");
        assert_eq!(grid.get_point(4, 2), b'4', "get 4");
        // Test inner
        assert_eq!(grid.get_point(2, 1), b'7', "get 7");

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

        println!("\nLeft Diagonals:");
        for diag in grid.left_diagonals() {
            let points: Vec<_> = diag.collect();
            println!("{:?}", points);
        }

        println!("\nRight Diagonals:");
        for diag in grid.right_diagonals() {
            let points: Vec<_> = diag.collect();
            println!("{:?}", points);
        }
        Ok(())
    }
}
