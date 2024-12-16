use std::collections::VecDeque;

use anyhow::{Context, Result};
use aoclib::grid::{
    is_move, Grid, Point, DOWN, LEFT, MOVE_DOWN, MOVE_LEFT, MOVE_RIGHT, MOVE_UP, RIGHT, UP,
};
use itertools::Itertools;

pub const WALL: u8 = b'#';
pub const BOX: u8 = b'O';
pub const ROBOT: u8 = b'@';
pub const FLOOR: u8 = b'.';

pub fn parse_inputs(input: &[u8]) -> Result<(Grid, &'_ [u8])> {
    let (first_move_pos, _) = input
        .iter()
        .find_position(|&&c| is_move(c))
        .with_context(|| "No move set found")?;
    Ok((
        Grid::new(&input[0..first_move_pos - 1])?,
        &input[first_move_pos..],
    ))
}

pub fn find_robot_start(grid: &Grid) -> Result<Point> {
    grid.points()
        .find(|p| grid.get_point(*p) == ROBOT)
        .with_context(|| "No robot found")
}

pub fn move_to_point(mv: u8) -> Option<Point> {
    match mv {
        MOVE_UP => Some(UP),
        MOVE_DOWN => Some(DOWN),
        MOVE_LEFT => Some(LEFT),
        MOVE_RIGHT => Some(RIGHT),
        _ => None,
    }
}

pub fn do_movements(grid: &mut Grid, moves: &[u8], mut robot_start: Point) -> Result<()> {
    // The strat is going to be to maintain a lookup of all the boxes.
    // When the robot moves, first check in the direction of the move,
    // putting boxes on the stack to move first if there are any.

    // println!("{}", std::str::from_utf8(grid.get_data())?);
    let mut boxes = VecDeque::new();
    for mv in moves {
        let Some(move_dir) = move_to_point(*mv) else {
            continue;
        };
        // println!("MOVE: {}", *mv as char);

        let mut next = robot_start + move_dir;
        // Peek ahead
        while grid.get_point(next) == BOX {
            boxes.push_front(next);
            next += move_dir;
        }

        // If next is clear space, start moving the boxes, then the robot
        // println!("{:?}", boxes);
        match grid.get_point(next) {
            FLOOR => {
                for b in boxes.drain(0..boxes.len()) {
                    *grid.get_point_mut(next) = BOX;
                    next = b;
                }
                *grid.get_point_mut(next) = ROBOT;
                *grid.get_point_mut(robot_start) = FLOOR;
                robot_start = next;
            }
            WALL => {
                // NoOp
                boxes.clear()
            }
            _ => unreachable!("No other next point possible"),
        }
        // println!("{}", std::str::from_utf8(grid.get_data())?);
    }
    Ok(())
}

pub fn score(grid: &Grid) -> usize {
    let mut total = 0;
    // GPS is 100 times the distance to the top edge + distance from left edge
    for point in grid.points() {
        if grid.get_point(point) == BOX {
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
    let (mut grid, moves) = parse_inputs(input)?;
    let start_point = find_robot_start(&grid)?;
    do_movements(&mut grid, moves, start_point)?;

    // println!("{}", std::str::from_utf8(grid.get_data())?);

    Ok(score(&grid).to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> anyhow::Result<()> {
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

        //         let input = b"########
        // #..O.O.#
        // ##@.O..#
        // #...O..#
        // #.#.O..#
        // #...O..#
        // #......#
        // ########

        // <^^>>>vv<v>>v<<";

        assert_eq!("10092", process(input)?);
        Ok(())
    }
}
