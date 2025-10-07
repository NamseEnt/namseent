use crate::*;

#[derive(Debug, PartialEq, Clone, Hash, Eq)]
pub enum Blender {
    BlendMode(BlendMode),
    Sksl(String),
    Arithmetic {
        k1: OrderedFloat,
        k2: OrderedFloat,
        k3: OrderedFloat,
        k4: OrderedFloat,
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

impl From<BlendMode> for Blender {
    fn from(value: BlendMode) -> Self {
        Blender::BlendMode(value)
    }
}
