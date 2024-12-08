use std::collections::{HashMap, HashSet};

use aoclib::grid::Grid;
use itertools::Itertools;

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
        for pair in locations.iter().combinations(2) {
            let loc_a = pair[0];
            let loc_b = pair[1];
            let dx_dy = loc_a.get_dx_dy(loc_b);

            // TODO: there must be a way to tell which directino to move in for each point?
            let loc_a_1 = *loc_a + dx_dy;
            let loc_a_2 = *loc_a - dx_dy;
            let loc_b_1 = *loc_b - dx_dy;
            let loc_b_2 = *loc_b + dx_dy;
            if loc_a_1 != *loc_b && grid.contains(loc_a_1) {
                positions.insert(loc_a_1);
            }
            if loc_a_2 != *loc_b && grid.contains(loc_a_2) {
                positions.insert(loc_a_2);
            }
            if loc_b_1 != *loc_a && grid.contains(loc_b_1) {
                positions.insert(loc_b_1);
            }
            if loc_b_2 != *loc_a && grid.contains(loc_b_2) {
                positions.insert(loc_b_2);
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

        assert_eq!("14", process(input)?);
        Ok(())
    }
}
