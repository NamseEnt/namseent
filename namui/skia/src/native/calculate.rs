use crate::*;
use anyhow::Result;
use namui_type::*;
use std::sync::Arc;

pub struct NativeCalculate;

impl NativeCalculate {
    pub fn group_glyph(font: &Font, paint: &Paint) -> Arc<dyn GroupGlyph> {
        todo!()
    }

    pub fn font_metrics(font: &Font) -> Option<FontMetrics> {
        todo!()
    }

    pub fn load_typeface(
        typeface_name: String,
        bytes: Vec<u8>,
    ) -> tokio::task::JoinHandle<Result<()>> {
        todo!()
    }

    pub fn path_contains_xy(path: &Path, paint: Option<&Paint>, xy: Xy<Px>) -> bool {
        todo!()
    }

    pub fn path_bounding_box(path: &Path, paint: Option<&Paint>) -> Option<Rect<Px>> {
        todo!()
    }
}
