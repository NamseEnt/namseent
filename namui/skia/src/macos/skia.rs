use crate::*;
use anyhow::Result;
use core_graphics_types::geometry::CGSize;
use foreign_types_shared::ForeignType;
use metal_rs::{Device, MTLPixelFormat, MetalLayer};
use namui_type::*;
use objc2::msg_send;
use objc2::runtime::AnyObject;
use skia_safe::gpu::{self, mtl};
use winit::raw_window_handle::HasWindowHandle;
use winit::window::Window;

pub struct NativeSkia {
    surface: NativeSurface,
}
unsafe impl Send for NativeSkia {}
unsafe impl Sync for NativeSkia {}

impl NativeSkia {
    pub(crate) fn new(window: &Window, window_wh: Wh<IntPx>) -> Result<NativeSkia> {
        let window_handle = window
            .window_handle()
            .expect("Failed to retrieve a window handle");
        let raw_window_handle = window_handle.as_raw();

        let device = Device::system_default().expect("no Metal device found");

        let metal_layer = {
            let layer = MetalLayer::new();
            layer.set_device(&device);
            layer.set_pixel_format(MTLPixelFormat::BGRA8Unorm);
            layer.set_presents_with_transaction(false);
            layer.set_framebuffer_only(false);

            unsafe {
                let view_ptr = match raw_window_handle {
                    winit::raw_window_handle::RawWindowHandle::AppKit(appkit) => {
                        appkit.ns_view.as_ptr()
                    }
                    _ => panic!("Expected AppKit window handle"),
                };
                let view: &AnyObject = &*(view_ptr as *const AnyObject);
                let _: () = msg_send![view, setWantsLayer: true];
                let layer_obj: &AnyObject = &*(layer.as_ref() as *const _ as *const AnyObject);
                let _: () = msg_send![view, setLayer: layer_obj];
            }
            layer.set_drawable_size(CGSize::new(
                window_wh.width.as_i32() as f64,
                window_wh.height.as_i32() as f64,
            ));
            layer
        };

        let command_queue = device.new_command_queue();

        let backend = unsafe {
            mtl::BackendContext::new(
                device.as_ptr() as mtl::Handle,
                command_queue.as_ptr() as mtl::Handle,
            )
        };

        let context = gpu::direct_contexts::make_metal(&backend, None)
            .ok_or_else(|| anyhow::anyhow!("Failed to create Metal Skia context"))?;

        Ok(Self {
            surface: NativeSurface::new(context, metal_layer, command_queue),
        })
    }

    pub fn move_to_next_frame(&mut self) {
        self.surface.move_to_next_frame();
    }
    pub fn surface(&mut self) -> &mut NativeSurface {
        &mut self.surface
    }
    pub fn on_resize(&mut self, wh: Wh<IntPx>) {
        self.surface.resize(wh);
    }
}
