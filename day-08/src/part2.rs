use std::collections::{HashMap, HashSet};

use aoclib::grid::Grid;

#[tracing::instrument]
pub fn process(input: &[u8]) -> anyhow::Result<String> {
    let grid = Grid::new(input)?;
    let mut antenae = HashMap::new();

    for (letter, point) in grid
        .points()
        .map(|p| (grid.get_point(p), p))
        .filter(|(kind, _point)| *kind != b'.')
    {
        antenae.entry(letter).or_insert(vec![]).push(point)
    }

    // for each antenae type, go through all the pairs of two and find the focal points
    let mut positions = HashSet::new();
    for (_letter, locations) in antenae {
        for first in locations.iter() {
            for second in locations.iter() {
                if first != second {
                    let dist = *second - *first;
                    let mut antinode = *second;

                    while grid.contains(antinode) {
                        positions.insert(antinode);
                        antinode += dist;
                    }
                }
            }
        }
    }

    Ok(positions.len().to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> anyhow::Result<()> {
        let input = b"............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............";

        assert_eq!("34", process(input)?);
        Ok(())
    }
}
