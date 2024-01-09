use crate::*;

#[type_derives(Copy)]
pub enum MaskFilter {
    Blur { blur: Blur },
}

#[type_derives(Copy)]
pub enum Blur {
    /// Fuzzy inside and outside
    Normal { sigma: f32 },
    /// Solid inside, fuzzy outside
    Solid { sigma: f32 },
    /// Nothing inside, fuzzy outside
    Outer { sigma: f32 },
    /// Fuzzy inside, nothing outside
    Inner { sigma: f32 },
}
