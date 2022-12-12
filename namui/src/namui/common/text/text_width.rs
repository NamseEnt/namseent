use super::*;
use crate::*;
use std::sync::Arc;

pub(crate) fn get_text_widths(
    text: &str,
    fonts: &Vec<Arc<Font>>,
    paint: Option<&Paint>,
) -> Vec<Px> {
    measure_glyphs(text, fonts, paint)
        .into_iter()
        .filter_map(|measure| match measure {
            GlyphMeasure::Failed => None,
            GlyphMeasure::Success { width, .. } => Some(width),
        })
        .collect()
}
