use std::cmp::Ordering;
use std::fmt::Debug;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

pub struct SafetyReport {
    levels: Vec<usize>,
    safety: Safety,
}

impl SafetyReport {
    pub fn make_safe(&self) -> Safety {
        if self.safety == Safety::Safe {
            return Safety::Safe;
        }

        // Try removing levels
        for level_to_remove in 0..self.levels.len() {
            let level_iter = self
                .levels
                .iter()
                .enumerate()
                .filter(|(i, _level)| *i != level_to_remove)
                .map(|(_i, level)| *level);
            let new_report = Safety::check_safety(level_iter);
            if new_report.safety == Safety::Safe {
                return Safety::Safe;
            }
        }
        self.safety
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Safety {
    Safe,
    Unsafe,
}

impl Safety {
    pub fn check_safety(report: impl Iterator<Item = usize>) -> SafetyReport {
        let mut prev = None;
        let mut direction = None;
        let mut overall_safety = Safety::Safe;
        let mut levels = vec![];

        for level in report {
            match (prev, direction) {
                (None, None) => {
                    prev = Some(level);
                }
                (None, Some(_direction)) => {
                    unreachable!("Can't have a prev direction and no previous value.")
                }
                (Some(previous_level), None) => {
                    let new_dir = Direction::get_direction(previous_level, level);
                    let diff = previous_level.abs_diff(level);
                    if !(1..=3).contains(&diff) {
                        overall_safety = Self::Unsafe;
                    }

                    direction = Some(new_dir);
                    prev = Some(level);
                }
                (Some(previous_level), Some(previous_direction)) => {
                    let new_dir = Direction::get_direction(previous_level, level);
                    let diff = previous_level.abs_diff(level);
                    if !(1..=3).contains(&diff) || previous_direction != new_dir {
                        overall_safety = Self::Unsafe;
                    }

                    direction = Some(new_dir);
                    prev = Some(level);
                }
            }
            levels.push(level);
        }

        SafetyReport {
            levels,
            safety: overall_safety,
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Direction {
    Increase,
    Decrease,
    Same,
}

impl Direction {
    pub fn get_direction(left: usize, right: usize) -> Direction {
        match left.cmp(&right) {
            Ordering::Less => Self::Increase,
            Ordering::Equal => Self::Same,
            Ordering::Greater => Self::Decrease,
        }
    }
}

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
        let safety = Safety::check_safety(level_iter);
        match safety.safety {
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
1 2 7 8
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9";
        let dir = TempDir::new("test")?;
        let file = dir.path().join("input1.txt");
        write_to_file(input, &file);

        assert_eq!("2", process(&file)?);
        Ok(())
    }
}
