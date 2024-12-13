use std::fmt::Display;

use aoclib::grid::{Grid, Point, DOWN, LEFT, ORTHOGONAL, RIGHT, UP};
use itertools::Itertools;
use rustc_hash::{FxHashMap, FxHashSet};

#[derive(Debug)]
pub struct Region {
    plant: u8,
    points: FxHashSet<Point>,
}

impl Display for Region {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", char::from(self.plant))?;
        write!(f, ": ")?;
        for point in &self.points {
            write!(f, "({}, {}) ", point.x, point.y)?;
        }
        Ok(())
    }
}

impl Region {
    pub fn new(plant: u8, point: Point) -> Self {
        let mut points = FxHashSet::default();
        points.insert(point);
        Self { plant, points }
    }

    pub fn is_adjacent_to(&self, point: Point) -> bool {
        for dir in ORTHOGONAL {
            if self.points.contains(&(point + dir)) {
                return true;
            }
        }
        false
    }

    /// Check if two Regions are orthogonally adjacent
    pub fn can_merge_with(&self, other: &Self) -> bool {
        let (test_region, contains_region) = if self.area() < other.area() {
            (self, other)
        } else {
            (other, self)
        };

        for point in &test_region.points {
            for dir in ORTHOGONAL {
                if contains_region.points.contains(&(*point + dir)) {
                    return true;
                }
            }
        }
        false
    }

    pub fn merge(&mut self, other: Self) {
        for point in other.points {
            self.add_point(point);
        }
    }

    pub fn add_point(&mut self, point: Point) {
        self.points.insert(point);
    }

    pub fn area(&self) -> usize {
        self.points.len()
    }

    pub fn perimeter(&self) -> usize {
        // Go through every point in the area and get a count of how many of
        // of its four sides face inward.
        let mut total = 0;
        for point in &self.points {
            let mut perimeter = 4;
            for dir in ORTHOGONAL {
                if self.points.contains(&(dir + *point)) {
                    perimeter -= 1;
                }
            }
            total += perimeter
        }
        total
    }

    /// Get all the perimter points
    pub fn perimeter_points(&self) -> impl Iterator<Item = (Point, usize, usize)> + '_ {
        self.points.iter().copied().filter_map(|p| {
            let mut perimeter = 4;
            let mut x_perm = 2;
            for dir in [UP, DOWN] {
                if self.points.contains(&(dir + p)) {
                    perimeter -= 1;
                    x_perm -= 1;
                }
            }
            let mut y_perm = 2;
            for dir in [LEFT, RIGHT] {
                if self.points.contains(&(dir + p)) {
                    perimeter -= 1;
                    y_perm -= 1;
                }
            }
            if perimeter == 0 {
                None
            } else {
                Some((p, x_perm, y_perm))
            }
        })
    }

    pub fn subtract_shared_permis(
        &self,
        a: (Point, usize, usize),
        b: (Point, usize, usize),
    ) -> (Point, usize, usize) {
        let mut a_x = a.1;
        let mut a_y = a.2;
        if !self.points.contains(&(a.0 + UP)) && !self.points.contains(&(b.0 + UP)) {
            a_x = a_x.saturating_sub(1);
        }
        if !self.points.contains(&(a.0 + DOWN)) && !self.points.contains(&(b.0 + DOWN)) {
            a_x = a_x.saturating_sub(1);
        }
        if !self.points.contains(&(a.0 + LEFT)) && !self.points.contains(&(b.0 + LEFT)) {
            a_y = a_y.saturating_sub(1);
        }
        if !self.points.contains(&(a.0 + RIGHT)) && !self.points.contains(&(b.0 + RIGHT)) {
            a_y = a_y.saturating_sub(1);
        }
        (a.0, a_x, a_y)
    }

    pub fn sides(&self) -> usize {
        let mut perimeter_points = self.perimeter_points().collect::<Vec<_>>();
        let mut x_sides = perimeter_points.clone();
        x_sides.sort_by_key(|(p, _, _)| p.x);
        perimeter_points.sort_by_key(|(p, _, _)| p.y);
        let y_sides = perimeter_points;

        let mut score = 0;
        let mut x_counted = FxHashMap::default();
        let mut y_counted = FxHashMap::default();
        for chunk in &x_sides.into_iter().chunk_by(|(p, _, _)| p.x) {
            let points = chunk.1.collect::<Vec<_>>();
            // need to see if an x-adjacent point has already been counted
            for (point, mut x_score, y_score) in &points {
                for dir in [LEFT, RIGHT] {
                    let checked = if let Some(seen) = x_counted.get(&(*point + dir)) {
                        self.subtract_shared_permis((*point, x_score, *y_score), *seen)
                    } else {
                        (*point, x_score, *y_score)
                    };
                    x_score = checked.1;
                }

                score += x_score;
                x_counted.insert(*point, (*point, x_score, *y_score));
            }
            println!("{}: {:?}", score, points);
        }
        println!("--");
        for chunk in &y_sides.into_iter().chunk_by(|(p, _, _)| p.y) {
            let points = chunk.1.collect::<Vec<_>>();
            // need to see if an y-adjacent point has already been counted
            for (point, x_score, mut y_score) in &points {
                for dir in [UP, DOWN] {
                    let checked = if let Some(seen) = y_counted.get(&(*point + dir)) {
                        self.subtract_shared_permis((*point, *x_score, y_score), *seen)
                    } else {
                        (*point, *x_score, y_score)
                    };
                    y_score = checked.2;
                }

                score += y_score;
                y_counted.insert(*point, (*point, *x_score, y_score));
            }
            println!("{}: {:?}", score, points);
        }

        score
    }
}

