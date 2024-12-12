use aoclib::grid::Grid;

use crate::part1::build_up_regions;

#[tracing::instrument]
pub fn process(input: &[u8]) -> anyhow::Result<String> {
    let grid = Grid::new(input)?;
    let map = build_up_regions(&grid);
    let mut total = 0;
    for (plant, regions) in map {
        for region in regions {
            println!("{}", region);
            let price = region.area() * region.sides();
            println!(
                "A region of {} plants with price of {} * {} = {}",
                char::from(plant),
                region.area(),
                region.sides(),
                price
            );
            total += price
        }
        // total += regions
        //     .iter()
        //     .map(|r| r.area() * r.sides())
        //     .sum::<usize>()
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

        assert_eq!("1206", process(input)?);
        Ok(())
    }
}
