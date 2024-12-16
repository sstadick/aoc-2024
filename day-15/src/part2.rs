use std::collections::VecDeque;

use anyhow::Result;
use aoclib::grid::{Grid, Point, DOWN, LEFT, RIGHT, UP};

use crate::part1::{find_robot_start, move_to_point, parse_inputs, BOX, FLOOR, ROBOT, WALL};

pub const L_BOX: u8 = b'[';
pub const R_BOX: u8 = b']';

#[derive(Debug, Clone, Copy)]
pub struct WBox {
    left: Point,
    right: Point,
    /// This is in the context of when it is created and the direction that we are checking in.
    moveable: bool,
}

impl WBox {
    pub fn new(point: Point, grid: &Grid, move_dir: Point) -> Self {
        let (left, right) = if grid.get_point(point) == L_BOX {
            (point, point + RIGHT)
        } else {
            (point + LEFT, point)
        };

        let moveable = match move_dir {
            UP => grid.get_point(left + UP) != WALL && grid.get_point(right + UP) != WALL,
            DOWN => grid.get_point(left + DOWN) != WALL && grid.get_point(right + DOWN) != WALL,
            LEFT => grid.get_point(left + LEFT) != WALL,
            RIGHT => grid.get_point(right + RIGHT) != WALL,
            _ => unreachable!("Illegal move"),
        };
        Self {
            left,
            right,
            moveable,
        }
    }

    pub fn new_with_moveable(point: Point, grid: &Grid, moveable: bool) -> Self {
        let (left, right) = if grid.get_point(point) == L_BOX {
            (point, point + RIGHT)
        } else {
            (point + LEFT, point)
        };

        Self {
            left,
            right,
            moveable,
        }
    }
}

pub fn expand_map(input: &[u8]) -> Vec<u8> {
    let mut new = Vec::with_capacity(input.len() * 2);
    for c in input {
        match *c {
            WALL => new.extend(&[WALL, WALL]),
            BOX => new.extend(&[b'[', b']']),
            ROBOT => new.extend(&[ROBOT, FLOOR]),
            FLOOR => new.extend(&[FLOOR, FLOOR]),
            _ => new.push(*c),
        }
    }
    new
}

pub fn do_movements_wide(grid: &mut Grid, moves: &[u8], mut robot_start: Point) -> Result<()> {
    // The strat is going to be to maintain a lookup of all the boxes.
    // When the robot moves, first check in the direction of the move,
    // putting boxes on the stack to move first if there are any.

    // println!("{}", std::str::from_utf8(grid.get_data())?);
    let mut boxes = VecDeque::new();
    let mut nexts = vec![];
    let mut next_nexts = vec![];
    for mv in moves {
        let Some(move_dir) = move_to_point(*mv) else {
            continue;
        };
        // println!("MOVE: {}", *mv as char);

        if grid.get_point(robot_start + move_dir) == WALL {
            continue;
        }
        nexts.push(robot_start + move_dir);

        // Peek ahead
        while !nexts.is_empty() {
            for next in &nexts {
                let next_point = grid.get_point(*next);
                if next_point == L_BOX || next_point == R_BOX {
                    let b = WBox::new(*next, &grid, move_dir);
                    boxes.push_front(b);
                    if b.moveable {
                        match move_dir {
                            UP | DOWN => {
                                next_nexts.push(b.left + move_dir);
                                next_nexts.push(b.right + move_dir);
                            }
                            LEFT => next_nexts.push(b.left + move_dir),
                            RIGHT => next_nexts.push(b.right + move_dir),
                            _ => unreachable!("Invalid dir"),
                        }
                    } else {
                        // So the last box on our vecdeque is not moveable
                        next_nexts.clear();
                        break;
                    }
                } else if next_point == FLOOR {
                    continue;
                } else if next_point == WALL {
                    // This should be caught by the box moeable check
                    next_nexts.clear();
                    break;
                }
            }
            std::mem::swap(&mut nexts, &mut next_nexts);
            next_nexts.clear();
        }

        // println!("{:?}", boxes);
        if !boxes.front().map(|b| b.moveable).unwrap_or(true) {
            // If the last box isn't moveable, get out
            boxes.clear();
            continue;
        }

        while let Some(b) = boxes.pop_front() {
            *grid.get_point_mut(b.left) = FLOOR;
            *grid.get_point_mut(b.right) = FLOOR;
            *grid.get_point_mut(b.left + move_dir) = L_BOX;
            *grid.get_point_mut(b.right + move_dir) = R_BOX;
        }
        *grid.get_point_mut(robot_start + move_dir) = ROBOT;
        *grid.get_point_mut(robot_start) = FLOOR;
        robot_start = robot_start + move_dir;

        // println!("{}", std::str::from_utf8(grid.get_data())?);
    }
    Ok(())
}

