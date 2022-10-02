use std::cmp::{max, min};
use std::io::IoSlice;

use byteorder::{ByteOrder, LittleEndian};
use integer_encoding::VarInt;

pub struct Node {
    dat: Box<[u8]>,
    pos: usize,
}

pub struct Buffer {
    curr: Box<[u8]>,
    // Length available in current buffer
    left: usize,
    // Cumulative len
    clen: usize,
    // Previous nodes in reverse order
    nodes: Vec<Node>,
}

impl Buffer {
    pub fn new() -> Self {
        Self::with_capacity(1000)
    }

    pub fn with_capacity(cap: usize) -> Self {
        Self {
            nodes: vec![],
            curr: vec![0; cap].into_boxed_slice(),
            clen: 0,
            left: cap,
        }
    }

    #[inline]
    pub fn mut_buf(&mut self) -> &mut [u8] {
        &mut self.curr[self.left ..]
    }

    #[inline]
    pub fn get_buf(&self) -> &[u8] {
        &self.curr[self.left ..]
    }

    pub fn buffers(&self) -> Vec<IoSlice> {
        let mut v = vec![IoSlice::new(self.get_buf())];
        v.extend(self.nodes.iter().rev().map(|n| IoSlice::new(&n.dat[n.pos ..])));
        v
    }

    #[inline(never)]
    #[cold]
    fn grow_hint(&mut self, hint: usize) {
        let size = max(self.curr.len(), hint);

        let old = std::mem::replace(&mut self.curr, vec![0; size].into_boxed_slice());
        self.clen += old.len() - self.left;
        self.nodes.push(Node {
            dat: old,
            pos: self.left,
        });

        self.left = self.curr.len();
    }

    #[inline(never)]
    #[cold]
    fn grow(&mut self) {
        let oldcap = self.curr.len();
        self.grow_hint((oldcap * 3) / 2);
    }

    pub fn len(&self) -> usize {
        self.clen + (self.curr.len() - self.left)
    }

    pub fn nest(&mut self) -> usize {
        self.len()
    }

    pub fn unnest(&mut self, v: usize) {
        self.vint(self.len() - v)
    }

    pub fn vint(&mut self, v: usize) {
        if self.left < 10 {
            self.grow();
        }

        let mut t = [0; 10];
        v.encode_var(&mut t);
        let len = v.required_space();

        self.left -= len;
        for i in 0 .. len {
            self.curr[self.left + i] = t[i];
        }
    }

    #[inline]
    pub fn fix32(&mut self, v: u32) {
        if self.left < 4 {
            self.grow()
        }

        self.left -= 4;
        LittleEndian::write_u32(self.mut_buf(), v)
    }

    #[inline]
    pub fn fix64(&mut self, v: u64) {
        if self.left < 8 {
            self.grow()
        }

        self.left -= 8;
        LittleEndian::write_u64(self.mut_buf(), v)
    }

    fn rawbytes(&mut self, mut bytes: &[u8]) {
        while !bytes.is_empty() {
            let copysize = min(bytes.len(), self.left);
            self.left -= copysize;
            let bufpos = bytes.len() - copysize;

            self.curr[self.left ..]
                .iter_mut()
                .zip(bytes[bufpos ..].iter())
                .for_each(|(d, s)| *d = *s);

            bytes = &bytes[.. bufpos];
            // If we did not copy everything, append a large buffer and continue
            if self.left == 0 {
                self.grow_hint(bytes.len())
            }
        }
    }
    pub fn bytes(&mut self, b: &[u8]) {
        // First copy the buffer
        self.rawbytes(b);
        // Then write the len, this is the big trick that allows us to not care about
        self.vint(b.len());
    }
}

#[test]
fn test_append() {
    let mut b = Buffer::new();
    b.rawbytes(&vec![0; 2048]);
    assert_eq!(b.len(), 2048);

    let mut b = Buffer::new();
    b.vint(0);
    assert_eq!(b.len(), 1);
}
