use crate::part1::{Conditional, Mul, Stream};

#[tracing::instrument]
pub fn process_(input: &[u8]) -> anyhow::Result<String> {
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

use lazy_static::lazy_static;
use regex::bytes::Regex;

lazy_static! {
    static ref RE: Regex = Regex::new(
        r"(?x)
    (?<mul_op>mul\(        # literal match of mul(
        (?<lhs>\d{1,3})    # Capture the inner digit that can be 1-3 digits long
        ,                  # literal literal comma
        (?<rhs>\d{1,3})\)  # Capture the inner digit that can be 1-3 digits long
    )|
    (?<do>do\(\))|         # Can match a do() literal
    (?<dont>don\'t\(\))    # Can match a don't() literal
    "
    )
    .unwrap();
}

pub fn ascii_bytes_to_num(bytes: &[u8]) -> u32 {
    let mut num = 0;
    for byte in bytes {
        num = (num * 10) + (*byte as u32 - 48);
    }
    num
}

pub fn process_regex(input: &[u8]) -> anyhow::Result<String> {
    let mut execute = true;
    let mut total = 0;
    for cap in RE.captures_iter(input) {
        if cap.name("mul_op").is_some() && execute {
            total += ascii_bytes_to_num(&cap["lhs"]) * ascii_bytes_to_num(&cap["rhs"]);
        } else if cap.name("do").is_some() {
            execute = true
        } else if cap.name("dont").is_some() {
            execute = false;
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