pub fn score_wide(grid: &Grid) -> usize {
    let mut total = 0;
    // GPS is 100 times the distance to the top edge + distance from left edge
    for point in grid.points() {
        if grid.get_point(point) == L_BOX {
            // let b = WBox::new_with_moveable(point, grid, true);

            let dist_from_left = point.x as usize;
            let dist_from_top = (grid.num_rows() - 1 - point.y as usize) * 100;
            // println!("{:?}: {}, {}", point, dist_from_left, dist_from_top);
            total += dist_from_left + dist_from_top;
        }
    }
    total
}

#[tracing::instrument]
pub fn process(input: &[u8]) -> anyhow::Result<String> {
    let (grid, moves) = parse_inputs(input)?;
    let expanded = expand_map(grid.get_data());
    let mut grid = Grid::new(&expanded)?;
    let start_point = find_robot_start(&grid)?;
    do_movements_wide(&mut grid, moves, start_point)?;
    Ok(score_wide(&grid).to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> anyhow::Result<()> {
        //         let input = b"#######
        // #...#.#
        // #.....#
        // #..OO@#
        // #..O..#
        // #.....#
        // #######

        // <vv<<^^<<^^";

        let input = b"##########
#..O..O.O#
#......O.#
#.OO..O.O#
#..O@..O.#
#O#..O...#
#O..O..O.#
#.OO.O.OO#
#....O...#
##########

<vv>^<v^>v>^vv^v>v<>v^v<v<^vv<<<^><<><>>v<vvv<>^v^>^<<<><<v<<<v^vv^v>^
vvv<<^>^v^^><<>>><>^<<><^vv^^<>vvv<>><^^v>^>vv<>v<<<<v<^v>^<^^>>>^<v<v
><>vv>v^v^<>><>>>><^^>vv>v<^^^>>v^v^<^^>v^^>v^<^v>v<>>v^v^<v>v^^<^^vv<
<<v<^>>^^^^>>>v^<>vvv^><v<<<>^^^vv^<vvv>^>v<^^^^v<>^>vvvv><>>v^<<^^^^^
^><^><>>><>^^<<^^v>>><^<v>^<vv>>v>>>^v><>^v><<<<v>>v<v<v>vvv>^<><<>^><
^>><>^v<><^vvv<^^<><v<<<<<><^v<<<><<<^^<v<^^^><^>>^<v^><<<^>>^v<v^v<v^
>^>>^v>vv>^<<^v<>><<><<v<<v><>v<^vv<<<>^^v^>^^>>><<^v>>v^v><^^>>^<>vv^
<><^^>^^^<><vvvvv^v<v<<>^v<v>v<<^><<><<><<<^^<<<^<<>><<><^^^>^^<>^>v<>
^^>vv<^v^v<vv>^<><v<^v>^^^>>>^^vvv^>vvv<>>>^<^>>>>>^<<^v>^vvv<>^<><<v>
v^^>>><<^^<>>^v^<v^vv<>v^<<>^<^v^v><^<<<><<^<v><v<>vv>>v><v^<vv<>v^<<^";

        assert_eq!("9021", process(input)?);
        Ok(())
    }
}
