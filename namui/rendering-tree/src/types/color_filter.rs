use crate::*;

#[derive(Debug, PartialEq, Clone, Copy, Hash, Eq, State)]
pub enum ColorFilter {
    Blend {
        color: Color,
        blend_mode: BlendMode,
    },
    ScaleMatrix {
        r: OrderedFloat,
        g: OrderedFloat,
        b: OrderedFloat,
        a: OrderedFloat,
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
impl From<&ColorFilter> for NativeColorFilter {
    fn from(value: &ColorFilter) -> Self {
        match *value {
            ColorFilter::Blend { color, blend_mode } => NativeColorFilter {
                skia_color_filter: skia_safe::color_filters::blend(color, blend_mode.into())
                    .unwrap(),
            },
            ColorFilter::ScaleMatrix { r, g, b, a } => {
                let mut color_matrix = skia_safe::ColorMatrix::default();
                color_matrix.set_scale(*r, *b, *g, Some(*a));
                let skia_color_filter = skia_safe::color_filters::matrix(&color_matrix, None);
                NativeColorFilter { skia_color_filter }
            }
        }
    }
}
