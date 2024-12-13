use std::collections::VecDeque;

use aoclib::grid::{Grid, Point, ORTHOGONAL};
use rayon::iter::ParallelBridge;
use rayon::prelude::*;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct PointAndValue {
    point: Point,
    value: u8,
}

impl PointAndValue {
    pub fn new(point: Point, value: u8) -> Self {
        Self { point, value }
    }
}

pub fn find_trailheads(grid: &Grid) -> impl Iterator<Item = Point> + '_ {
    grid.points()
        .filter(|p| ascii_to_num(grid.get_point(*p)) == 0)
}

/// Assume safe since AOC input is good.
#[inline]
pub fn ascii_to_num(value: u8) -> u8 {
    value - 48
}

/// Count the number of 9's accessable by this trailhead
pub fn explore_trailhead(
    grid: &Grid,
    trailhead: Point,
    stack: &mut VecDeque<PointAndValue>,
    is_valid_next_step: fn(PointAndValue, PointAndValue, &VecDeque<PointAndValue>) -> bool,
) -> usize {
    // Passing a stack to reuse here in the hopes of wrapping in rayon later
    stack.clear();
    stack.push_back(PointAndValue::new(trailhead, 0));
    let mut count = 0;

    while !stack.is_empty() {
        let poi = stack.pop_front().unwrap();
        if poi.value == 9 {
            count += 1;
            continue;
        }
        for dir in ORTHOGONAL {
            let possible_step = poi.point + dir;
            if !grid.contains(possible_step) {
                continue;
            }
            let possible_value = ascii_to_num(grid.get_point(possible_step));
            let new = PointAndValue::new(possible_step, possible_value);
            if is_valid_next_step(poi, new, stack) {
                // if poi.value + 1 == new.value && !stack.contains(&new) {
                stack.push_back(new);
            }
        }
    }

    count
}

// Rayon was not faster :(
pub fn part1_sum_trailheads_rayon(grid: &Grid) -> usize {
    // Find all starting points
    find_trailheads(grid)
        .par_bridge()
        .map(|trailhead| {
            let mut stack = VecDeque::new();
            let score = explore_trailhead(
                grid,
                trailhead,
                &mut stack,
                |current_point: PointAndValue,
                 new_point: PointAndValue,
                 stack: &VecDeque<PointAndValue>| {
                    current_point.value + 1 == new_point.value && !stack.contains(&new_point)
                },
            );
            score
        })
        .sum()
}

pub fn part1_sum_trailheads(grid: &Grid) -> usize {
    // Find all starting points
    let mut stack = VecDeque::new();
    let mut count = 0;
    for trailhead in find_trailheads(grid) {
        let score = explore_trailhead(
            grid,
            trailhead,
            &mut stack,
            |current_point: PointAndValue,
             new_point: PointAndValue,
             stack: &VecDeque<PointAndValue>| {
                current_point.value + 1 == new_point.value && !stack.contains(&new_point)
            },
        );
        count += score
    }

    count
}

pub fn part2_sum_trailheads(grid: &Grid) -> usize {
    // Find all starting points
    let mut stack = VecDeque::new();
    let mut count = 0;
    for trailhead in find_trailheads(grid) {
        let score = explore_trailhead(
            grid,
            trailhead,
            &mut stack,
            |current_point: PointAndValue,
             new_point: PointAndValue,
             _stack: &VecDeque<PointAndValue>| {
                current_point.value + 1 == new_point.value
            },
        );
        count += score
    }

    count
}

#[tracing::instrument]
pub fn process(input: &[u8]) -> anyhow::Result<String> {
    let grid = Grid::new(input)?;
    Ok(part1_sum_trailheads(&grid).to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> anyhow::Result<()> {
        let input = b"89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732";

        assert_eq!("36", process(input)?);
        Ok(())
    }
}
