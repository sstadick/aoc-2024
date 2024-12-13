pub struct Stream<'a> {
    bytes: &'a [u8],
    pub offset: usize,
}

impl<'a> Stream<'a> {
    pub fn new(bytes: &'a [u8]) -> Self {
        Self { bytes, offset: 0 }
    }

    pub fn read_byte(&mut self) -> Option<u8> {
        self.offset += 1;
        self.bytes.get(self.offset).copied()
    }

    pub fn unread_byte(&mut self) {
        self.offset = self.offset.saturating_sub(1);
    }

    pub fn unread_bytes(&mut self, num_bytes: usize) {
        self.offset = self.offset.saturating_sub(num_bytes);
    }

    pub fn is_empty(&self) -> bool {
        self.offset >= self.bytes.len()
    }
}

#[derive(Debug)]
pub enum Conditional {
    Do,
    DoNot,
}

impl Conditional {
    const PREFIX: &[u8] = b"do";
    const DO_NOT_SUFFIX: &[u8] = b"n't()";
    const DO_SUFFIX: &[u8] = b"()";
    pub fn parse_conditional(stream: &mut Stream) -> Option<Self> {
        let mut index_into_prefix = 0;
        let mut index_into_suffix = 0;
        let mut in_dont: bool = false;
        let mut suffix = Self::DO_SUFFIX;
        let mut bytes_read = 0;
        while let Some(byte) = stream.read_byte() {
            bytes_read += 1;
            if index_into_prefix != Self::PREFIX.len() {
                // Match the `do`
                if byte == Self::PREFIX[index_into_prefix] {
                    index_into_prefix += 1;
                } else {
                    stream.unread_bytes(bytes_read);
                    return None;
                }
            } else {
                // Match either `()` or `n't()`
                if index_into_suffix == 0 {
                    // This is the first suffix character and determines do/dont
                    if byte == Self::DO_SUFFIX[0] {
                        // Do
                        index_into_suffix += 1;
                        in_dont = false;
                        suffix = Self::DO_SUFFIX;
                    } else if byte == Self::DO_NOT_SUFFIX[0] {
                        // Do Not
                        index_into_suffix += 1;
                        in_dont = true;
                        suffix = Self::DO_NOT_SUFFIX;
                    } else {
                        stream.unread_bytes(bytes_read);
                        return None;
                    }
                } else if index_into_suffix != suffix.len() {
                    if byte == suffix[index_into_suffix] {
                        index_into_suffix += 1;
                    } else {
                        stream.unread_bytes(bytes_read);
                        return None;
                    }
                } else {
                    // We did it!
                    if in_dont {
                        stream.unread_byte(); // We've gone one byte too far
                        return Some(Self::DoNot);
                    } else {
                        stream.unread_byte(); // We've gone one byte too far
                        return Some(Self::Do);
                    }
                }
            }
        }
        stream.unread_bytes(bytes_read);
        None
    }
}

/// A mul instruction looks like: `mul(\d{1,3},\d{1,3})`
#[derive(Debug)]
pub struct Mul {
    lhs: u16,
    rhs: u16,
}

impl Mul {
    const PREFIX: &[u8] = b"mul(";
    pub fn parse_mul(stream: &mut Stream) -> Option<Self> {
        let mut index_into_prefix = 0;
        let mut lhs = None;
        let mut rhs = None;
        let mut comma_seen = false;
        let mut bytes_read = 0;
        while let Some(byte) = stream.read_byte() {
            bytes_read += 1;
            if index_into_prefix != Self::PREFIX.len() {
                // Match the `mul(` prefix
                if byte == Self::PREFIX[index_into_prefix] {
                    index_into_prefix += 1;
                } else {
                    stream.unread_bytes(bytes_read);
                    return None;
                }
            } else {
                // match the `\d{1,3},\d{1,3})
                match byte {
                    48..=57 => {
                        // Digit
                        let num = (byte - 48) as u16;
                        match (lhs.as_mut(), rhs.as_mut()) {
                            (None, None) => lhs = Some(num),
                            (None, Some(_)) => {
                                unreachable!("rhs should never be fillling with no lhs")
                            }
                            (Some(lhs), None) => {
                                if !comma_seen {
                                    *lhs = (*lhs * 10) + num;
                                } else {
                                    rhs = Some(num)
                                }
                            }
                            (Some(_), Some(rhs)) => {
                                *rhs = (*rhs * 10) + num;
                            }
                        }
                    }
                    44 => {
                        // comma
                        comma_seen = true
                    }
                    41 => {
                        // Right paren
                        match (lhs, rhs) {
                            (Some(lhs), Some(rhs)) => {
                                return Some(Self { lhs, rhs });
                            }
                            _ => {
                                stream.unread_bytes(bytes_read);
                                return None;
                            }
                        }
                    }
                    _ => {
                        // Anything else is not valid
                        stream.unread_bytes(bytes_read);
                        return None;
                    }
                }
            }
        }

        stream.unread_bytes(bytes_read);
        None
    }

    pub fn mul(&self) -> u32 {
        self.lhs as u32 * self.rhs as u32
    }
}

#[inline]
pub fn get_digit(byte: u8) -> Option<u16> {
    match byte {
        48..=57 => Some((byte - 48) as u16),
        _ => None,
    }
}

#[tracing::instrument]
pub fn process(input: &[u8]) -> anyhow::Result<String> {
    let mut stream = Stream::new(input);
    let mut total = 0;
    while !stream.is_empty() {
        if let Some(mul) = Mul::parse_mul(&mut stream) {
            total += mul.mul();
        } else {
            //  Advance forward one, not useful for part 1 but needed for part 2
            let _ = stream.read_byte();
        }
    }
    Ok(total.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> anyhow::Result<()> {
        let input = b"xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";

        assert_eq!("161", process(input)?);
        Ok(())
    }
}
