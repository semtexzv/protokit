use std::cmp::min;
use std::fmt::{Debug, Formatter};
use std::marker::PhantomData;
use std::mem::size_of;
use std::os::raw::c_void;
use std::{mem, usize};

use integer_encoding::VarInt;
pub use memoffset::offset_of;
pub type Tag = u32;
pub type Ret = usize;

pub const fn tag(mut v: u32) -> Tag {
    let mut buf = [0u8; 4];
    let mut i = 0;

    while v > 0 {
        buf[i] = (v & 0b0111_1111) as u8;
        if v > 128 {
            buf[i] |= 0b1000_0000;
        }
        v >>= 7;
        i += 1;
    }
    u32::from_le_bytes(buf)
}

pub struct DecodeState {
    pub info: MessageInfo,
    pub error: Option<Box<String>>,
}

pub type ParseFn = unsafe fn(obj: *mut c_void, buf: &[u8], field: &FieldInfo, state: &mut DecodeState) -> Ret;

#[derive(Copy, Clone)]
pub struct MessageInfo {
    pub dispatch: ParseFn,
    pub fields: &'static [FieldInfo],
    pub name: &'static str,
}

impl MessageInfo {
    pub const fn sorted(fields: &'static [FieldInfo], name: &'static str) -> Self {
        Self {
            dispatch: dispatch1,
            fields,
            name,
        }
    }
}

impl Debug for MessageInfo {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("MessageInfo")
            .field("name", &self.name)
            .field("fields", &&self.fields)
            .finish()
    }
}

pub struct FieldInfo {
    pub tag: Tag,
    pub offset: isize,
    pub parser: ParseFn,
    pub number: u32,
}

impl Debug for FieldInfo {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("FieldInfo")
            .field("tag", &self.tag)
            .field("offset", &self.offset)
            .finish()
    }
}

pub trait TableDecodable {
    fn info(&self) -> MessageInfo;
}

#[cold]
#[inline(never)]
fn error(obj: *mut c_void, buf: &[u8], field: &FieldInfo, state: &mut DecodeState) -> Ret {
    #[inline(never)]
    fn _impl(_obj: *mut c_void, _buf: &[u8], _field: &FieldInfo, _state: &mut DecodeState) -> Ret {
        panic!("Error");
        1
    }
    _impl(obj, buf, field, state)
}

// We need to make the outer fn tail call as well
// it seems that if the function is not a tail call, we get bunch of stack management on callers
#[cold]
#[inline(never)]
fn empty(obj: *mut c_void, buf: &[u8], field: &FieldInfo, state: &mut DecodeState) -> Ret {
    #[inline(never)]
    fn _impl(_obj: *mut c_void, _buf: &[u8], _field: &FieldInfo, state: &mut DecodeState) -> Ret {
        state.error = Some(Box::new("More data".to_string()));
        0
    }
    _impl(obj, buf, field, state)
}

#[cold]
#[inline(never)]
fn not_found(obj: *mut c_void, buf: &[u8], field: &FieldInfo, state: &mut DecodeState) -> Ret {
    #[inline(never)]
    fn _impl(_obj: *mut c_void, buf: &[u8], _field: &FieldInfo, state: &mut DecodeState) -> Ret {
        // state.error = Some("field not found");
        state.error = Some(Box::new(format!(
            "Not found: {:#?}, buf: {:#?}",
            state.info,
            &buf[0 ..]
        )));
        println!("Not found: {:#?}, buf: {:#?}", state.info, &buf[0 .. min(buf.len(), 4)]);
        1
    }
    _impl(obj, buf, field, state)
}

#[cold]
#[inline(never)]
fn done(obj: *mut c_void, buf: &[u8], field: &FieldInfo, state: &mut DecodeState) -> Ret {
    #[inline(never)]
    fn _impl(_obj: *mut c_void, _buf: &[u8], _field: &FieldInfo, _state: &mut DecodeState) -> Ret {
        1
    }
    _impl(obj, buf, field, state)
}

