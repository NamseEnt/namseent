use crate::native::calculate::NativeCalculate;
use crate::*;
use anyhow::Result;
use namui_type::*;
use std::sync::Arc;

pub struct NativeSkia {
    surface: NativeSurface,
    calculate: NativeCalculate,
}

impl NativeSkia {}

impl NativeSkia {
    fn move_to_next_frame(&mut self) {
        self.surface.move_to_next_frame();
    }
    fn surface(&mut self) -> &mut dyn SkSurface {
        &mut self.surface
    }
    fn on_resize(&mut self, wh: Wh<IntPx>) {
        self.surface.resize(wh);
    }
}

impl SkCalculate for NativeSkia {
    fn group_glyph(&self, font: &Font, paint: &Paint) -> Arc<dyn GroupGlyph> {
        self.calculate.group_glyph(font, paint)
    }

    fn font_metrics(&self, font: &Font) -> Option<FontMetrics> {
        self.calculate.font_metrics(font)
    }

    fn load_typeface(
        &self,
        typeface_name: String,
        bytes: Vec<u8>,
    ) -> tokio::task::JoinHandle<Result<()>> {
        self.calculate.load_typeface(typeface_name, bytes)
    }

    fn path_contains_xy(&self, path: &Path, paint: Option<&Paint>, xy: Xy<Px>) -> bool {
        self.calculate.path_contains_xy(path, paint, xy)
    }

    fn path_bounding_box(&self, path: &Path, paint: Option<&Paint>) -> Option<Rect<Px>> {
        self.calculate.path_bounding_box(path, paint)
    }

    fn load_image_from_encoded(&self, bytes: &[u8]) -> tokio::task::JoinHandle<Image> {
        self.calculate.load_image_from_encoded(bytes)
    }

    fn load_image_from_raw(
        &self,
        image_info: ImageInfo,
        bytes: &[u8],
    ) -> tokio::task::JoinHandle<Image> {
        self.calculate.load_image_from_raw(image_info, bytes)
    }
}
