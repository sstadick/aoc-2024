use std::{collections::HashSet, i32};

use aoclib::grid::{self, Grid, Point};

use crate::part1::{
    parse_guard, Guard, QUAD_BOTTOM_LEFT, QUAD_BOTTOM_RIGHT, QUAD_TOP_LEFT, QUAD_TOP_RIGHT, TALL,
    WIDE,
};

#[tracing::instrument]
pub fn process(input: &[u8]) -> anyhow::Result<String> {
    let mut offset = 0;
    let mut guards = vec![];
    while offset < input.len() {
        let (pos, bytes_read) = parse_guard(&input[offset..])?;
        let guard = Guard::new(pos.0, pos.1, pos.2, pos.3);
        guards.push(guard);
        offset += bytes_read + 1;
    }
    find_lowest_danger(guards);

    // let mut grid_data = vec![];
    // for row in 0..TALL {
    //     for column in 0..WIDE {
    //         grid_data.push(b' ');
    //     }
    //     grid_data.push(b'\n');
    // }
    // let mut grid = Grid::new(&grid_data)?;
    // println!("After {} Seconds", 0);
    // println!("{}", std::str::from_utf8(grid.get_data())?);
    // println!("------------------------------");

    // // ticks
    // for i in 0..10_000 {
    //     let mut guard_set = HashSet::new();
    //     for guard in guards.iter_mut() {
    //         // Reset the point
    //         *grid.get_point_mut(Point::new(guard.x as isize, (TALL - 1 - guard.y) as isize)) = b' ';
    //         let mut new_x = (guard.x + guard.x_velo) % WIDE;
    //         let mut new_y = (guard.y + guard.y_velo) % TALL;
    //         if new_x < 0 {
    //             new_x += WIDE;
    //         }
    //         if new_y < 0 {
    //             new_y += TALL;
    //         }
    //         guard.x = new_x;
    //         guard.y = new_y;
    //         // Draw the new point
    //         *grid.get_point_mut(Point::new(guard.x as isize, (TALL - 1 - guard.y) as isize)) = b'*';
    //         guard_set.insert((guard.x, guard.y));
    //     }

    //     let image = std::str::from_utf8(grid.get_data())?;
    //     // let distinct = guard_set.len();
    //     // if distinct == guards.len() || distinct == guards.len() - 1 || distinct == guards.len() - 2
    //     if image.contains("********") {
    //         println!("After {} Seconds", i);
    //         println!("{}", image);
    //         println!("------------------------------")
    //     }
    // }

    Ok("OK".to_string())
}

pub fn find_lowest_danger(mut guards: Vec<Guard>) {
    // ticks
    let mut min_score = i32::MAX;
    let mut second = 0;
    for i in 0..11_000 {
        // Count up guards in each quadrant
        let mut top_left = 0;
        let mut top_right = 0;
        let mut bottom_left = 0;
        let mut bottom_right = 0;
        let mut guard_set = HashSet::new();
        for guard in guards.iter_mut() {
            let mut new_x = (guard.x + guard.x_velo) % WIDE;
            let mut new_y = (guard.y + guard.y_velo) % TALL;
            if new_x < 0 {
                new_x += WIDE;
            }
            if new_y < 0 {
                new_y += TALL;
            }
            guard.x = new_x;
            guard.y = new_y;

            guard_set.insert((guard.x, guard.y));

            // update score
            if QUAD_TOP_LEFT.in_x(guard.x) && QUAD_TOP_LEFT.in_y(guard.y) {
                top_left += 1;
            } else if QUAD_TOP_RIGHT.in_x(guard.x) && QUAD_TOP_RIGHT.in_y(guard.y) {
                top_right += 1;
            } else if QUAD_BOTTOM_LEFT.in_x(guard.x) && QUAD_BOTTOM_LEFT.in_y(guard.y) {
                bottom_left += 1;
            } else if QUAD_BOTTOM_RIGHT.in_x(guard.x) && QUAD_BOTTOM_RIGHT.in_y(guard.y) {
                bottom_right += 1;
            }
        }
        let safety_score = top_left * top_right * bottom_left * bottom_right;
        if safety_score < min_score {
            min_score = safety_score;
            second = i;
        }
        if guard_set.len() == guards.len() {
            println!("Time with no overlaps: {}", second + 1);
        }
    }
    println!("Score: {}", min_score);
    println!("Time: {}", second + 1);
}

#[cfg(test)]
mod tests {
    use super::*;

    // #[test]
    // fn test_process() -> anyhow::Result<()> {
    //     todo!("haven't built test yet");
    //     let input = b"";

    //     assert_eq!("", process(input)?);
    //     Ok(())
    // }
}
