use crate::part1::{Conditional, Mul, Stream};

#[tracing::instrument]
pub fn process(input: &[u8]) -> anyhow::Result<String> {
    let mut stream = Stream::new(input);
    let mut total = 0;
    let mut execute = true;
    // mul\(\d{1,3},\d{1,3}\) -> 678 with regex
    while !stream.is_empty() {
        if let Some(cond) = Conditional::parse_conditional(&mut stream) {
            match cond {
                Conditional::Do => execute = true,
                Conditional::DoNot => execute = false,
            }
        } else if let Some(mul) = Mul::parse_mul(&mut stream) {
            if execute {
                total += mul.mul();
            }
        } else {
            //  Advance forward one, not useful for part 1 but needed for part 2
            let _ = stream.read_byte();
        }
    }
    Ok(total.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> anyhow::Result<()> {
        let input = b"xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";
        assert_eq!("48", process(input)?);
        Ok(())
    }
}
