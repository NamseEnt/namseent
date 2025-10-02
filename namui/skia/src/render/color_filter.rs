use crate::*;

#[derive(Debug, PartialEq, Clone, Copy, Hash, Eq)]
pub enum ColorFilter {
    Blend {
        color: Color,
        blend_mode: BlendMode,
    },
    ScaleMatrix {
        r: OrderedFloat<f32>,
        g: OrderedFloat<f32>,
        b: OrderedFloat<f32>,
        a: OrderedFloat<f32>,
    },
}

impl ColorFilter {
    pub fn scale_matrix(r: f32, g: f32, b: f32, a: f32) -> Self {
        ColorFilter::ScaleMatrix {
            r: r.into(),
            g: g.into(),
            b: b.into(),
            a: a.into(),
        }
    }
}
