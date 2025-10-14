use crate::*;
use anyhow::Result;
use namui_type::*;

pub struct NativeSurface {
    surface: skia_safe::surface::Surface,
    framebuffer_info: skia_safe::gpu::gl::FramebufferInfo,
    context: skia_safe::gpu::DirectContext,
}
unsafe impl Send for NativeSurface {}
unsafe impl Sync for NativeSurface {}

impl NativeSurface {
    pub fn flush(&mut self) {
        self.context
            .flush_and_submit_surface(&mut self.surface, Some(skia_safe::gpu::SyncCpu::Yes));
    }

    pub fn canvas(&mut self) -> &dyn SkCanvas {
        self.surface.canvas()
    }

    pub fn resize(&mut self, window_wh: Wh<IntPx>) {
        let surface = Self::make_gl_surface(&mut self.context, self.framebuffer_info, window_wh);
        self.surface = surface;
    }

    pub(crate) fn new(
        mut context: skia_safe::gpu::DirectContext,
        window_wh: Wh<IntPx>,
        framebuffer_info: skia_safe::gpu::gl::FramebufferInfo,
    ) -> Result<Self> {
        let surface = Self::make_gl_surface(&mut context, framebuffer_info, window_wh);

        Ok(Self {
            context,
            framebuffer_info,
            surface,
        })
    }

    fn make_gl_surface(
        context: &mut skia_safe::gpu::DirectContext,
        framebuffer_info: skia_safe::gpu::gl::FramebufferInfo,
        window_wh: Wh<IntPx>,
    ) -> skia_safe::surface::Surface {
        let backend_render_target = skia_safe::gpu::backend_render_targets::make_gl(
            (window_wh.width.as_i32(), window_wh.height.as_i32()),
            1,
            0,
            framebuffer_info,
        );

        skia_safe::gpu::surfaces::wrap_backend_render_target(
            context,
            &backend_render_target,
            skia_safe::gpu::SurfaceOrigin::BottomLeft,
            skia_safe::ColorType::RGBA8888,
            None,
            None,
        )
        .expect("failed to wrap backend render target")
    }
}
