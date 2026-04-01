use crate::*;
use core_graphics_types::geometry::CGSize;
use foreign_types_shared::ForeignTypeRef;
use metal_rs::{CommandQueue, MetalDrawable, MetalLayer};
use namui_type::*;
use skia_safe::gpu::{self, DirectContext, SurfaceOrigin, backend_render_targets, mtl};
use std::borrow::ToOwned;

pub struct NativeSurface {
    context: DirectContext,
    metal_layer: MetalLayer,
    command_queue: CommandQueue,
    current_drawable: Option<MetalDrawable>,
    current_surface: Option<skia_safe::surface::Surface>,
}
unsafe impl Send for NativeSurface {}
unsafe impl Sync for NativeSurface {}

impl NativeSurface {
    pub(crate) fn new(
        context: DirectContext,
        metal_layer: MetalLayer,
        command_queue: CommandQueue,
    ) -> Self {
        Self {
            context,
            metal_layer,
            command_queue,
            current_drawable: None,
            current_surface: None,
        }
    }

    pub fn resize(&mut self, window_wh: Wh<IntPx>) {
        self.metal_layer.set_drawable_size(CGSize::new(
            window_wh.width.as_i32() as f64,
            window_wh.height.as_i32() as f64,
        ));
    }

    /// Should be called before use surface
    pub fn move_to_next_frame(&mut self) {
        self.current_surface = None;
        self.current_drawable = None;

        let drawable_ref = match self.metal_layer.next_drawable() {
            Some(drawable) => drawable,
            None => return,
        };

        let (drawable_width, drawable_height) = {
            let size = self.metal_layer.drawable_size();
            (size.width as i32, size.height as i32)
        };

        let surface = unsafe {
            let texture_info =
                mtl::TextureInfo::new(drawable_ref.texture().as_ptr() as mtl::Handle);

            let backend_render_target =
                backend_render_targets::make_mtl((drawable_width, drawable_height), &texture_info);

            gpu::surfaces::wrap_backend_render_target(
                &mut self.context,
                &backend_render_target,
                SurfaceOrigin::TopLeft,
                skia_safe::ColorType::BGRA8888,
                None,
                None,
            )
            .expect("Failed to wrap Metal backend render target")
        };

        self.current_drawable = Some(drawable_ref.to_owned());
        self.current_surface = Some(surface);
    }

    pub fn flush(&mut self) {
        self.context.flush_and_submit();

        if let Some(ref drawable) = self.current_drawable {
            let command_buffer = self.command_queue.new_command_buffer();
            command_buffer.present_drawable(drawable);
            command_buffer.commit();
        }

        self.current_surface = None;
        self.current_drawable = None;
    }

    pub fn canvas(&mut self) -> &dyn SkCanvas {
        self.current_surface
            .as_mut()
            .expect("No surface available. Call move_to_next_frame() first.")
            .canvas()
    }
}
