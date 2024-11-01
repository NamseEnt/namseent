mod leb128;
#[cfg(feature = "vec")]
mod std_vec;
// mod map;
mod list;
mod primitive;
mod str;
// mod set;

use anyhow::*;
use bytes::{Buf, Bytes, BytesMut};
use leb128::*;
pub use list::*;
use std::{
    cell::OnceCell,
    ops::{Deref, DerefMut},
};
pub use str::*;
// pub use map::*;
// pub use set::*;

pub trait Nsd: Clone {
    fn byte_len(&self) -> usize;
    fn write_on_bytes(&self, dest: &mut [u8]) -> Result<()>;
    fn from_bytes(bytes: &mut Bytes) -> Result<Self>
    where
        Self: Sized;
    fn to_bytes(&self) -> Bytes {
        let byte_len = self.byte_len();
        let mut bytes = BytesMut::with_capacity(byte_len);
        bytes.resize(byte_len, 0);
        self.write_on_bytes(&mut bytes).unwrap();
        bytes.freeze()
    }
}

#[derive(Debug)]
pub enum FromBytesError {
    NotEnoughBytes,
}

impl std::fmt::Display for FromBytesError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Not enough bytes")
    }
}

impl std::error::Error for FromBytesError {}

struct DestWriter<'a> {
    dest: &'a mut [u8],
    index: usize,
}
impl<'a> DestWriter<'a> {
    fn new(dest: &'a mut [u8]) -> Self {
        Self { dest, index: 0 }
    }

    fn write(&mut self, nsd: &impl Nsd) -> Result<()> {
        let byte_len = nsd.byte_len();
        if self.index + byte_len > self.dest.len() {
            bail!(FromBytesError::NotEnoughBytes);
        }
        nsd.write_on_bytes(&mut self.dest[self.index..self.index + byte_len])?;
        self.index += byte_len;
        Result::Ok(())
    }
    fn write_bytes(&mut self, bytes: &[u8]) -> Result<()> {
        if self.index + bytes.len() > self.dest.len() {
            bail!(FromBytesError::NotEnoughBytes);
        }
        self.dest[self.index..self.index + bytes.len()].copy_from_slice(bytes);
        self.index += bytes.len();
        Result::Ok(())
    }
    fn written_len(&self) -> usize {
        self.index
    }
}

fn test() {
    {
        let a = MyType {
            a: A {
                b: B {
                    value_1: 1,
                    value_2: 2,
                },
            },
            b: B {
                value_1: 3,
                value_2: 4,
            },
            c: C {
                text: "Hello".to_string(),
            },
        };
        let a = a.a();
        let _b = a.b();
    }

    {
        let mut a = MyType {
            a: A {
                b: B {
                    value_1: 1,
                    value_2: 2,
                },
            },
            b: B {
                value_1: 3,
                value_2: 4,
            },
            c: C {
                text: "Hello".to_string(),
            },
        };
        a.mutate().b().value_1 = 5;
    }

    {
        let a = LazyMyType::from_bytes(Bytes::new());
        let a = a.a();
        let b = a.b();
    }

    {
        let mut a = LazyMyType::from_bytes(Bytes::new());
        a.mutate().b().value_1 = 5;
    }

    {
        let serialized = Serialized::<MyType> {
            bytes: Bytes::new(),
            _phantom: std::marker::PhantomData,
        };
        let a = serialized.access();
        let a = a.a();
        let b = a.b();
    }

    {
        let mut a = LazyMyType::from_bytes(Bytes::new());
        a.mutate().b().value_1 = 5;
    }
}

struct MyType {
    a: A,
    b: B,
    c: C,
}
impl MyType {}
impl Serialize for MyType {
    fn byte_len(&self) -> usize {
        todo!()
    }

    fn write_on_bytes(&self, dest: &mut [u8]) {
        todo!()
    }
}
impl RefMyType for MyType {
    fn a(&self) -> impl RefA {
        &self.a
    }

    fn b(&self) -> impl RefB {
        &self.b
    }

