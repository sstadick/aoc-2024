use crate::part1::{Machine, A_COST, B_COST};

#[tracing::instrument]
pub fn process(input: &[u8]) -> anyhow::Result<String> {
    // Find the sum of the min costs for each machine that can reach the prize in 100 moves.
    let mut total = 0;
    for machine in Machine::parse_machines(input) {
        let mut machine = machine?;
        machine.prize.x += 10000000000000;
        machine.prize.y += 10000000000000;
        // This gives an answer that is too low and I don't know why. I'm guess float arithmetic gets weird.
        // if let Ok((x, y)) = machine.find_solution() {
        //     println!("Soution for {:?}", machine);
        //     total += x * A_COST + y * B_COST;
        // }

        // The smart way to do it: https://www.youtube.com/watch?v=-5J-DAsWuJc&t=306s
        let ax = machine.button_a.x as f64;
        let ay = machine.button_a.y as f64;
        let bx = machine.button_b.x as f64;
        let by = machine.button_b.y as f64;
        let px = machine.prize.x as f64;
        let py = machine.prize.y as f64;
        let ca = (px * by - py * bx) / (ax * by - ay * bx);
        let cb = (px - ax * ca) / bx;
        if (ca % 1.0 == 0.0) && (cb % 1.0 == 0.0) {
            total += (ca.round() as usize * A_COST as usize) + cb.round() as usize
        }
    }
    Ok(total.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> anyhow::Result<()> {
        let input = b"Button A: X+94, Y+34
Button B: X+22, Y+67
Prize: X=8400, Y=5400

Button A: X+26, Y+66
Button B: X+67, Y+21
Prize: X=12748, Y=12176

Button A: X+17, Y+86
Button B: X+84, Y+37
Prize: X=7870, Y=6450

Button A: X+69, Y+23
Button B: X+27, Y+71
Prize: X=18641, Y=10279";

        assert_eq!("480", process(input)?);
        Ok(())
    }
}
