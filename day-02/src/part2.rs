use std::fmt::Debug;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

use crate::part1::Safety;

#[tracing::instrument]
pub fn process<P: AsRef<Path> + Debug>(input: P) -> anyhow::Result<String> {
    let reader = BufReader::new(File::open(input)?);
    let mut count_safe_reports = 0;
    for line in reader.lines() {
        let line = line?;
        let level_iter = line.split_ascii_whitespace().map(|num| {
            num.parse::<usize>()
                .expect("Failed to parse int from level")
        });
        let safety_report = Safety::check_safety(level_iter);
        match safety_report.make_safe() {
            Safety::Safe => count_safe_reports += 1,
            Safety::Unsafe => (),
        }
    }
    Ok(count_safe_reports.to_string())
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
        let input = "7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9";
        let dir = TempDir::new("test")?;
        let file = dir.path().join("input2.txt");
        write_to_file(input, &file);

        assert_eq!("4", process(&file)?);
        Ok(())
    }
}