    fn c(&self) -> impl RefC {
        &self.c
    }
}
impl MutMyType for MyType {
    fn a(&mut self) -> impl MutA {
        &mut self.a
    }

    fn b(&mut self) -> impl MutB {
        &mut self.b
    }

    fn c(&mut self) -> impl MutC {
        &mut self.c
    }
}

/**
 * Memory layout
 *
 * [field A bytes range]
 * [field B bytes range]
 * [field C bytes range]
 * ...
 * [Field A bytes]
 * [Field B bytes]
 * [Field C bytes]
 * ...
 */

struct LazyMyType {
    bytes: Bytes,
    a_end_offset: usize,
    b_end_offset: usize,
    c_end_offset: usize,
    a_start_offset: usize,
    a_original: OnceCell<LazyA>,
    b_original: OnceCell<LazyB>,
    c_original: OnceCell<LazyC>,
    a_mutated: bool,
    b_mutated: bool,
    c_mutated: bool,
}
impl Serialize for LazyMyType {
    fn byte_len(&self) -> usize {
        if !self.a_mutated && !self.b_mutated && !self.c_mutated {
            return self.bytes.len();
        }

        todo!()
    }

    fn write_on_bytes(&self, dest: &mut [u8]) {
        if !self.a_mutated && !self.b_mutated && !self.c_mutated {
            dest.copy_from_slice(&self.bytes);
            return;
        }

        todo!()
    }
}
impl LazyMyType {
    pub(crate) fn from_bytes(bytes: Bytes) -> Self {
        let bytes_len = bytes.len();
        let mut header = bytes.clone();
        Self {
            bytes,
            a_end_offset: leb128::read(&mut header).unwrap(),
            b_end_offset: leb128::read(&mut header).unwrap(),
            c_end_offset: leb128::read(&mut header).unwrap(),
            a_start_offset: bytes_len - header.len(),
            a_original: OnceCell::new(),
            b_original: OnceCell::new(),
            c_original: OnceCell::new(),
            a_mutated: false,
            b_mutated: false,
            c_mutated: false,
        }
    }
    pub fn deserialize(self) -> MyType {
        todo!()
    }
}
trait RefMyType {
    fn a(&self) -> impl RefA;
    fn b(&self) -> impl RefB;
    fn c(&self) -> impl RefC;
}
impl RefMyType for LazyMyType {
    fn a(&self) -> impl RefA {
        self.a_original.get_or_init(|| {
            let a_byte_range = self.a_start_offset..self.a_end_offset;
            let a_bytes = self.bytes.slice(a_byte_range);
            LazyA::from_bytes(a_bytes)
        })
    }

    fn b(&self) -> impl RefB {
        self.b_original.get_or_init(|| {
            let b_byte_range = (self.a_end_offset + 1)..self.b_end_offset;
            let b_bytes = self.bytes.slice(b_byte_range);
            LazyB::from_bytes(b_bytes)
        })
    }

    fn c(&self) -> impl RefC {
        self.c_original.get_or_init(|| {
            let c_byte_range = (self.b_end_offset + 1)..self.c_end_offset;
            let c_bytes = self.bytes.slice(c_byte_range);
            LazyC::from_bytes(c_bytes)
        })
    }
}

trait MutMyType {
    fn a(&mut self) -> impl MutA;
    fn b(&mut self) -> impl MutB;
    fn c(&mut self) -> impl MutC;
    fn mutate(&mut self) -> &mut Self {
        self
    }
}
impl MutMyType for LazyMyType {
    fn a(&mut self) -> impl MutA {
        self.a_original.get_or_init(|| {
            let a_byte_range = self.a_start_offset..self.a_end_offset;
            let a_bytes = self.bytes.slice(a_byte_range);
            LazyA::from_bytes(a_bytes)
        });
        self.a_mutated = true;
        self.a_original.get_mut().unwrap()
    }

    fn b(&mut self) -> impl MutB {
        self.b_original.get_or_init(|| {
            let b_byte_range = (self.a_end_offset + 1)..self.b_end_offset;
            let b_bytes = self.bytes.slice(b_byte_range);
            LazyB::from_bytes(b_bytes)
        });
        self.b_mutated = true;
        self.b_original.get_mut().unwrap()
    }

