use crate::*;
use anyhow::Result;
use namui_type::*;
use winit::raw_window_handle::HasWindowHandle;
use ::windows::Win32::{
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
}
unsafe impl Send for NativeSkia {}
unsafe impl Sync for NativeSkia {}

impl NativeSkia {
    pub(crate) fn new(window: &winit::window::Window, window_wh: Wh<IntPx>) -> Result<NativeSkia> {
        let window_handle = window
            .window_handle()
            .expect("Failed to retrieve a window handle");
        let raw_window_handle = window_handle.as_raw();

        let hwnd = match raw_window_handle {
            winit::raw_window_handle::RawWindowHandle::Win32(handle) => {
                HWND(handle.hwnd.get() as _)
            }
            _ => panic!("Expected Win32 window handle"),
        };

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
