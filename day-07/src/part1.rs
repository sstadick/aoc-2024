use anyhow::{anyhow, Result};
use aoclib::{concat_u64, parsers::try_parse_num};

#[derive(Debug, Clone)]
pub struct Checker {
    answer: u64,
    layer: Vec<u64>,
}

impl Checker {
    pub fn new(answer: u64) -> Self {
        Self {
            answer,
            layer: vec![],
        }
    }

    pub fn push(&mut self, number: u64) {
        if self.layer.is_empty() {
            self.layer.push(number);
            return;
        }
        let mut new_layer = Vec::with_capacity(self.layer.len() * 2);
        for outcome in &self.layer {
            new_layer.push(outcome + number);
            new_layer.push(outcome * number);
        }
        self.layer = new_layer;
    }

    pub fn push_part2(&mut self, number: u64) {
        if self.layer.is_empty() {
            self.layer.push(number);
            return;
        }
        let mut new_layer = Vec::with_capacity(self.layer.len() * 2);
        for outcome in &self.layer {
            new_layer.push(outcome + number);
            new_layer.push(outcome * number);
            new_layer.push(concat_u64(*outcome, number));
        }
        self.layer = new_layer;
    }

    pub fn check(&self) -> bool {
        self.layer.iter().any(|value| *value == self.answer)
    }
}

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

            checker.push(num);

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

        assert_eq!("3749", process(input)?);
        Ok(())
    }
}
