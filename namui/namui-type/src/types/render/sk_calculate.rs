
use tokio::task::JoinHandle;
use std::sync::Arc;
use crate::*;

pub trait SkCalculate {
    fn group_glyph(&self, font: &Font, paint: &Paint) -> Arc<dyn GroupGlyph>;
    fn font_metrics(&self, font: &Font) -> Option<FontMetrics>;
    fn load_typeface(&self, typeface_name: String, bytes: Vec<u8>) -> JoinHandle<anyhow::Result<()>>;
    fn path_contains_xy(&self, path: &Path, paint: Option<&Paint>, xy: Xy<Px>) -> bool;
    fn path_bounding_box(&self, path: &Path, paint: Option<&Paint>) -> Option<Rect<Px>>;
    fn load_image_from_encoded(&self, bytes: &[u8]) -> JoinHandle<Image>;
    fn load_image_from_raw(&self, image_info: ImageInfo, bytes: &[u8]) -> JoinHandle<Image>;
}
