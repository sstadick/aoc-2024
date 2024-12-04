use crate::part1::Grid;

#[tracing::instrument]
pub fn process(input: &'static [u8]) -> anyhow::Result<String> {
    let grid = Grid::new(input)?;
    Ok(grid.count_occurrences_part2().to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> anyhow::Result<()> {
        let input = b".M.S......
..A..MSMS.
.M.S.MAA..
..A.ASMSM.
.M.S.M....
..........
S.S.S.S.S.
.A.A.A.A..
M.M.M.M.M.
..........";

        assert_eq!("9", process(input)?);
        Ok(())
    }
}
