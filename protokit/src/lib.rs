extern crate core;

pub use anyhow::Result;
#[cfg(feature = "grpc")]
pub use grpc;
pub use textformat::{reflect, Indent};
pub use {binformat, textformat};

pub mod types;

pub trait Field: Default {
    /// When is_present is false we do not serialize this field.
    fn is_present(&self) -> bool;

    fn output(&self, s: &mut OutputStream);
    fn input(&mut self, s: &mut InputStream) -> Result<()>;
}

impl<T> Field for Option<T> where T: Field {
    fn is_present(&self) -> bool {
        self.is_some()
    }

    fn output(&self, s: &mut OutputStream) {
        if let Some(it) = self {
            it.output(s);
        }
    }

    fn input(&mut self, s: &mut InputStream) -> Result<()> {
        self.get_or_insert_with(Default::default)
            .input(s)
    }
}

pub struct InputStream<'a> {}

impl<'a> InputStream<'a> {
    fn vint(&mut self) -> Result<u64> { todo!() }

    fn sint32(&mut self) -> Result<i32> { todo!() }
    fn sint64(&mut self) -> Result<i64> { todo!() }

    fn fixed32(&mut self) -> Result<u32> { todo!() }
    fn fixed64(&mut self) -> Result<u64> { todo!() }

    fn sfixed32(&mut self) -> Result<i32> { todo!() }
    fn sfixed64(&mut self) -> Result<i64> { todo!() }

    fn float(&mut self) -> Result<f32> { todo!() }
    fn double(&mut self) -> Result<f64> { todo!() }

    fn string(&mut self, s: &mut String) -> Result<()> { todo!() }
    fn bytes(&mut self, s: &mut Vec<u8>) -> Result<()> { todo!() }

    fn bool(&mut self) -> Result<bool> { todo!() }
    fn r#enum(&mut self) -> Result<u32> { todo!() }
}

pub struct OutputStream<'a> {}

impl<'a> OutputStream<'a> {
    fn vint(&mut self, v: u64) {}

    fn sint32(&mut self, v: i32) {}
    fn sint64(&mut self, v: i64) {}

    fn fixed32(&mut self, f: u32) {}
    fn fixed64(&mut self, f: f64) {}

    fn sfixed32(&mut self, f: i32) {}
    fn sfixed64(&mut self, f: i64) {}

    fn float(&mut self, f: f32) {}
    fn double(&mut self, f: f32) {}

    fn string(&mut self, f: &str) {}
    fn bytes(&mut self, f: &[u8]) {}

    fn bool(&mut self, v: bool) {}
    fn r#enum(&mut self, v: impl Into<u32>) {}
}