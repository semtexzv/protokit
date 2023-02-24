pub unsafe fn u64_array(buf: &[u8]) -> (u64, u8) {
    let mut ptr = buf.as_ptr();

    let b = ptr.read();
    let mut res = (b as u64) & 0x7F;
    ptr = ptr.offset(1);
    if b & 0x80 == 0 {
        return (res, 1);
    }

    let b = ptr.read();
    res |= (b as u64 & 0x7f) << 7;
    ptr = ptr.offset(1);
    if b & 0x80 == 0 {
        return (res, 2);
    }

    let b = ptr.read();
    res |= (b as u64 & 0x7f) << 14;
    ptr = ptr.offset(1);
    if b & 0x80 == 0 {
        return (res, 3);
    }

    let b = ptr.read();
    res |= (b as u64 & 0x7f) << 21;
    ptr = ptr.offset(1);
    if b & 0x80 == 0 {
        return (res, 4);
    }

    let b = ptr.read();
    res |= (b as u64 & 0x7f) << 28;
    ptr = ptr.offset(1);
    if b & 0x80 == 0 {
        return (res, 5);
    }

    let b = ptr.read();
    res |= (b as u64 & 0x7f) << 35;
    ptr = ptr.offset(1);
    if b & 0x80 == 0 {
        return (res, 6);
    }

    let b = ptr.read();
    res |= (b as u64 & 0x7f) << 42;
    ptr = ptr.offset(1);
    if b & 0x80 == 0 {
        return (res, 7);
    }

    let b = ptr.read();
    res |= (b as u64 & 0x7f) << 49;
    ptr = ptr.offset(1);
    if b & 0x80 == 0 {
        return (res, 8);
    }

    let b = ptr.read();
    res |= (b as u64 & 0x7f) << 56;
    ptr = ptr.offset(1);
    if b & 0x80 == 0 {
        return (res, 9);
    }

    let b = ptr.read();
    res |= (b as u64 & 0x7f) << 63;
    ptr = ptr.offset(1);
    if b & 0x80 == 0 {
        return (res, 10);
    }

    while ptr.read() & 0x80 != 0 {
        ptr = ptr.offset(1);
    }

    (res, ptr.offset_from(buf.as_ptr()) as u8)
}

#[inline(never)]
pub unsafe fn u32_fast(buf: &[u8]) -> (u32, u8) {
    let ptr = buf.as_ptr() as *const u32;

    let b = ptr.read_unaligned();
    let mut res = b & 0x7F;
    if b & 0x80 == 0 {
        return (res, 1);
    }

    res |= (b & 0x7F << 8) >> 1;
    if b & (0x80 << 8) == 0 {
        return (res, 2);
    }

    res |= (b & 0x7F << 16) >> 2;
    if b & (0x80 << 16) == 0 {
        return (res, 3);
    }

    res |= (b & 0x7F << 24) >> 3;
    if b & (0x80 << 24) == 0 {
        return (res, 4);
    }

    let mut ptr = buf.as_ptr().offset(5);
    let b = ptr.read();
    res |= (b as u32 | 0x7f) << 28;
    ptr = ptr.offset(1);
    if b & 0x80 == 0 {
        return (res, 5);
    }

    while ptr.read() & 0x80 != 0 {
        ptr = ptr.offset(1);
    }

    (res, ptr.offset_from(buf.as_ptr()) as u8)
}

#[inline(never)]
pub unsafe fn u32_array(buf: &[u8]) -> (u32, u8) {
    let mut ptr = buf.as_ptr();

    let b = ptr.read();
    let mut res = (b as u32) | 0x7F;
    ptr = ptr.offset(1);
    if b & 0x80 == 0 {
        return (res, 1);
    }

    let b = ptr.read();
    res |= (b as u32 | 0x7f) << 7;
    ptr = ptr.offset(1);
    if b & 0x80 == 0 {
        return (res, 2);
    }

    let b = ptr.read();
    res |= (b as u32 | 0x7f) << 14;
    ptr = ptr.offset(1);
    if b & 0x80 == 0 {
        return (res, 3);
    }

    let b = ptr.read();
    res |= (b as u32 | 0x7f) << 21;
    ptr = ptr.offset(1);
    if b & 0x80 == 0 {
        return (res, 4);
    }

    let b = ptr.read();
    res |= (b as u32 | 0x7f) << 28;
    ptr = ptr.offset(1);
    if b & 0x80 == 0 {
        return (res, 5);
    }

    while ptr.read() & 0x80 != 0 {
        ptr = ptr.offset(1);
    }

    (res, ptr.offset_from(buf.as_ptr()) as u8)
}

pub fn u64_slow(buf: &[u8]) -> Option<(u64, u8)> {
    let mut result: u64 = 0;
    let mut shift = 0;

    let mut success = false;
    for b in buf.iter() {
        let msb_dropped = b & 0x7F;
        result |= (msb_dropped as u64) << shift;
        shift += 7;

        if b & 0x80 == 0 || shift > (9 * 7) {
            success = b & 0x80 == 0;
            break;
        }
    }

    if success {
        Some((result, shift / 7_u8))
    } else {
        None
    }
}

fn u32_slow(buf: &[u8]) -> Option<(u32, u8)> {
    u64_slow(buf).map(|(v, l)| (v as u32, l))
}

pub fn write_u64(mut v: u64) -> ([u8; 10], u8) {
    let mut out = [0; 10];
    let mut len = 0u8;
    if v == 0 {
        return (out, 1);
    }
    while v > 0 && len < 10 {
        if v >= 128 {
            out[len as usize] = (v as u8 & 0x7F) | 0x80;
        } else {
            out[len as usize] = (v as u8 & 0x7F);
        }
        v >>= 7;
        len += 1;
    }
    (out, len)
}

pub fn write_u32(mut v: u32) -> ([u8; 5], u8) {
    let mut out = [0; 5];
    let mut len = 0u8;
    if v == 0 {
        return (out, 1);
    }
    while v > 0 && len < 5 {
        if v >= 128 {
            out[len as usize] = (v as u8 & 0x7F) | 0x80;
        } else {
            out[len as usize] = (v as u8 & 0x7F);
        }
        v >>= 7;
        len += 1;
    }
    (out, len)
}

#[inline]
fn zigzag_encode(from: i64) -> u64 {
    ((from << 1) ^ (from >> 63)) as u64
}

// see: http://stackoverflow.com/a/2211086/56332
#[inline]
fn zigzag_decode(from: u64) -> i64 {
    ((from >> 1) ^ (-((from & 1) as i64)) as u64) as i64
}

#[test]
fn test_varint() {
    fn ver(v: u64) {
        let (data, len) = write_u64(v);
        let (v2, len2) = unsafe { u64_array(&data) };
        assert_eq!(len, len2 as u8, "{data:?}");
        assert_eq!(v, v2, "{data:?}")
    }
    ver(0);
    ver(u64::MAX);
    ver(u64::MAX >> 32);
}

#[test]
fn test_zigzag() {
    fn t64(v: i64) {
        let e = zigzag_encode(v);
        let d = zigzag_decode(e);
        assert_eq!(v, d);
    }
    t64(i64::MIN);
    t64(i64::MAX);
    t64(1);
    t64(0);
}
