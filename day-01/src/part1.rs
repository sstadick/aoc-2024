use std::fmt::Debug;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

use anyhow::Context;

pub fn naive_parse_file<P: AsRef<Path>>(file: P) -> anyhow::Result<(Vec<usize>, Vec<usize>)> {
    let reader = BufReader::new(File::open(file)?);
    let mut firsts = vec![];
    let mut seconds = vec![];
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
        firsts.push(first);
        seconds.push(second);
    }
    firsts.sort_unstable();
    seconds.sort_unstable();
    Ok((firsts, seconds))
}

#[tracing::instrument]
pub fn process<P: AsRef<Path> + Debug>(input: P) -> anyhow::Result<String> {
    let (first_side, second_side) = naive_parse_file(input)?;
    let ret = first_side
        .into_iter()
        .zip(second_side.into_iter())
        .map(|(a, b)| a.abs_diff(b))
        .sum::<usize>();

    Ok(ret.to_string())
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
        let file = dir.path().join("input1.txt");
        write_to_file(input, &file);

        assert_eq!("11", process(&file)?);
        Ok(())
    }
}
