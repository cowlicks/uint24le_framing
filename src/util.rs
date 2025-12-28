/// Size of a uint24 (24 bits is 3 bytes)
pub(crate) const UINT_24_LENGTH: usize = 3;
#[inline]
/// Return a new buffer with the provided `data` prefixed with it's length
pub(crate) fn wrap_uint24_le(data: &[u8]) -> Vec<u8> {
    let mut buf: Vec<u8> = vec![0; 3];
    let n = data.len();
    write_uint24_le(n, &mut buf);
    buf.extend(data);
    buf
}

#[inline]
/// Write `n` as a little endian usigned 24 bit integrer to the first
/// panics: if `buf` is less than 3 bytes.
pub(crate) fn write_uint24_le(n: usize, buf: &mut [u8]) {
    buf[0] = (n & 255) as u8;
    buf[1] = ((n >> 8) & 255) as u8;
    buf[2] = ((n >> 16) & 255) as u8;
}

/// Read uint24 from the given `buffer` as a `u64`
pub(crate) fn stat_uint24_le(buffer: &[u8]) -> Option<(usize, u64)> {
    if buffer.len() >= 3 {
        let len =
            ((buffer[0] as u32) | ((buffer[1] as u32) << 8) | ((buffer[2] as u32) << 16)) as u64;
        Some((UINT_24_LENGTH, len))
    } else {
        None
    }
}
