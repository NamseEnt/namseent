#[derive(
    Debug,
    Clone,
    Copy,
    serde::Serialize,
    serde::Deserialize,
    strum_macros::EnumIter,
    strum_macros::AsRefStr,
    strum_macros::EnumString,
)]
pub enum ImageInterpolation {
    AllLinear,
    SquashAndStretch { frame_per_second: f32 },
}

impl ImageInterpolation {
    pub fn iter() -> impl Iterator<Item = ImageInterpolation> {
        fn generic_iter<E>() -> impl Iterator<Item = E>
        where
            E: strum::IntoEnumIterator,
        {
            E::iter()
        }
        generic_iter::<ImageInterpolation>()
    }
}
