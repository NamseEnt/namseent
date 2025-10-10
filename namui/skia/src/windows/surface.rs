use crate::*;
use anyhow::Result;
use namui_type::*;
use skia_safe::gpu::d3d::{ID3D12CommandQueue, ID3D12Device, ID3D12Resource};
use windows::{
    Win32::{
        Foundation::{HANDLE, HWND},
        Graphics::{
            Direct3D12::{
                D3D12_CPU_DESCRIPTOR_HANDLE, D3D12_DESCRIPTOR_HEAP_DESC,
                D3D12_DESCRIPTOR_HEAP_TYPE_RTV, D3D12_FENCE_FLAG_NONE,
                D3D12_RESOURCE_STATE_PRESENT, ID3D12DescriptorHeap, ID3D12Fence,
            },
            Dxgi::{
                Common::{DXGI_FORMAT_R8G8B8A8_UNORM, DXGI_SAMPLE_DESC},
                CreateDXGIFactory1, DXGI_PRESENT, DXGI_SWAP_CHAIN_DESC1, DXGI_SWAP_CHAIN_FLAG,
                DXGI_SWAP_EFFECT_FLIP_DISCARD, DXGI_USAGE_RENDER_TARGET_OUTPUT, IDXGIFactory4,
                IDXGISwapChain3,
            },
        },
        System::Threading::{CreateEventA, INFINITE, WaitForSingleObjectEx},
    },
    core::*,
};

const FRAME_COUNT: u32 = 2;

pub(crate) struct NativeSurface {
    surfaces: Vec<skia_safe::surface::Surface>,
    swap_chain: IDXGISwapChain3,
    context: skia_safe::gpu::DirectContext,
    buffer_index: usize,
    render_targets: Vec<ID3D12Resource>,
    fence: ID3D12Fence,
    fence_values: Vec<u64>,
    fence_event: HANDLE,
    command_queue: ID3D12CommandQueue,
    device: ID3D12Device,
    rtv_heap: ID3D12DescriptorHeap,
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
            CreateDXGIFactory1::<IDXGIFactory4>()
                .expect("CreateDXGIFactory1 failed")
                .CreateSwapChainForHwnd(command_queue, hwnd, &swap_chain_desc, None, None)
                .expect("CreateSwapChainForHwnd failed")
        }
        .cast()
        .expect("cast failed from IDXGISwapChain1 to IDXGISwapChain3");

        let buffer_index = unsafe { swap_chain.GetCurrentBackBufferIndex() } as usize;

        let rtv_heap: ID3D12DescriptorHeap = unsafe {
            device.CreateDescriptorHeap(&D3D12_DESCRIPTOR_HEAP_DESC {
                NumDescriptors: FRAME_COUNT,
                Type: D3D12_DESCRIPTOR_HEAP_TYPE_RTV,
                ..Default::default()
            })
        }?;

        let (surfaces, render_targets) =
            setup_surfaces(device, &swap_chain, &mut context, window_wh, &rtv_heap)?;

        let fence_values = (0..FRAME_COUNT).map(|_| 1000).collect::<Vec<_>>();
        let fence: ID3D12Fence =
            unsafe { device.CreateFence(fence_values[buffer_index], D3D12_FENCE_FLAG_NONE) }?;
        let fence_event = unsafe { CreateEventA(None, false, false, None)? };

        Ok(Self {
            device: device.clone(),
            context,
            surfaces,
            swap_chain,
            buffer_index,
            render_targets,
            fence,
            fence_values,
            fence_event,
            command_queue: command_queue.clone(),
            rtv_heap,
        })
    }

    pub(crate) fn resize(&mut self, window_wh: Wh<IntPx>) {
        let desc = unsafe { self.swap_chain.GetDesc1() }.unwrap();

        if desc.Width == window_wh.width.as_i32() as u32
            && desc.Height == window_wh.height.as_i32() as u32
        {
            return;
        }

        self.context.flush(None);
        self.context.submit(Some(skia_safe::gpu::SyncCpu::Yes));

        for i in 0..(FRAME_COUNT as usize) {
            if unsafe { self.fence.GetCompletedValue() } < self.fence_values[i] {
                unsafe {
                    self.fence
                        .SetEventOnCompletion(self.fence_values[i], self.fence_event)
                        .unwrap()
                };
            }
            self.surfaces.remove(0);
            self.render_targets.remove(0);
        }

        unsafe {
            self.swap_chain
                .ResizeBuffers(
                    0,
                    window_wh.width.as_i32() as u32,
                    window_wh.height.as_i32() as u32,
                    DXGI_FORMAT_R8G8B8A8_UNORM,
                    DXGI_SWAP_CHAIN_FLAG(0),
                )
                .expect("swap_chain.resize_buffers failed");
        };

        let (surfaces, render_targets) = setup_surfaces(
            &self.device,
            &self.swap_chain,
            &mut self.context,
            window_wh,
            &self.rtv_heap,
        )
        .unwrap();
        self.surfaces = surfaces;
        self.render_targets = render_targets;
    }

    /// Should be called before use surface
    pub(crate) fn move_to_next_frame(&mut self) {
        let current_fence_value = self.fence_values[self.buffer_index];
        self.buffer_index = unsafe { self.swap_chain.GetCurrentBackBufferIndex() } as usize;

        if unsafe { self.fence.GetCompletedValue() } < self.fence_values[self.buffer_index] {
            unsafe {
                self.fence
                    .SetEventOnCompletion(self.fence_values[self.buffer_index], self.fence_event)
                    .unwrap();
            };
            unsafe {
                WaitForSingleObjectEx(self.fence_event, INFINITE, false);
            };
        }

        self.fence_values[self.buffer_index] = current_fence_value.wrapping_add(1);
    }
}

