use crate::part1::obstruct_the_guard;

#[tracing::instrument]
pub fn process(input: &[u8]) -> anyhow::Result<String> {
    Ok(obstruct_the_guard(input)?.to_string())
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

        assert_eq!("6", process(input)?);
        Ok(())
    }
}
