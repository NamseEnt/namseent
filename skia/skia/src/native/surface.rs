use crate::*;
use skia_safe::gpu::d3d::{ID3D12CommandQueue, ID3D12Device, ID3D12Resource};
use windows::{
    core::ComInterface,
    Win32::{
        Foundation::HWND,
        Graphics::{
            Direct3D12::{
                ID3D12DescriptorHeap, D3D12_CPU_DESCRIPTOR_HANDLE, D3D12_DESCRIPTOR_HEAP_DESC,
                D3D12_DESCRIPTOR_HEAP_TYPE_RTV, D3D12_RESOURCE_STATE_COMMON,
            },
            Dxgi::{
                Common::{
                    DXGI_FORMAT_R8G8B8A8_UNORM, DXGI_SAMPLE_DESC,
                    DXGI_STANDARD_MULTISAMPLE_QUALITY_PATTERN,
                },
                CreateDXGIFactory1, IDXGIFactory4, IDXGISwapChain3, DXGI_SWAP_CHAIN_DESC1,
                DXGI_SWAP_EFFECT_FLIP_DISCARD, DXGI_USAGE_RENDER_TARGET_OUTPUT,
            },
        },
    },
};

pub(crate) struct NativeSurface {
    surfaces: Vec<skia_safe::surface::Surface>,
    swap_chain: IDXGISwapChain3,
    context: skia_safe::gpu::DirectContext,
    surface_index: usize,
}
unsafe impl Send for NativeSurface {}
unsafe impl Sync for NativeSurface {}

impl NativeSurface {
    pub(crate) fn new(
        mut context: skia_safe::gpu::DirectContext,
        window_wh: Wh<IntPx>,
        device: &ID3D12Device,
        command_queue: &ID3D12CommandQueue,
        hwnd: HWND,
    ) -> Result<Self> {
        const FRAME_COUNT: u32 = 2;

        let swap_chain_desc = DXGI_SWAP_CHAIN_DESC1 {
            BufferCount: FRAME_COUNT,
            Width: window_wh.width.as_i32() as u32,
            Height: window_wh.height.as_i32() as u32,
            Format: DXGI_FORMAT_R8G8B8A8_UNORM,
            BufferUsage: DXGI_USAGE_RENDER_TARGET_OUTPUT,
            SwapEffect: DXGI_SWAP_EFFECT_FLIP_DISCARD,
            SampleDesc: DXGI_SAMPLE_DESC {
                Count: 1,
                ..Default::default()
            },
            ..Default::default()
        };

        let swap_chain: IDXGISwapChain3 = unsafe {
            CreateDXGIFactory1::<IDXGIFactory4>()?.CreateSwapChainForHwnd(
                command_queue,
                hwnd,
                &swap_chain_desc,
                None,
                None,
            )?
        }
        .cast()?;

        let frame_index = unsafe { swap_chain.GetCurrentBackBufferIndex() };

        let rtv_heap: ID3D12DescriptorHeap = unsafe {
            device.CreateDescriptorHeap(&D3D12_DESCRIPTOR_HEAP_DESC {
                NumDescriptors: FRAME_COUNT,
                Type: D3D12_DESCRIPTOR_HEAP_TYPE_RTV,
                ..Default::default()
            })
        }?;

        let rtv_descriptor_size =
            unsafe { device.GetDescriptorHandleIncrementSize(D3D12_DESCRIPTOR_HEAP_TYPE_RTV) }
                as usize;

        let rtv_handle = D3D12_CPU_DESCRIPTOR_HANDLE {
            ptr: unsafe { rtv_heap.GetCPUDescriptorHandleForHeapStart() }.ptr
                + frame_index as usize * rtv_descriptor_size,
        };

        let render_targets: Vec<ID3D12Resource> = {
            let mut render_targets = vec![];
            for i in 0..FRAME_COUNT {
                let render_target: ID3D12Resource = unsafe { swap_chain.GetBuffer(i)? };
                unsafe {
                    device.CreateRenderTargetView(
                        &render_target,
                        None,
                        D3D12_CPU_DESCRIPTOR_HANDLE {
                            ptr: rtv_handle.ptr + i as usize * rtv_descriptor_size,
                        },
                    )
                };
                render_targets.push(render_target);
            }
            render_targets
        };

        let surfaces = render_targets
            .iter()
            .map(|render_target| {
                let backend_render_target = skia_safe::gpu::BackendRenderTarget::new_d3d(
                    (window_wh.width.as_i32(), window_wh.height.as_i32()),
                    &skia_safe::gpu::d3d::TextureResourceInfo {
                        resource: render_target.clone(),
                        alloc: None,
                        resource_state: D3D12_RESOURCE_STATE_COMMON,
                        format: DXGI_FORMAT_R8G8B8A8_UNORM,
                        sample_count: 1,
                        level_count: 0,
                        sample_quality_pattern: DXGI_STANDARD_MULTISAMPLE_QUALITY_PATTERN,
                        protected: skia_safe::gpu::Protected::No,
                    },
                );

                skia_safe::gpu::surfaces::wrap_backend_render_target(
                    &mut context,
                    &backend_render_target,
                    skia_safe::gpu::SurfaceOrigin::BottomLeft,
                    skia_safe::ColorType::RGBA8888,
                    None,
                    None,
                )
                .ok_or(anyhow::anyhow!("wrap_backend_render_target failed"))
            })
            .collect::<Result<Vec<_>>>()?;

        Ok(Self {
            context,
            surfaces,
            swap_chain,
            surface_index: 0,
        })
    }
}

impl SkSurface for NativeSurface {
    fn flush(&mut self) {
        let surface = &mut self.surfaces[self.surface_index];
        self.context.flush_and_submit_surface(surface, None);
        unsafe {
            self.swap_chain
                .Present(1, 0)
                .ok()
                .expect("swap_chain.present failed")
        };

        self.surface_index = (self.surface_index + 1) % self.surfaces.len();
    }

    fn canvas(&self) -> &dyn SkCanvas {
        todo!()
    }
}