#[cold]
#[inline(never)]
pub unsafe fn dispatch4(obj: *mut c_void, buf: &[u8], field: &FieldInfo, state: &mut DecodeState) -> Ret {
    if buf.is_empty() {
        return done(obj, buf, field, state);
    }

    let mut tag: [u8; 4] = [0; 4];
    for i in 0 .. min(buf.len(), 4) {
        tag[i] = buf[i]
    }

    let tag = u32::from_le_bytes(tag);
    let infos = &state.info.fields;
    if let Ok(v) = infos.binary_search_by_key(&tag, |b| b.tag) {
        return (infos[v].parser)(obj, buf, &infos[v], state);
    }
    not_found(obj, buf, field, state)
}

#[cold]
#[inline(never)]
pub unsafe fn dispatch2(obj: *mut c_void, buf: &[u8], field: &FieldInfo, state: &mut DecodeState) -> Ret {
    if buf.is_empty() {
        return done(obj, buf, field, state);
    }
    if buf.len() < 2 || buf[1] > 0x80 {
        return dispatch4(obj, buf, field, state);
    }

    let tag : u32  = u16::from_le_bytes(unsafe { *(buf as *const [u8] as *const [u8; 2]) }) as u32;

    let infos = &state.info.fields;
    if let Ok(v) = infos.binary_search_by_key(&tag, |b| b.tag) {
        return (infos[v].parser)(obj, buf, &infos[v], state);
    }
    not_found(obj, buf, field, state)
}

#[inline(never)]
pub unsafe fn dispatch1(obj: *mut c_void, buf: &[u8], field: &FieldInfo, state: &mut DecodeState) -> Ret {
    if buf.is_empty() || buf[0] > 0x80 {
        return dispatch2(obj, buf, field, state);
    }

    let tag = buf[0] as u32;
    for info in state.info.fields {
        if info.tag == tag as _ {
            return (info.parser)(obj, buf, info, state);
        }
        if info.tag > tag as _ {
            break;
        }
    }
    not_found(obj, buf, field, state)
}

pub trait Tabular {
    unsafe fn decode(obj: *mut c_void, buf: &[u8], field: &FieldInfo, state: &mut DecodeState) -> Ret;
}

pub struct Wrap<F, T, const O: usize> {
    _t: PhantomData<(F, T)>
}

// Library does automatic zigzag encoding, we need to decode to u64 and then transmute if we don't want it
pub trait UnsignedVariant {
    type DecodeType: VarInt;
}

impl UnsignedVariant for u64 {
    type DecodeType = u64;
}

impl UnsignedVariant for u32 {
    type DecodeType = u32;
}

impl UnsignedVariant for i64 {
    type DecodeType = u64;
}

impl UnsignedVariant for i32 {
    type DecodeType = u32;
}

pub struct SInt<T, const O: usize>(PhantomData<T>);

impl<T: VarInt, const O: usize> Tabular for SInt<T, O> {
    #[inline(never)]
    unsafe fn decode(obj: *mut c_void, buf: &[u8], field: &FieldInfo, state: &mut DecodeState) -> Ret {
        if let Some((value, len)) = T::decode_var(&buf[O ..]) {
            let ptr = obj.offset(field.offset) as *mut T;
            *ptr = value;
            // set_bit(obj, field.number);
            (state.info.dispatch)(obj, &buf[O + len ..], field, state)
        } else {
            empty(obj, buf, field, state)
        }
    }
}


pub struct VInt<T, const O: usize>(PhantomData<T>);

impl<T: VarInt + UnsignedVariant, const O: usize> Tabular for VInt<T, O> {
    #[inline(never)]
    unsafe fn decode(obj: *mut c_void, buf: &[u8], field: &FieldInfo, state: &mut DecodeState) -> Ret {
        if let Some((value, len)) = T::DecodeType::decode_var(&buf[O ..]) {
            let ptr = obj.offset(field.offset) as *mut T;
            *ptr = *mem::transmute::<_, &T>(&value);
            // set_bit(obj, field.number);
            (state.info.dispatch)(obj, &buf[O + len ..], field, state)
        } else {
            empty(obj, buf, field, state)
        }
    }
}

