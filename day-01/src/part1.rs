use std::fmt::Debug;
use std::path::Path;

#[tracing::instrument]
pub fn process<P: AsRef<Path> + Debug>(_input: P) -> anyhow::Result<String> {
    todo!("day 01 - part 1");
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
        todo!("haven't built test yet");
        let input = "";
        let dir = TempDir::new("test")?;
        let file = dir.path().join("input1.txt");
        write_to_file(input, &file);

        assert_eq!("", process(&file)?);
        Ok(())
    }
}
