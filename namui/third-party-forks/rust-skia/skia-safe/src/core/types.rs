use crate::prelude::*;
use skia_bindings::SkFourByteTag;
use std::ops::Deref;

//
// FourByteTag
//

#[derive(Copy, Clone, PartialEq, Eq, Hash, Default, Debug)]
#[repr(transparent)]
pub struct FourByteTag(SkFourByteTag);

native_transmutable!(SkFourByteTag, FourByteTag, four_byte_tag_layout);

impl Deref for FourByteTag {
    type Target = u32;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl From<(char, char, char, char)> for FourByteTag {
    fn from((a, b, c, d): (char, char, char, char)) -> Self {
        Self::from_chars(a, b, c, d)
    }
}

impl From<u32> for FourByteTag {
    fn from(v: u32) -> Self {
        Self::new(v)
    }
}

impl FourByteTag {
    pub const fn from_chars(a: char, b: char, c: char, d: char) -> Self {
        Self(
            (a as u8 as u32) << 24
                | (b as u8 as u32) << 16
                | (c as u8 as u32) << 8
                | d as u8 as u32,
        )
    }

    pub const fn new(v: u32) -> Self {
        Self(v)
    }

    pub fn a(self) -> u8 {
        (self.into_native() >> 24) as u8
    }

    pub fn b(self) -> u8 {
        (self.into_native() >> 16) as u8
    }

    pub fn c(self) -> u8 {
        (self.into_native() >> 8) as u8
    }

    pub fn d(self) -> u8 {
        self.into_native() as u8
    }
}

// TODO: wrap for type safety?
pub type GlyphId = skia_bindings::SkGlyphID;

// TODO: wrap for type safety?
pub type Unichar = skia_bindings::SkUnichar;

// note std::time::Duration is used in place of MSec in public
// facing functions.
// pub(crate) type MSec = skia_bindings::SkMSec;
// pub(crate) const MSEC_MAX: u32 = std::i32::MAX as u32;

#[cfg(feature = "gpu")]
#[deprecated(since = "0.60.0", note = "Use gpu::Budgeted")]
pub type Budgeted = crate::gpu::Budgeted;