impl<T: VarInt + UnsignedVariant, const O: usize> Tabular for VInt<Vec<T>, O> {
    unsafe fn decode(obj: *mut c_void, buf: &[u8], field: &FieldInfo, state: &mut DecodeState) -> Ret {
        let ptr = &mut *(obj.offset(field.offset) as *mut Vec<T>);
        let dst = ptr.spare_capacity_mut();
        if dst.is_empty() {
            return error(obj, buf, field, state);
        }

        if let Some((value, len)) = T::DecodeType::decode_var(&buf[O ..]) {
            dst.get_unchecked_mut(0).write(*mem::transmute::<_, &T>(&value));
            ptr.set_len(ptr.len() + 1);

            // set_bit(obj, field.number);
            (state.info.dispatch)(obj, &buf[O + len ..], field, state)
        } else {
            empty(obj, buf, field, state)
        }
    }
}

pub struct PackedVInt<T: VarInt, const O: usize>(PhantomData<T>);

impl<T: VarInt, const O: usize> Tabular for PackedVInt<T, O> {
    unsafe fn decode(obj: *mut c_void, buf: &[u8], field: &FieldInfo, state: &mut DecodeState) -> Ret {
        if let Some((len, lenlen)) = usize::decode_var(&buf[O ..]) {
            if buf.len() < len + lenlen + O {
                return empty(obj, buf, field, state);
            }
            let ptr: &mut Vec<T> = (obj.offset(field.offset) as *mut Vec<T>).as_mut().unwrap_unchecked();

            let mut buf = &buf[O + lenlen .. O + lenlen + len];

            while let Some((val, vallen)) = T::decode_var(buf) {
                ptr.push(val);
                buf = &buf[vallen ..];
            }
            // Invalid varint encountered
            if !buf.is_empty() {
                return not_found(obj, buf, field, state);
            }
            buf = &buf[O + lenlen + len ..];
        }
        // set_bit(obj, field.number);
        (state.info.dispatch)(obj, buf, field, state)
    }
}

pub struct Fixed<T: Copy, const O: usize>(PhantomData<T>);

impl<T: Copy, const O: usize> Tabular for Fixed<T, O> {
    unsafe fn decode(obj: *mut c_void, buf: &[u8], field: &FieldInfo, state: &mut DecodeState) -> Ret {
        if buf.len() < size_of::<T>() + O {
            return empty(obj, buf, field, state);
        }
        let dst_ptr = obj.offset(field.offset) as *mut T;
        let src_ptr = buf.as_ptr() as *const T;
        dst_ptr.write(src_ptr.read_unaligned());
        // set_bit(obj, field.number);
        (state.info.dispatch)(obj, &buf[O + size_of::<T>() ..], field, state)
    }
}

pub struct PackedFixed<T, const O: usize>(PhantomData<T>);

impl<T: Default + Copy, const O: usize> Tabular for PackedFixed<T, O> {
    unsafe fn decode(obj: *mut c_void, buf: &[u8], field: &FieldInfo, state: &mut DecodeState) -> Ret {
        if let Some((len, lenlen)) = usize::decode_var(&buf[O ..]) {
            if buf.len() < len + lenlen + O {
                return empty(obj, buf, field, state);
            }
            let ptr: &mut Vec<T> = (obj.offset(field.offset) as *mut Vec<T>).as_mut().unwrap_unchecked();

            let old_items = ptr.len();
            let new_items = len / size_of::<T>();
            ptr.resize(old_items + new_items, T::default());

            let dst_ptr = ptr[old_items ..].as_mut_ptr();
            let src_ptr = buf[O + lenlen .. O + lenlen + len].as_ptr() as *const T;

            for i in 0 .. new_items as isize {
                dst_ptr.offset(i).write(src_ptr.offset(i).read_unaligned())
            }

            // set_bit(obj, field.number);
            (state.info.dispatch)(obj, &buf[O + len + lenlen ..], field, state)
        } else {
            empty(obj, buf, field, state)
        }
    }
}