    fn c(&mut self) -> impl MutC {
        self.c_original.get_or_init(|| {
            let c_byte_range = (self.b_end_offset + 1)..self.c_end_offset;
            let c_bytes = self.bytes.slice(c_byte_range);
            LazyC::from_bytes(c_bytes)
        });
        self.c_mutated = true;
        self.c_original.get_mut().unwrap()
    }
}

struct Serialized<T> {
    bytes: Bytes,
    _phantom: std::marker::PhantomData<T>,
}
impl<T> Serialize for Serialized<T> {
    fn byte_len(&self) -> usize {
        self.bytes.len()
    }

    fn write_on_bytes(&self, dest: &mut [u8]) {
        dest.copy_from_slice(&self.bytes);
    }
}
impl Serialized<MyType> {
    pub fn access(self) -> LazyMyType {
        LazyMyType::from_bytes(self.bytes)
    }
    pub fn deserialize(self) -> MyType {
        todo!()
    }
}

trait Serialize {
    fn byte_len(&self) -> usize;
    fn write_on_bytes(&self, dest: &mut [u8]);
}

struct A {
    b: B,
}
struct LazyA {
    bytes: Bytes,
    b_end_offset: usize,
    b_start_offset: usize,
    b_original: OnceCell<LazyB>,
}
impl LazyA {
    pub(crate) fn from_bytes(bytes: Bytes) -> Self {
        let bytes_len = bytes.len();
        let mut header = bytes.clone();
        Self {
            bytes,
            b_end_offset: leb128::read(&mut header).unwrap(),
            b_start_offset: bytes_len - header.len(),
            b_original: OnceCell::new(),
        }
    }
}

trait RefA {
    fn b(&self) -> impl RefB;
}

impl RefA for &LazyA {
    fn b(&self) -> impl RefB {
        self.b_original.get_or_init(|| {
            let b_byte_range = (self.b_start_offset + 1)..self.b_end_offset;
            let b_bytes = self.bytes.slice(b_byte_range);
            LazyB::from_bytes(b_bytes)
        })
    }
}
impl RefA for &A {
    fn b(&self) -> impl RefB {
        &self.b
    }
}

trait MutA {
    fn b(&mut self) -> impl MutB;
}

impl MutA for &mut LazyA {
    fn b(&mut self) -> impl MutB {
        self.b_original.get_or_init(|| {
            let b_byte_range = (self.b_start_offset + 1)..self.b_end_offset;
            let b_bytes = self.bytes.slice(b_byte_range);
            LazyB::from_bytes(b_bytes)
        });
        self.b_original.get_mut().unwrap()
    }
}
impl MutA for &mut A {
    fn b(&mut self) -> impl MutB {
        &mut self.b
    }
}

struct B {
    value_1: i32,
    value_2: i32,
}
type LazyB = B;
impl LazyB {
    fn from_bytes(mut bytes: Bytes) -> Self {
        Self {
            value_1: bytes.get_i32_le(),
            value_2: bytes.get_i32_le(),
        }
    }
}

trait RefB: Deref<Target = B> {}
impl RefB for &B {}

trait MutB: DerefMut<Target = B> {}
impl MutB for &mut B {}

struct C {
    text: String,
}
struct LazyC {}
impl LazyC {
    fn from_bytes(bytes: Bytes) -> Self {
        Self {}
    }
}

trait RefC {
    fn text(&self) -> &String;
}

impl RefC for &LazyC {
    fn text(&self) -> &String {
        todo!()
    }
}

impl RefC for &C {
    fn text(&self) -> &String {
        &self.text
    }
}

trait MutC {
    fn text(&mut self) -> &mut String;
}

impl MutC for &mut LazyC {
    fn text(&mut self) -> &mut String {
        todo!()
    }
}

impl MutC for &mut C {
    fn text(&mut self) -> &mut String {
        &mut self.text
    }
}
