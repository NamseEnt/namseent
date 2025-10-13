use crate::*;
use anyhow::Result;
use std::sync::Arc;

pub struct NativeSkia {
    surface: NativeSurface,
}
unsafe impl Send for NativeSkia {}
unsafe impl Sync for NativeSkia {}

unsafe extern "C" {
    fn emscripten_glGetString(name: u32) -> *const u8;
    fn emscripten_glGetIntegerv(pname: u32, data: *mut i32);
}

impl NativeSkia {
    pub fn move_to_next_frame(&mut self) {
        // Nothing?
    }
    pub fn surface(&mut self) -> &mut NativeSurface {
        &mut self.surface
    }
    pub fn on_resize(&mut self, wh: Wh<IntPx>) {
        self.surface.resize(wh);
    }

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
        })
    }
}
