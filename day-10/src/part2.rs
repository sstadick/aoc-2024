use aoclib::grid::Grid;

use crate::part1::part2_sum_trailheads;

#[tracing::instrument]
pub fn process(input: &[u8]) -> anyhow::Result<String> {
    let grid = Grid::new(input)?;
    Ok(part2_sum_trailheads(&grid).to_string())
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

        assert_eq!("81", process(input)?);
        Ok(())
    }
}
