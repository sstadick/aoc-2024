use anyhow::{anyhow, Result};
use aoclib::parsers::try_parse_num;

use crate::part1::Checker;

pub fn parse_input(input: &[u8]) -> Result<u64> {
    let mut sum = 0;
    let mut offset = 0;
    while offset < input.len() {
        // parse a number up till :
        let (answer, bytes_read) =
            try_parse_num::<u64>(&input[offset..]).ok_or(anyhow!("Failed to parse num"))?;
        // Consume a space and :
        offset += 2 + bytes_read;

        let mut checker = Checker::new(answer);
        // parse <number>/s till a newline
        loop {
            let (num, bytes_read) =
                try_parse_num::<u64>(&input[offset..]).ok_or(anyhow!("Failed to parse num"))?;
            offset += bytes_read;

            checker.push_part2(num);

            // If we hit end of line, break loop
            // otherwise just increement offset to skip past space
            if offset >= input.len() || input[offset] == b'\n' {
                offset += 1;
                break;
            }
            offset += 1
        }

        if checker.check() {
            sum += answer;
        }
    }

    Ok(sum)
}

#[tracing::instrument]
pub fn process(input: &[u8]) -> anyhow::Result<String> {
    parse_input(input).map(|x| x.to_string())
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> anyhow::Result<()> {
        let input = b"190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20";

        assert_eq!("11387", process(input)?);
        Ok(())
    }
}