pub trait ExtendFromBytes {
    fn extend_from_bytes(&mut self, bytes: &[u8], state: &mut Option<Box<String>>) -> bool;
}

impl ExtendFromBytes for Box<str> {
    fn extend_from_bytes(&mut self, bytes: &[u8], state: &mut Option<Box<String>>) -> bool {
        match std::str::from_utf8(bytes) {
            Ok(s) => {
                let mut str = std::mem::replace(self, Box::from("")).into_string();
                str.push_str(s);
                *self = str.into_boxed_str();
                true
            }
            Err(e) => {
                *state = Some(Box::new(format!("{e:?}")));
                false
            }
        }
    }
}

impl ExtendFromBytes for String {
    #[inline(always)]
    fn extend_from_bytes(&mut self, bytes: &[u8], state: &mut Option<Box<String>>) -> bool {
        match std::str::from_utf8(bytes) {
            Ok(s) => {
                self.push_str(s);
                true
            }
            Err(e) => {
                *state = Some(Box::new(format!("{e:?}")));
                false
            }
        }
    }
}

impl ExtendFromBytes for Vec<String> {
    #[inline(always)]
    fn extend_from_bytes(&mut self, bytes: &[u8], state: &mut Option<Box<String>>) -> bool {
        match std::str::from_utf8(bytes) {
            Ok(s) => {
                self.push(s.to_string());
                true
            }
            Err(e) => {
                *state = Some(Box::new(format!("{e:?}")));
                false
            }
        }
    }
}

impl ExtendFromBytes for Vec<u8> {
    fn extend_from_bytes(&mut self, bytes: &[u8], _state: &mut Option<Box<String>>) -> bool {
        self.extend_from_slice(bytes);
        true
    }
}

impl ExtendFromBytes for Box<[u8]> {
    fn extend_from_bytes(&mut self, bytes: &[u8], _state: &mut Option<Box<String>>) -> bool {
        let mut v = std::mem::take(self).into_vec();
        v.extend_from_slice(bytes);
        *self = v.into_boxed_slice();
        true
    }
}

impl ExtendFromBytes for Vec<Vec<u8>> {
    fn extend_from_bytes(&mut self, _bytes: &[u8], _state: &mut Option<Box<String>>) -> bool {
        todo!()
    }
}

pub struct Bytes<T, const O: usize>(PhantomData<T>);

impl<T: ExtendFromBytes, const O: usize> Tabular for Bytes<T, O> {
    unsafe fn decode(obj: *mut c_void, buf: &[u8], field: &FieldInfo, state: &mut DecodeState) -> Ret {
        let dst_ptr = (obj.offset(field.offset) as *mut T).as_mut().unwrap_unchecked();
        if let Some((data_len, len)) = usize::decode_var(&buf[O ..]) {
            if buf.len() < data_len + len + O {
                return empty(obj, buf, field, state);
            }
            if dst_ptr.extend_from_bytes(&buf[O + len .. O + len + data_len], &mut state.error) {
                // set_bit(obj, field.number);
                return (state.info.dispatch)(obj, &buf[O + len + data_len ..], field, state);
            }
        }
        not_found(obj, buf, field, state)
    }
}

pub struct Nested<T: TableDecodable, const O: usize>(PhantomData<T>);

