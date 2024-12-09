use crate::part1::{checksum, expand, less_naive_compaction};

#[tracing::instrument]
pub fn process(input: &[u8]) -> anyhow::Result<String> {
    let mut expanded = expand(input);
    less_naive_compaction(&mut expanded);
    let checksum = checksum(&expanded);
    Ok(checksum.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> anyhow::Result<()> {
        let input = b"2333133121414131402";

        assert_eq!("2858", process(input)?);
        Ok(())
    }
}
