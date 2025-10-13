use crate::*;
use anyhow::Result;
use std::sync::Arc;

pub struct NativeSkia {
    surface: NativeSurface,
    calculate: NativeCalculate,
}
unsafe impl Send for NativeSkia {}
unsafe impl Sync for NativeSkia {}

unsafe extern "C" {
    fn emscripten_glGetString(name: u32) -> *const u8;
    fn emscripten_glGetIntegerv(pname: u32, data: *mut i32);
}

impl NativeSkia {
    pub(crate) fn new(window_wh: Wh<IntPx>) -> Result<NativeSkia> {
        let interface = skia_safe::gpu::gl::Interface::new_load_with(|addr| match addr {
            "glGetString" => emscripten_glGetString as _,
            _ => todo!("unknown function on gl interface: {}", addr),
        })
        .expect("failed to load gl interface");

        let context = skia_safe::gpu::direct_contexts::make_gl(interface, None)
            .expect("failed to create gl direct context");

        let framebuffer_info = {
            let mut fboid: i32 = 0;
            unsafe {
                emscripten_glGetIntegerv(
                    0x8ca6, // gl::FRAMEBUFFER_BINDING
                    &mut fboid,
                )
            };

            skia_safe::gpu::gl::FramebufferInfo {
                fboid: fboid.try_into().unwrap(),
                format: skia_safe::gpu::gl::Format::RGBA8.into(),
                protected: skia_safe::gpu::Protected::No,
            }
        };

        Ok(Self {
            surface: NativeSurface::new(context, window_wh, framebuffer_info)?,
            calculate: NativeCalculate::new(),
        })
    }
}

impl NativeSkia {
    fn move_to_next_frame(&mut self) {
        // Nothing?
    }
    fn surface(&mut self) -> &mut dyn SkSurface {
        &mut self.surface
    }
    fn on_resize(&mut self, wh: Wh<IntPx>) {
        self.surface.resize(wh);
    }
    // fn load_image_from_bytes(&self, bytes: &[u8], image_info: ImageInfo, encoded: bool) -> Image {
    //     let image = if encoded {
    //         skia_safe::Image::from_encoded(skia_safe::Data::new_copy(bytes))
    //     } else {
    //         let row_bytes = image_info.width.as_f32() as usize * image_info.color_type.word();
    //         skia_safe::images::raster_from_data(
    //             &image_info.into(),
    //             skia_safe::Data::new_copy(bytes),
    //             row_bytes,
    //         )
    //     }
    //     .unwrap();

    //     Image::new(image_info, image)
    // }
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
