use aoclib::{count_digits, parsers::try_parse_num};
use rustc_hash::{FxBuildHasher, FxHashMap};

#[inline]
pub fn blink(num: u64) -> (u64, Option<u64>) {
    if num == 0 {
        return (1, None);
    }

    let digit_count = count_digits(num);
    if digit_count % 2 == 0 {
        let lhs = num / (10_u64.pow(digit_count / 2));
        let rhs = num % (10_u64.pow(digit_count / 2));
        return (lhs, Some(rhs));
    }

    (num * 2024, None)
}

pub fn parse_stones(input: &[u8]) -> impl Iterator<Item = u64> + '_ {
    let mut offset = 0;
    std::iter::from_fn(move || {
        if offset > input.len() - 1 {
            return None;
        }
        if let Some((num, bytes_read)) = try_parse_num::<u64>(&input[offset..]) {
            offset += bytes_read + 1;
            return Some(num);
        }
        None
    })
}

pub fn stone_counter_acc(input: &[u8], blinks: usize) -> usize {
    let mut stones = FxHashMap::default();
    for num in parse_stones(input) {
        stones.insert(num, 1);
    }

    for _ in 0..blinks {
        // let mut new_stones = FxHashMap::with_capacity(stones.len());
        let mut new_stones =
            FxHashMap::with_capacity_and_hasher(stones.len(), FxBuildHasher::default());
        for (stone, count) in &stones {
            let (lhs, rhs) = blink(*stone);
            *new_stones.entry(lhs).or_insert(0) += count;
            if let Some(rhs) = rhs {
                *new_stones.entry(rhs).or_insert(0) += count;
            }
        }
        stones = new_stones;
    }
    stones.values().copied().sum::<usize>()
}

pub fn stone_counter(input: &[u8], blinks: usize) -> usize {
    let mut count = 0;

    for num in parse_stones(input) {
        // Do 25 blinks on a number and count the result
        let mut stack = vec![num];

        for _ in 0..blinks {
            let mut new_stack = vec![];
            for value in stack {
                let (lhs, rhs) = blink(value);
                new_stack.push(lhs);
                if let Some(rhs) = rhs {
                    new_stack.push(rhs);
                }
            }
            stack = new_stack;
        }
        count += stack.len();
    }
    count
}

#[tracing::instrument]
pub fn process(input: &[u8]) -> anyhow::Result<String> {
    let count = stone_counter_acc(input, 25);
    Ok(count.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[test]
    fn test_process() -> anyhow::Result<()> {
        let input = b"125 17";

        assert_eq!("55312", process(input)?);
        Ok(())
    }

    #[rstest]
    #[case(0, (1, None))]
    #[case(125, (253000, None))]
    #[case(17, (1, Some(7)))]
    #[case(253000, (253, Some(0)))]
    fn test_blink(#[case] input: u64, #[case] expected: (u64, Option<u64>)) {
        assert_eq!(blink(input), expected)
    }
}
