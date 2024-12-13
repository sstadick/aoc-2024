#[tracing::instrument]
pub fn process(input: &[u8]) -> anyhow::Result<String> {
    todo!("day 01 - part 1");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> anyhow::Result<()> {
        let input = b"";

        assert_eq!("", process(input)?);
        Ok(())
    }
}
