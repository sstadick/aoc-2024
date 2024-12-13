#[tracing::instrument]
pub fn process(input: &[u8]) -> anyhow::Result<String> {
    todo!("day 01 - part 2");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> anyhow::Result<()> {
        todo!("haven't built test yet");
        let input = b"";

        assert_eq!("", process(input)?);
        Ok(())
    }
}
