use crate::*;
use anyhow::Result;
use namui_type::*;
use std::sync::Arc;

pub(crate) struct CkCalculate;

impl CkCalculate {
    pub(crate) fn new() -> Self {
        CkCalculate
    }
}

impl SkCalculate for CkCalculate {
    fn group_glyph(&self, font: &Font, paint: &Paint) -> Arc<dyn GroupGlyph> {
        CkGroupGlyph::get(font, paint)
    }

    fn font_metrics(&self, font: &Font) -> Option<FontMetrics> {
        CkFont::get(font).map(|x| x.metrics)
    }

    fn load_typeface(&self, typeface_name: &str, bytes: &[u8]) -> Result<()> {
        CkTypeface::load(typeface_name, bytes)
    }

    fn path_contains_xy(&self, path: &Path, paint: Option<&Paint>, xy: Xy<Px>) -> bool {
        CkPath::get(path).contains(paint, xy)
    }

    fn path_bounding_box(&self, path: &Path, paint: Option<&Paint>) -> Option<Rect<Px>> {
        CkPath::get(path).bounding_box(paint)
    }
}