impl<T: TableDecodable, const O: usize> Tabular for Nested<T, O> {
    unsafe fn decode(obj: *mut c_void, buf: &[u8], field: &FieldInfo, state: &mut DecodeState) -> Ret {
        let dst_ptr = (obj.offset(field.offset) as *mut T).as_mut().unwrap_unchecked();

        if let Some((data_len, len)) = u32::decode_var(&buf[O ..]) {
            let data_len = data_len as usize;

            if buf.len() < data_len + len + O {
                return empty(obj, buf, field, state);
            }

            let (nested, rest) = buf.split_at(O + len + data_len);

            let old_info = state.info;
            state.info = dst_ptr.info();
            // Entering a new stack frame here
            let _result = (state.info.dispatch)(dst_ptr as *mut T as *mut c_void, &nested[O + len ..], field, state);
            state.info = old_info;

            return (old_info.dispatch)(obj, rest, field, state);
        }
        empty(obj, buf, field, state)
    }
}

pub struct Repeated<T: TableDecodable + Default, const O: usize>(PhantomData<T>);

impl<T: TableDecodable + Default, const O: usize> Tabular for Repeated<T, O> {
    unsafe fn decode(obj: *mut c_void, buf: &[u8], field: &FieldInfo, state: &mut DecodeState) -> Ret {
        let dst_ptr: &mut Vec<T> = &mut *(obj.offset(field.offset) as *mut Vec<T>);

        if let Some((data_len, len)) = u32::decode_var(&buf[O ..]) {
            let data_len = data_len as usize;

            if buf.len() < data_len + len + O {
                return empty(obj, buf, field, state);
            }

            let (nested, rest) = buf.split_at(O + len + data_len);


            dst_ptr.push(T::default());

            let old_info = state.info;
            state.info = dst_ptr.last().unwrap_unchecked().info();
            let obj_ptr = dst_ptr.last_mut().unwrap_unchecked();
            // Entering a new stack frame here
            if !(state.info.dispatch)(obj_ptr as *mut T as *mut c_void, &nested[O + len ..], field, state) != 0 {
                panic!()
            }

            state.info = old_info;
            return (state.info.dispatch)(obj, rest, field, state);
        }
        empty(obj, buf, field, state)
    }
}

pub struct Mapped<K, V, const O: usize>(PhantomData<(K, V)>);

impl<K: Tabular, V: Tabular, const O: usize> Tabular for Mapped<K, V, O> {
    unsafe fn decode(_obj: *mut c_void, _buf: &[u8], _field: &FieldInfo, _state: &mut DecodeState) -> Ret {
        todo!()
    }
}

pub struct Enum<E: From<u32>, const O: usize>(PhantomData<E>);

impl<T: From<u32>, const O: usize> Tabular for Enum<T, O> {
    unsafe fn decode(obj: *mut c_void, buf: &[u8], field: &FieldInfo, state: &mut DecodeState) -> Ret {
        let dst_ptr: &mut T = (obj.offset(field.offset) as *mut T).as_mut().unwrap_unchecked();

        if let Some((variant, len)) = usize::decode_var(&buf[O ..]) {
            *dst_ptr = T::from(variant as u32);
            return (state.info.dispatch)(obj, &buf[O + len ..], field, state);
        }
        empty(obj, buf, field, state)
    }
}

pub struct EnumOpt<E: From<u32>, const O: usize>(PhantomData<E>);

impl<T: From<u32>, const O: usize> Tabular for EnumOpt<T, O> {
    unsafe fn decode(obj: *mut c_void, buf: &[u8], field: &FieldInfo, state: &mut DecodeState) -> Ret {
        let dst_ptr: &mut Option<T> = (obj.offset(field.offset) as *mut Option<T>).as_mut().unwrap_unchecked();

        if let Some((variant, len)) = usize::decode_var(&buf[O ..]) {
            *dst_ptr = Some(T::from(variant as u32));
            return (state.info.dispatch)(obj, &buf[O + len ..], field, state);
        }
        empty(obj, buf, field, state)
    }
}


// pub struct OneOf<T, const O: usize>(PhantomData<T>);
//
// impl<T: From<u32>, const O: usize> Format for OneOf<T, O> {
//     unsafe fn decode(obj: *mut c_void, state: &mut DecodeState) -> Ret {
//         todo!()
//     }
// }
