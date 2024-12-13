pub mod grid;
pub mod parsers;

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[inline]
pub fn count_digits(n: u64) -> u32 {
    ((n as f64).log10() + 1.0).floor() as u32
}

/// Concatenate two numbers. EX: concat_u64(1234, 5768) = 12345768
///
/// Determine the number of digits in b (4), then multiply a by 10^digits
/// So a will now be 12340000, then add b.
pub fn concat_u64(a: u64, b: u64) -> u64 {
    if b == 0 {
        return a * 10;
    }
    let mut digits = 0;
    let mut n = b;
    while n > 0 {
        digits += 1;
        n /= 10;
    }
    (a * 10_u64.pow(digits)) + b
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    #[rstest]
    #[case(17, 2)]
    #[case(100, 3)]
    #[case(9999, 4)]
    #[case(22345, 5)]
    #[case(123456, 6)]
    #[case(1000000, 7)]
    fn test_count_digits(#[case] input: u64, #[case] expected: u32) {
        assert_eq!(count_digits(input), expected);
    }
}
