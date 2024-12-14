use anyhow::{bail, Result};
use aoclib::parsers::try_parse_num;

// TEST DATA
// pub const TALL: i16 = 7;
// pub const WIDE: i16 = 11;
pub const TALL: i16 = 103;
pub const WIDE: i16 = 101;

pub const QUAD_TOP_LEFT: Quadrant = Quadrant::new(0, WIDE / 2, 0, TALL / 2);
pub const QUAD_TOP_RIGHT: Quadrant = Quadrant::new((WIDE / 2) + 1, WIDE, 0, TALL / 2);
pub const QUAD_BOTTOM_LEFT: Quadrant = Quadrant::new(0, WIDE / 2, (TALL / 2) + 1, TALL);
pub const QUAD_BOTTOM_RIGHT: Quadrant = Quadrant::new((WIDE / 2) + 1, WIDE, (TALL / 2) + 1, TALL);

#[derive(Debug)]
pub struct Quadrant {
    pub x_lower: i16,
    pub x_upper: i16,
    pub y_lower: i16,
    pub y_upper: i16,
}

impl Quadrant {
    pub const fn new(x_lower: i16, x_upper: i16, y_lower: i16, y_upper: i16) -> Self {
        Self {
            x_lower,
            x_upper,
            y_lower,
            y_upper,
        }
    }

    pub fn in_x(&self, x: i16) -> bool {
        (self.x_lower..self.x_upper).contains(&x)
    }

    pub fn in_y(&self, y: i16) -> bool {
        (self.y_lower..self.y_upper).contains(&y)
    }
}

#[derive(Debug)]
pub struct Guard {
    pub x: i16,
    pub y: i16,
    pub x_velo: i16,
    pub y_velo: i16,
}

impl Guard {
    pub fn new(x: i16, y: i16, x_velo: i16, y_velo: i16) -> Self {
        Self {
            x,
            y,
            x_velo,
            y_velo,
        }
    }
}

pub fn parse_guard(input: &[u8]) -> Result<((i16, i16, i16, i16), usize)> {
    let mut offset = 2;
    let Some((initial_x, bytes_read)) = try_parse_num::<u32>(&input[offset..]) else {
        bail!("Failed to parse initial x pos.")
    };
    let initial_x = initial_x as i16;
    offset += bytes_read + 1;

    let Some((initial_y, bytes_read)) = try_parse_num::<u32>(&input[offset..]) else {
        bail!("Failed to parse initial yj pos.")
    };
    let initial_y = initial_y as i16;
    offset += bytes_read + 3;

    let x_velo_neg = input[offset] == b'-';
    if x_velo_neg {
        offset += 1;
    }
    let Some((x_velocity, bytes_read)) = try_parse_num::<u32>(&input[offset..]) else {
        bail!("Failed to parse x velocity.")
    };
    let x_velocity = x_velocity as i16 * if x_velo_neg { -1 } else { 1 };
    offset += bytes_read + 1;

    let y_velo_neg = input[offset] == b'-';
    if y_velo_neg {
        offset += 1;
    }
    let Some((y_velocity, bytes_read)) = try_parse_num::<u32>(&input[offset..]) else {
        bail!("Failed to parse y velocity.")
    };
    let y_velocity = y_velocity as i16 * if y_velo_neg { -1 } else { 1 };
    offset += bytes_read;

    Ok(((initial_x, initial_y, x_velocity, y_velocity), offset))
}

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

    // ticks
    for _ in 0..100 {
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
        }
    }

    // Count up guards in each quadrant
    let mut top_left = 0;
    let mut top_right = 0;
    let mut bottom_left = 0;
    let mut bottom_right = 0;

    for guard in &guards {
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

    Ok((top_left * top_right * bottom_left * bottom_right).to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> anyhow::Result<()> {
        let input = b"p=0,4 v=3,-3
p=6,3 v=-1,-3
p=10,3 v=-1,2
p=2,0 v=2,-1
p=0,0 v=1,3
p=3,0 v=-2,-2
p=7,6 v=-1,-3
p=3,0 v=-1,-2
p=9,3 v=2,3
p=7,3 v=-1,2
p=2,4 v=2,-3
p=9,5 v=-3,-3";

        assert_eq!("12", process(input)?);
        Ok(())
    }
}
