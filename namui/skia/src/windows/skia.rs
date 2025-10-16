use crate::native::calculate::NativeCalculate;
use crate::*;
use anyhow::Result;
use namui_type::*;
use std::sync::Arc;
use windows::Win32::{
    Foundation::HWND,
    Graphics::{
        Direct3D::D3D_FEATURE_LEVEL_11_0,
        Direct3D12::{
            D3D12_COMMAND_LIST_TYPE_DIRECT, D3D12_COMMAND_QUEUE_DESC,
            D3D12_COMMAND_QUEUE_FLAG_NONE, D3D12CreateDevice, ID3D12CommandQueue, ID3D12Device,
        },
        Dxgi::{
            CreateDXGIFactory2, DXGI_ADAPTER_FLAG, DXGI_ADAPTER_FLAG_NONE,
            DXGI_ADAPTER_FLAG_SOFTWARE, DXGI_CREATE_FACTORY_FLAGS, IDXGIAdapter1, IDXGIFactory4,
        },
    },
};

pub struct NativeSkia {
    surface: NativeSurface,
    calculate: NativeCalculate,
}
unsafe impl Send for NativeSkia {}
unsafe impl Sync for NativeSkia {}

impl NativeSkia {
    pub(crate) fn new(window_id: usize, window_wh: Wh<IntPx>) -> Result<NativeSkia> {
        // unsafe {
        //     let mut debug: Option<ID3D12Debug> = None;
        //     if let Some(debug) = D3D12GetDebugInterface(&mut debug).ok().and(debug) {
        //         debug.EnableDebugLayer();
        //     }
        // }

        let hwnd = HWND(window_id as _);

        // Use `DXGI_CREATE_FACTORY_DEBUG` flag if needed.
        // https://github.com/NamseEnt/namseent/issues/738
        let factory = unsafe { CreateDXGIFactory2::<IDXGIFactory4>(DXGI_CREATE_FACTORY_FLAGS(0)) }?;
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
            calculate: NativeCalculate::new(),
        })
    }
}

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

fn get_hardware_adapter(factory: &IDXGIFactory4) -> Result<IDXGIAdapter1> {
    for i in 0.. {
        let adapter = unsafe { factory.EnumAdapters1(i)? };

        let desc = unsafe { adapter.GetDesc1()? };

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
