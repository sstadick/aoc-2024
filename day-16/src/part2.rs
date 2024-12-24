use std::collections::BinaryHeap;

use aoclib::grid::{Grid, Point, DOWN, LEFT, RIGHT, UP};
use rustc_hash::{FxHashMap, FxHashSet};

pub fn find_start(grid: &Grid) -> Point {
    grid.points()
        .filter(|p| grid.get_point(*p) == b'S')
        .take(1)
        .next()
        .unwrap()
}
pub fn find_end(grid: &Grid) -> Point {
    grid.points()
        .filter(|p| grid.get_point(*p) == b'E')
        .take(1)
        .next()
        .unwrap()
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct Node {
    pos: Point,
    cost: usize,
}

impl Node {
    pub fn new(pos: Point, cost: usize) -> Self {
        Self { pos, cost }
    }
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other
            .cost
            .cmp(&self.cost)
            .then_with(|| self.pos.cmp(&other.pos))
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

pub fn cost(came_from: Point, current: Point, next: Point) -> usize {
    let dir = current - came_from;
    let next_dir = next - current;
    let has_turned = matches!(
        (dir, next_dir),
        (UP, LEFT)
            | (UP, RIGHT)
            | (DOWN, LEFT)
            | (DOWN, RIGHT)
            | (LEFT, UP)
            | (RIGHT, UP)
            | (LEFT, DOWN)
            | (RIGHT, DOWN)
    );

    if has_turned {
        1001
    } else {
        1
    }
}

pub fn manhattan_dist(a: Point, b: Point) -> usize {
    (a.x - b.x).unsigned_abs() + (a.y - b.y).unsigned_abs()
}

pub fn heuristic(start: Point, goal: Point, query: Point) -> usize {
    manhattan_dist(start, query) + manhattan_dist(goal, query)
}

/// Ye Olde A*
///
/// # References
/// - https://www.redblobgames.com/pathfinding/a-star/introduction.html
pub fn find_lowest_cost_path(
    grid: &mut Grid,
) -> (
    FxHashSet<Point>,
    FxHashMap<Point, Point>,
    FxHashMap<Point, usize>,
) {
    let start = find_start(grid);
    let goal = find_end(grid);

    let mut frontier = BinaryHeap::new();
    frontier.push(Node::new(start, 0));

    let mut came_from = FxHashMap::default();
    let mut cost_so_far = FxHashMap::default();

    came_from.insert(start, start - LEFT);
    cost_so_far.insert(start, 0);

    while !frontier.is_empty() {
        let current = frontier.pop().unwrap();
        if current.pos == goal {
            break;
        }

        for next in grid
            .orthogonal_neighbors(current.pos)
            .filter(|p| grid.get_point(*p) != b'#')
        {
            let last_point = *came_from.get(&current.pos).unwrap_or(&current.pos);
            let new_cost =
                cost_so_far.get(&current.pos).unwrap() + cost(last_point, current.pos, next);
            if !cost_so_far.contains_key(&next) || new_cost < *cost_so_far.get(&next).unwrap() {
                cost_so_far.insert(next, new_cost);
                let priority = new_cost + heuristic(start, goal, next);
                frontier.push(Node::new(next, priority));
                came_from.insert(next, current.pos);
            }
        }
    }

    // Walk back the path for Debug
    let mut path = FxHashSet::default();
    let mut node = goal;
    while node != start {
        path.insert(node);
        let cost = cost_so_far.get(&node).unwrap();
        println!("{:?} -> {}", node, cost);
        node = *came_from.get(&node).unwrap();
        // *grid.get_point_mut(node) = b'*';
    }
    (path, came_from, cost_so_far)
}

/// Ye Olde A*
///
/// # References
/// - https://www.redblobgames.com/pathfinding/a-star/introduction.html
pub fn find_lowest_cost_paths(
    grid: &mut Grid,
    original_path: FxHashSet<Point>,
    original_came_from: FxHashMap<Point, Point>,
    original_cost_from: FxHashMap<Point, usize>,
) -> usize {
    let start = find_start(grid);
    let goal = find_end(grid);

    let mut frontier = BinaryHeap::new();
    frontier.push(Node::new(start, 0));

    let mut came_from = FxHashMap::default();
    let mut cost_so_far = FxHashMap::default();

    came_from.insert(start, start - LEFT);
    cost_so_far.insert(start, 0);

    while !frontier.is_empty() {
        let current = frontier.pop().unwrap();
        if current.pos == goal {
            break;
        }

        for next in grid
            .orthogonal_neighbors(current.pos)
            .filter(|p| grid.get_point(*p) != b'#')
        {
            let last_point = *came_from.get(&current.pos).unwrap_or(&current.pos);
            let new_cost =
                cost_so_far.get(&current.pos).unwrap() + cost(last_point, current.pos, next);
            if !cost_so_far.contains_key(&next) || new_cost < *cost_so_far.get(&next).unwrap() {
                cost_so_far.insert(next, new_cost);
                let priority = new_cost + heuristic(start, goal, next);
                frontier.push(Node::new(next, priority));
                came_from.insert(next, current.pos);

                if !original_path.contains(&current.pos)
                    && original_path.contains(&next)
                    && new_cost <= *original_cost_from.get(&next).unwrap_or(&usize::MAX)
                {
                    println!("Onto something: {:?}", current);
                }
            }
        }
    }

    // Walk back the path for Debug
    // let mut path = FxHashSet::default();
    // let mut node = goal;
    // while node != start {
    //     path.insert(node);
    //     let cost = cost_so_far.get(&node).unwrap();
    //     println!("{:?} -> {}", node, cost);
    //     node = *came_from.get(&node).unwrap();
    //     *grid.get_point_mut(node) = b'*';
    // }
    0
}

#[tracing::instrument]
pub fn process(input: &[u8]) -> anyhow::Result<String> {
    // Work through a maze from S to E, turns cost 1000, straight lines cost 1
    // Solve for lowest score
    let mut grid = Grid::new(input)?;
    let (x, y, z) = find_lowest_cost_path(&mut grid);
    let answer = find_lowest_cost_paths(&mut grid, x, y, z);
    Ok(answer.to_string())
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> anyhow::Result<()> {
        let input = b"###############
#.......#....E#
#.#.###.#.###.#
#.....#.#...#.#
#.###.#####.#.#
#.#.#.......#.#
#.#.#####.###.#
#...........#.#
###.#.#####.#.#
#...#.....#.#.#
#.#.#.###.#.#.#
#.....#...#.#.#
#.###.#.#.#.#.#
#S..#.....#...#
###############";

        assert_eq!("45", process(input)?);
        Ok(())
    }
}
