use anyhow::{anyhow, bail, Result};
use aoclib::{grid::Point, parsers::try_parse_num};

use ndarray::{Array1, Array2};
use ndarray_linalg::Solve;

pub const A_COST: isize = 3;
pub const B_COST: isize = 1;
const EPSILON: f64 = 1e-15;

#[derive(Debug)]
pub struct Machine {
    pub button_a: Point,
    pub button_b: Point,
    pub prize: Point,
}

impl Machine {
    /// Solve the
    /// # References
    /// - https://en.wikipedia.org/wiki/Diophantine_equation
    pub fn find_solution(&self) -> Result<(isize, isize)> {
        // solve_movement(self.button_a, self.button_b, self.prize)
        // Create the coefficient matrix A
        let a = Array2::from_shape_vec(
            (2, 2),
            vec![
                self.button_a.x as f64,
                self.button_b.x as f64,
                self.button_a.y as f64,
                self.button_b.y as f64,
            ],
        )?;

        // Create the target vector b
        let b = Array1::from_vec(vec![self.prize.x as f64, self.prize.y as f64]);

        let s = a.solve(&b)?;

        let x = s[0].round();
        let y = s[1].round();
        if (x - s[0]).abs() > EPSILON || (y - s[1]).abs() > EPSILON {
            bail!("Non integer solution");
        }
        Ok((x as isize, y as isize))
    }

    pub fn parse_button(input: &[u8]) -> Result<(Point, usize)> {
        let mut offset = 12;
        let Some((x, bytes_read)) = try_parse_num::<u32>(&input[offset..]) else {
            bail!("Failed to parse x");
        };
        offset += bytes_read + 4;
        let Some((y, bytes_read)) = try_parse_num::<u32>(&input[offset..]) else {
            bail!("Failed to parse x");
        };
        offset += bytes_read;
        Ok((Point::new(x as isize, y as isize), offset))
    }

    pub fn parse_prize(input: &[u8]) -> Result<(Point, usize)> {
        let mut offset = 9;
        let Some((x, bytes_read)) = try_parse_num::<u32>(&input[offset..]) else {
            bail!("Failed to parse x");
        };
        offset += bytes_read + 4;
        let Some((y, bytes_read)) = try_parse_num::<u32>(&input[offset..]) else {
            bail!("Failed to parse x");
        };
        offset += bytes_read;
        Ok((Point::new(x as isize, y as isize), offset))
    }

    // Button A: X+94, Y+34
    // Button B: X+22, Y+67
    // Prize: X=8400, Y=5400
    pub fn parse_machines(input: &[u8]) -> impl Iterator<Item = Result<Self>> + '_ {
        let mut offset = 0;

        std::iter::from_fn(move || {
            if offset >= input.len() {
                return None;
            }
            let Ok((button_a, bytes_read)) = Self::parse_button(&input[offset..]) else {
                return Some(Err(anyhow!("Failed to parse Button A.")));
            };
            offset += bytes_read + 1;
            let Ok((button_b, bytes_read)) = Self::parse_button(&input[offset..]) else {
                return Some(Err(anyhow!("Failed to parse Button B.")));
            };
            offset += bytes_read + 1;
            let Ok((prize, bytes_read)) = Self::parse_prize(&input[offset..]) else {
                return Some(Err(anyhow!("Failed to parse Prize.")));
            };
            offset += bytes_read + 2;
            Some(Ok(Self {
                button_a,
                button_b,
                prize,
            }))
        })
    }
}

#[tracing::instrument]
pub fn process(input: &[u8]) -> anyhow::Result<String> {
    // Find the sum of the min costs for each machine that can reach the prize in 100 moves.
    let mut total = 0;
    for machine in Machine::parse_machines(input) {
        let machine = machine?;
        if let Ok((x, y)) = machine.find_solution() {
            if x > 100 || y > 100 {
                continue;
            }
            total += x * A_COST + y * B_COST;
        }
    }
    Ok(total.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> anyhow::Result<()> {
        let input = b"Button A: X+94, Y+34
Button B: X+22, Y+67
Prize: X=8400, Y=5400

Button A: X+26, Y+66
Button B: X+67, Y+21
Prize: X=12748, Y=12176

Button A: X+17, Y+86
Button B: X+84, Y+37
Prize: X=7870, Y=6450

Button A: X+69, Y+23
Button B: X+27, Y+71
Prize: X=18641, Y=10279";

        assert_eq!("480", process(input)?);
        Ok(())
    }
}