impl SkSurface for NativeSurface {
    fn flush(&mut self) {
        let surface = &mut self.surfaces[self.buffer_index];

        self.context.flush_surface_with_access(
            surface,
            skia_safe::surface::BackendSurfaceAccess::Present,
            &Default::default(),
        );
        self.context.submit(None);

        unsafe {
            self.swap_chain
                .Present(1, DXGI_PRESENT(0))
                .ok()
                .expect("swap_chain.present failed")
        };

        unsafe {
            self.command_queue
                .Signal(&self.fence, self.fence_values[self.buffer_index])
                .unwrap()
        };
    }

    fn canvas(&mut self) -> &dyn SkCanvas {
        self.surfaces[self.buffer_index].canvas()
    }
}

fn setup_surfaces(
    device: &ID3D12Device,
    swap_chain: &IDXGISwapChain3,
    context: &mut skia_safe::gpu::DirectContext,
    window_wh: Wh<IntPx>,
    rtv_heap: &ID3D12DescriptorHeap,
) -> Result<(Vec<skia_safe::surface::Surface>, Vec<ID3D12Resource>)> {
    let rtv_descriptor_size =
        unsafe { device.GetDescriptorHandleIncrementSize(D3D12_DESCRIPTOR_HEAP_TYPE_RTV) } as usize;

    let rtv_handle = D3D12_CPU_DESCRIPTOR_HANDLE {
        ptr: unsafe { rtv_heap.GetCPUDescriptorHandleForHeapStart() }.ptr,
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
                    resource_state: D3D12_RESOURCE_STATE_PRESENT,
                    format: DXGI_FORMAT_R8G8B8A8_UNORM,
                    sample_count: 1,
                    level_count: 1,
                    sample_quality_pattern: 0,
                    protected: skia_safe::gpu::Protected::No,
                },
            );

            skia_safe::gpu::surfaces::wrap_backend_render_target(
                context,
                &backend_render_target,
                skia_safe::gpu::SurfaceOrigin::TopLeft,
                skia_safe::ColorType::RGBA8888,
                None,
                None,
            )
            .ok_or(anyhow::anyhow!("wrap_backend_render_target failed"))
        })
        .collect::<Result<Vec<_>>>()?;

    Ok((surfaces, render_targets))
}
