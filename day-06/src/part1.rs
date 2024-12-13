use std::collections::HashSet;

use anyhow::{anyhow, Result};
use aoclib::grid::{Grid, Point, DOWN, LEFT, RIGHT, UP};
use rayon::{iter::ParallelBridge, prelude::*};

pub const BLOCKER: u8 = b'#';
pub const UP_MOVE: u8 = b'^';
pub const RIGHT_MOVE: u8 = b'>';
pub const LEFT_MOVE: u8 = b'<';
pub const DOWN_MOVE: u8 = b'v';
pub const GUARD: &[u8] = &[UP_MOVE, RIGHT_MOVE, LEFT_MOVE, DOWN_MOVE];

pub fn turn_90(dir: u8) -> u8 {
    match dir {
        UP_MOVE => RIGHT_MOVE,
        RIGHT_MOVE => DOWN_MOVE,
        LEFT_MOVE => UP_MOVE,
        DOWN_MOVE => LEFT_MOVE,
        _ => unreachable!(),
    }
}

/// Count the number of unique points the guard goes thorugh.
///
/// When the guard hits an obstacle they turn right 90 and keep going.
/// Stop when the guard leaves the grid.
pub fn walk_the_guard(grid: Grid, start_pos: Point) -> usize {
    let mut positions = HashSet::new();
    let mut pos = start_pos;
    let mut dir = grid.get_point(pos);

    loop {
        positions.insert(pos);
        let next_pos = match dir {
            UP_MOVE => pos + UP,
            RIGHT_MOVE => pos + RIGHT,
            LEFT_MOVE => pos + LEFT,
            DOWN_MOVE => pos + DOWN,
            _ => unreachable!(),
        };

        if !grid.contains(next_pos) {
            break;
        }

        if grid.get_point(next_pos) == BLOCKER {
            dir = turn_90(dir)
        } else {
            pos = next_pos
        }
    }

    positions.len()
}

#[inline]
pub fn get_next_pos(pos: Point, dir: u8) -> Point {
    match dir {
        UP_MOVE => pos + UP,
        RIGHT_MOVE => pos + RIGHT,
        LEFT_MOVE => pos + LEFT,
        DOWN_MOVE => pos + DOWN,
        _ => unreachable!(),
    }
}

/// Advance one step, turning counts as a step.
///
/// If the step takes you out of the grid this returns None.
pub fn one_step(grid: &Grid, pos: Point, dir: u8) -> Option<(Point, u8)> {
    let mut pos = pos;
    let mut dir = dir;
    let next_pos = get_next_pos(pos, dir);

    if !grid.contains(next_pos) {
        return None;
    }

    if grid.get_point(next_pos) == BLOCKER {
        dir = turn_90(dir)
    } else {
        pos = next_pos
    }
    Some((pos, dir))
}

/// Floyd's tortoise and hare algorithm, with an adjustment for pathfinding.
pub fn turtle_guard_and_bunny_guard(grid: Grid, start_pos: Point) -> bool {
    let mut turtle_pos = start_pos;
    let mut bunny_pos = start_pos;
    let mut turtle_dir = grid.get_point(turtle_pos);
    let mut bunny_dir = turtle_dir;

    loop {
        // Advance turtle one step
        if let Some((turtle_next_pos, turtle_next_dir)) = one_step(&grid, turtle_pos, turtle_dir) {
            turtle_pos = turtle_next_pos;
            turtle_dir = turtle_next_dir;
        } else {
            return false;
        }

        // Advance bunny two steps
        if let Some((bunny_next_pos, bunny_next_dir)) = one_step(&grid, bunny_pos, bunny_dir) {
            bunny_pos = bunny_next_pos;
            bunny_dir = bunny_next_dir;
        } else {
            return false;
        }
        if let Some((bunny_next_pos, bunny_next_dir)) = one_step(&grid, bunny_pos, bunny_dir) {
            bunny_pos = bunny_next_pos;
            bunny_dir = bunny_next_dir;
        } else {
            return false;
        }

        // Check if they land on the same point
        if bunny_pos == turtle_pos && bunny_dir == turtle_dir {
            return true;
        }
    }
}

/// Place obstacles in the grid and see if we can create a loop.
///
/// Count the number of points where placing an obstacle will create a loop
pub fn obstruct_the_guard(input: &[u8]) -> Result<usize> {
    let grid = Grid::new(input)?;
    let guard_start = grid
        .rows()
        .flatten()
        .find(|p| GUARD.contains(&grid.get_point(*p)))
        .ok_or(anyhow!("No start pos found for guard"))?;
    let result = grid
        .points()
        .par_bridge()
        .map(|point| {
            if grid.get_point(point) != BLOCKER && point != guard_start {
                // Try and make it a blocker
                let mut permuted_grid = grid.clone();
                *permuted_grid.get_point_mut(point) = BLOCKER;
                // Test for loop
                if turtle_guard_and_bunny_guard(permuted_grid, guard_start) {
                    1
                } else {
                    0
                }
            } else {
                0
            }
        })
        .sum();
    Ok(result)
}

#[tracing::instrument]
pub fn process(input: &[u8]) -> Result<String> {
    // Determine the number of unique points the guard visits
    let grid = Grid::new(input)?;
    let start = grid
        .rows()
        .flatten()
        .find(|p| GUARD.contains(&grid.get_point(*p)))
        .ok_or(anyhow!("No start pos found for guard"))?;
    let answer = walk_the_guard(grid, start);

    Ok(answer.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> anyhow::Result<()> {
        let input = b"....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...";

        assert_eq!("41", process(input)?);
        Ok(())
    }
}