pub fn is_adjacent(a: &Point, b: &Point) -> bool {
    for dir in ORTHOGONAL {
        if *a + dir == *b {
            return true;
        }
    }
    false
}

pub fn build_up_regions(grid: &Grid) -> FxHashMap<u8, Vec<Region>> {
    let mut map = FxHashMap::default();

    for point in grid.points() {
        let value = grid.get_point(point);
        let possible_regions = map.entry(value).or_insert(vec![]);
        if possible_regions.is_empty() {
            possible_regions.push(Region::new(value, point))
        } else {
            // Search for a region that has a point that is orthogonally adjacent to this point;
            let mut found_region = false;
            for region in possible_regions.iter_mut() {
                if region.is_adjacent_to(point) {
                    region.add_point(point);
                    found_region = true;
                    break;
                }
            }

            if !found_region {
                possible_regions.push(Region::new(value, point));
            }
        }
    }

    // TODO: there _must_ be a better way
    // Merge regions since our point scan can create disjoint regions
    for regions in map.values_mut() {
        loop {
            let mut any_merges_done = false;
            let mut checked_regions = vec![];
            while !regions.is_empty() {
                let mut to_check = regions.remove(regions.len() - 1);
                let mut i = 0;
                while i < regions.len() {
                    if to_check.can_merge_with(&regions[i]) {
                        any_merges_done = true;
                        to_check.merge(regions.remove(i));
                    } else {
                        i += 1;
                    }
                }
                checked_regions.push(to_check);
            }
            *regions = checked_regions;

            // keep merging till there's no more to merge
            if !any_merges_done {
                break;
            }
        }
    }

    map
}

#[tracing::instrument]
pub fn process(input: &[u8]) -> anyhow::Result<String> {
    let grid = Grid::new(input)?;
    let map = build_up_regions(&grid);
    let mut total = 0;
    for (_plant, regions) in map {
        // for region in regions {
        //     let price = region.area() * region.perimeter();
        //     println!("{}", region);
        //     println!(
        //         "A region of {} plants with price of {} * {} = {}",
        //         char::from(plant),
        //         region.area(),
        //         region.perimeter(),
        //         price
        //     );
        //     total += price
        // }
        total += regions
            .iter()
            .map(|r| r.area() * r.perimeter())
            .sum::<usize>()
    }
    Ok(total.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> anyhow::Result<()> {
        let input = b"RRRRIICCFF
RRRRIICCCF
VVRRRCCFFF
VVRCCCJFFF
VVVVCJJCFE
VVIVCCJJEE
VVIIICJJEE
MIIIIIJJEE
MIIISIJEEE
MMMISSJEEE";

        assert_eq!("1930", process(input)?);
        Ok(())
    }
}
