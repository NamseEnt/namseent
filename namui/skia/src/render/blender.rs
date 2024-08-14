use crate::*;

#[derive(Debug, PartialEq, Clone, Hash, Eq)]
pub enum Blender {
    BlendMode(BlendMode),
    Sksl(String),
    Arithmetic {
        k1: OrderedFloat<f32>,
        k2: OrderedFloat<f32>,
        k3: OrderedFloat<f32>,
        k4: OrderedFloat<f32>,
    },
}

impl Blender {
    /// Create a blender that implements the following:
    /// `k1 * src * dst + k2 * src + k3 * dst + k4`
    pub fn arithmetic(k1: f32, k2: f32, k3: f32, k4: f32) -> Self {
        Blender::Arithmetic {
            k1: k1.into(),
            k2: k2.into(),
            k3: k3.into(),
            k4: k4.into(),
        }
    }
}

impl From<Blender> for skia_safe::Blender {
    fn from(blender: Blender) -> Self {
        match blender {
            Blender::BlendMode(blend_mode) => skia_safe::BlendMode::from(blend_mode).into(),
            Blender::Sksl(sksl) => skia_safe::RuntimeEffect::make_for_blender(sksl, None)
                .unwrap()
                .make_blender(skia_safe::Data::new_empty(), None)
                .unwrap(),
            Blender::Arithmetic { k1, k2, k3, k4 } => {
                skia_safe::Blender::arithmetic(k1.into(), k2.into(), k3.into(), k4.into(), false)
                    .unwrap()
            }
        }
    }
}

impl From<BlendMode> for Blender {
    fn from(value: BlendMode) -> Self {
        Blender::BlendMode(value)
    }
}
