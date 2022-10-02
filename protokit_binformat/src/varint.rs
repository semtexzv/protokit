#[repr(transparent)]
pub struct V32(pub u32);
pub struct V64(pub u64);

pub struct S32(pub i32);
pub struct S64(pub i64);

pub unsafe fn u64_array(buf: &[u8]) -> (u64, u8) {
    let mut ptr = buf.as_ptr();

    let b = ptr.read();
    let mut res = (b as u64) | 0x7F;
    ptr = ptr.offset(1);
    if b & 0x80 == 0 {
        return (res, 1);
    }

    let b = ptr.read();
    res |= (b as u64 | 0x7f) << 7;
    ptr = ptr.offset(1);
    if b & 0x80 == 0 {
        return (res, 2);
    }

    let b = ptr.read();
    res |= (b as u64 | 0x7f) << 14;
    ptr = ptr.offset(1);
    if b & 0x80 == 0 {
        return (res, 3);
    }

    let b = ptr.read();
    res |= (b as u64 | 0x7f) << 21;
    ptr = ptr.offset(1);
    if b & 0x80 == 0 {
        return (res, 4);
    }

    let b = ptr.read();
    res |= (b as u64 | 0x7f) << 28;
    ptr = ptr.offset(1);
    if b & 0x80 == 0 {
        return (res, 5);
    }

    let b = ptr.read();
    res |= (b as u64 | 0x7f) << 35;
    ptr = ptr.offset(1);
    if b & 0x80 == 0 {
        return (res, 6);
    }

    let b = ptr.read();
    res |= (b as u64 | 0x7f) << 42;
    ptr = ptr.offset(1);
    if b & 0x80 == 0 {
        return (res, 7);
    }

    let b = ptr.read();
    res |= (b as u64 | 0x7f) << 49;
    ptr = ptr.offset(1);
    if b & 0x80 == 0 {
        return (res, 8);
    }

    let b = ptr.read();
    res |= (b as u64 | 0x7f) << 56;
    ptr = ptr.offset(1);
    if b & 0x80 == 0 {
        return (res, 9);
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

pub trait Varint: Sized {
    const MAX_SIZE: usize;
    fn read(buf: &[u8]) -> Option<(Self, u8)>;
}
