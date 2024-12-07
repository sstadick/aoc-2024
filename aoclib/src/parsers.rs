use num::{FromPrimitive, Num};

#[inline]
pub fn is_ascii_num(byte: u8) -> bool {
    matches!(byte, 48..=57)
}

/// Try to parse a num from a byte slice.
///
/// The returned result is None if no number could be parsed.
/// The second element of the returned tuple is the number of bytes read, which will not include the delimiter
pub fn try_parse_num<T>(bytes: &[u8]) -> Option<(T, usize)>
where
    T: Num + FromPrimitive + From<u32>,
{
    let mut bytes_read = 0;
    let mut num = T::zero();
    while bytes_read < bytes.len() && is_ascii_num(bytes[bytes_read]) {
        // TODO: is there a better way to handle "constants" of type T?
        num = (num * T::from(10)) + (T::from_u8(bytes[bytes_read])? - T::from(48));
        bytes_read += 1;
    }

    if bytes_read == 0 {
        return None;
    }
    Some((num, bytes_read))
}
