use std::collections::HashMap;
use std::fmt::Debug;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

use anyhow::Context;

pub fn naive_parse_file<P: AsRef<Path>>(
    file: P,
) -> anyhow::Result<(Vec<usize>, HashMap<usize, usize>)> {
    let reader = BufReader::new(File::open(file)?);
    let mut left = vec![];
    let mut map = HashMap::default();
    for line in reader.lines() {
        let line = line?;
        let first = line
            .split(" ")
            .next()
            .context("Failed to parse first number")?
            .parse::<usize>()?;
        let second = line
            .rsplit(" ")
            .next()
            .context("Failed to parse second number")?
            .parse::<usize>()?;
        *map.entry(second).or_insert(0) += 1;
        left.push(first);
    }
    Ok((left, map))
}

#[tracing::instrument]
pub fn process<P: AsRef<Path> + Debug>(input: P) -> anyhow::Result<String> {
    let (left_side, right_side_counts) = naive_parse_file(input)?;
    let ret = left_side
        .into_iter()
        .map(|left| right_side_counts.get(&left).unwrap_or(&0) * left)
        .sum::<usize>()
        .to_string();
    Ok(ret)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;
    use std::path::Path;
    use tempdir::TempDir;

    fn write_to_file<P: AsRef<Path>>(data: &str, file: P) {
        let mut file = std::fs::File::create(file).unwrap();
        file.write_all(data.as_bytes()).unwrap();
    }

    #[test]
    fn test_process() -> anyhow::Result<()> {
        let input = "3   4
4   3
2   5
1   3
3   9
3   3";
        let dir = TempDir::new("test")?;
        let file = dir.path().join("input2.txt");
        write_to_file(input, &file);

        assert_eq!("31", process(&file)?);
        Ok(())
    }
}
