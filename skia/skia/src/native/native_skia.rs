use super::*;
use crate::*;
use anyhow::Result;
use std::sync::Arc;
use windows::Win32::{
    Foundation::HWND,
    Graphics::{
        Direct3D::D3D_FEATURE_LEVEL_11_0,
        Direct3D12::{
            D3D12CreateDevice, ID3D12CommandQueue, ID3D12Device, D3D12_COMMAND_LIST_TYPE_DIRECT,
            D3D12_COMMAND_QUEUE_DESC, D3D12_COMMAND_QUEUE_FLAG_NONE,
        },
        Dxgi::{
            CreateDXGIFactory1, IDXGIAdapter1, IDXGIFactory4, DXGI_ADAPTER_FLAG,
            DXGI_ADAPTER_FLAG_NONE, DXGI_ADAPTER_FLAG_SOFTWARE,
        },
    },
};

pub(crate) struct NativeSkia {
    backend_context: skia_safe::gpu::d3d::BackendContext,
    context: skia_safe::gpu::DirectContext,
    surface: NativeSurface,
    hwnd: HWND,
}
unsafe impl Send for NativeSkia {}
unsafe impl Sync for NativeSkia {}

impl NativeSkia {
    pub(crate) fn new(window_id: usize, window_wh: Wh<IntPx>) -> Result<NativeSkia> {
        let hwnd = HWND(window_id as isize);

        let factory = unsafe { CreateDXGIFactory1::<IDXGIFactory4>() }?;
        let adapter = get_hardware_adapter(&factory)?;

        let mut device: Option<ID3D12Device> = None;
        unsafe { D3D12CreateDevice(&adapter, D3D_FEATURE_LEVEL_11_0, &mut device) }?;
        let device = device.unwrap();

        let command_queue = unsafe {
            device.CreateCommandQueue::<ID3D12CommandQueue>(&D3D12_COMMAND_QUEUE_DESC {
                Flags: D3D12_COMMAND_QUEUE_FLAG_NONE,
                Type: D3D12_COMMAND_LIST_TYPE_DIRECT,
                ..Default::default()
            })
        }?;

        let backend_context = skia_safe::gpu::d3d::BackendContext {
            adapter,
            device,
            queue: command_queue,
            memory_allocator: None,
            protected_context: skia_safe::gpu::Protected::No,
        };

        let context =
            unsafe { skia_safe::gpu::DirectContext::new_d3d(&backend_context, None).unwrap() };

        Ok(Self {
            surface: NativeSurface::new(
                context.clone(),
                window_wh,
                &backend_context.device,
                &backend_context.queue,
                hwnd,
            )?,
            backend_context,
            context,
            hwnd,
        })
    }
}

impl SkSkia for NativeSkia {
    fn surface(&mut self) -> &mut impl SkSurface {
        &mut self.surface
    }
    fn on_resize(&mut self, wh: Wh<IntPx>) {
        self.surface = NativeSurface::new(
            self.context.clone(),
            wh,
            &self.backend_context.device,
            &self.backend_context.queue,
            self.hwnd,
        )
        .unwrap();
    }

    fn group_glyph(&self, font: &Font, paint: &Paint) -> Arc<dyn GroupGlyph> {
        NativeGroupGlyph::get(font, paint)
    }

    fn font_metrics(&self, font: &Font) -> Option<FontMetrics> {
        NativeFont::get(font).map(|x| x.metrics)
    }

    fn load_typeface(&self, typeface_name: &str, bytes: &[u8]) {
        NativeTypeface::load(typeface_name, bytes)
    }

    fn image(&self, image_source: &ImageSource) -> Option<Image> {
        NativeImage::get(image_source).map(|x| x.image())
    }

    fn path_contains_xy(&self, path: &Path, paint: Option<&Paint>, xy: Xy<Px>) -> bool {
        NativePath::get(path).contains(paint, xy)
    }

    fn path_bounding_box(&self, path: &Path, paint: Option<&Paint>) -> Option<Rect<Px>> {
        NativePath::get(path).bounding_box(paint)
    }

    fn load_image(&self, image_source: ImageSource, encoded_image: &[u8]) {
        NativeImage::load(image_source, encoded_image);
    }
}

fn get_hardware_adapter(factory: &IDXGIFactory4) -> Result<IDXGIAdapter1> {
    for i in 0.. {
        let adapter = unsafe { factory.EnumAdapters1(i)? };

        let mut desc = Default::default();
        unsafe { adapter.GetDesc1(&mut desc)? };

        if (DXGI_ADAPTER_FLAG(desc.Flags as i32) & DXGI_ADAPTER_FLAG_SOFTWARE)
            != DXGI_ADAPTER_FLAG_NONE
        {
            // Don't select the Basic Render Driver adapter. If you want a
            // software adapter, pass in "/warp" on the command line.
            continue;
        }

        // Check to see whether the adapter supports Direct3D 12, but don't
        // create the actual device yet.
        if unsafe {
            D3D12CreateDevice(
                &adapter,
                D3D_FEATURE_LEVEL_11_0,
                std::ptr::null_mut::<Option<ID3D12Device>>(),
            )
        }
        .is_ok()
        {
            return Ok(adapter);
        }
    }

    unreachable!()
}
