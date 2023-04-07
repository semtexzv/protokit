#![allow(unused_parens)]
use core::ops::{BitOrAssign, Shl};

pub fn decode<T>(buf: &[u8]) -> (T, u8)
where
    T: Default + From<u8> + Shl<u8, Output = T> + BitOrAssign,
{
    let mut result: T = T::default();
    let mut shift = 0;

    let mut success = false;
    for b in buf.iter() {
        let msb_dropped = b & 0x7F;
        result |= (T::from(msb_dropped)) << shift;
        shift += 7;

        if b & 0x80 == 0 || shift > (9 * 7) {
            success = b & 0x80 == 0;
            break;
        }
    }

    if success {
        (result, shift / 7_u8)
    } else {
        (result, 0)
    }
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
pub fn zigzag_encode64(from: i64) -> u64 {
    ((from << 1) ^ (from >> 63)) as u64
}

#[inline]
pub fn zigzag_encode32(from: i32) -> u32 {
    ((from << 1) ^ (from >> 31)) as u32
}

// see: http://stackoverflow.com/a/2211086/56332
#[inline]
pub fn zigzag_decode64(from: u64) -> i64 {
    ((from >> 1) ^ (-((from & 1) as i64)) as u64) as i64
}

// see: http://stackoverflow.com/a/2211086/56332
#[inline]
pub fn zigzag_decode32(from: u32) -> i32 {
    ((from >> 1) ^ (-((from & 1) as i32)) as u32) as i32
}

#[test]
fn test_varint() {
    fn ver(v: u64) {
        let (data, len) = write_u64(v);
        let (v2, len2) = unsafe { decode::<u64>(&data) };
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
        let e = zigzag_encode64(v);
        let d = zigzag_decode64(e);
        assert_eq!(v, d);
    }
    t64(i64::MIN);
    t64(i64::MAX);
    t64(1);
    t64(0);
    t64(-1);
    t64(-2);
}
