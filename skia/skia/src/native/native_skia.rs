use super::*;
use crate::*;
use anyhow::Result;
use windows::{
    core::ComInterface,
    Win32::{
        Foundation::HWND,
        Graphics::{
            Direct3D::D3D_FEATURE_LEVEL_11_0,
            Direct3D12::{
                D3D12CreateDevice, ID3D12CommandQueue, ID3D12DescriptorHeap, ID3D12Device,
                ID3D12Resource, D3D12_COMMAND_LIST_TYPE_DIRECT, D3D12_COMMAND_QUEUE_DESC,
                D3D12_COMMAND_QUEUE_FLAG_NONE, D3D12_CPU_DESCRIPTOR_HANDLE,
                D3D12_DESCRIPTOR_HEAP_DESC, D3D12_DESCRIPTOR_HEAP_TYPE_RTV,
                D3D12_RESOURCE_STATE_COMMON,
            },
            Dxgi::{
                Common::{
                    DXGI_FORMAT_R8G8B8A8_UNORM, DXGI_SAMPLE_DESC,
                    DXGI_STANDARD_MULTISAMPLE_QUALITY_PATTERN,
                },
                CreateDXGIFactory1, IDXGIAdapter1, IDXGIFactory4, IDXGISwapChain3,
                DXGI_ADAPTER_FLAG, DXGI_ADAPTER_FLAG_NONE, DXGI_ADAPTER_FLAG_SOFTWARE,
                DXGI_SWAP_CHAIN_DESC1, DXGI_SWAP_EFFECT_FLIP_DISCARD,
                DXGI_USAGE_RENDER_TARGET_OUTPUT,
            },
        },
    },
};

pub(crate) struct NativeSkia {
    context: skia_safe::gpu::DirectContext,
    surfaces: Vec<skia_safe::surface::Surface>,

    // d3d12 stuff
    swap_chain: IDXGISwapChain3,
}
unsafe impl Send for NativeSkia {}
unsafe impl Sync for NativeSkia {}

impl NativeSkia {
    pub(crate) fn new(window_id: usize, window_wh: Wh<IntPx>) -> Result<NativeSkia> {
        const FRAME_COUNT: u32 = 2;
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
            factory.CreateSwapChainForHwnd(&command_queue, hwnd, &swap_chain_desc, None, None)?
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

        let backend_context = skia_safe::gpu::d3d::BackendContext {
            adapter,
            device,
            queue: command_queue,
            memory_allocator: None,
            protected_context: skia_safe::gpu::Protected::No,
        };

        let mut context =
            unsafe { skia_safe::gpu::DirectContext::new_d3d(&backend_context, None).unwrap() };

        let surfaces = render_targets
            .iter()
            .map(|render_target| {
                let backend_render_target = skia_safe::gpu::BackendRenderTarget::new_d3d(
                    (window_wh.width.as_i32(), window_wh.height.as_i32()),
                    // &skia_safe::gpu::d3d::TextureResourceInfo::from_resource(render_targets.pop().unwrap()),
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
        })
    }
}

impl SkSkia for NativeSkia {
    fn group_glyph(&self, font: &Font, paint: &Paint) -> Arc<dyn GroupGlyph> {
        todo!()
    }

    fn font_metrics(&self, font: &Font) -> Option<FontMetrics> {
        todo!()
    }

    fn load_typeface(&self, typeface_name: &str, bytes: &[u8]) {
        todo!()
    }

    fn image(&self, image_source: &ImageSource) -> Option<Image> {
        todo!()
    }

    fn path_contains_xy(&self, path: &Path, paint: Option<&Paint>, xy: Xy<Px>) -> bool {
        todo!()
    }

    fn path_bounding_box(&self, path: &Path, paint: Option<&Paint>) -> Option<Rect<Px>> {
        todo!()
    }

    fn encode_loaded_image_to_png(
        &self,
        image: &Image,
    ) -> std::pin::Pin<Box<dyn std::future::Future<Output = Vec<u8>>>> {
        todo!()
    }
    // fn group_glyph(&self, font: &Font, paint: &Paint) -> std::sync::Arc<dyn GroupGlyph> {
    //     CkGroupGlyph::get(font, paint)
    // }

    // fn font_metrics(&self, font: &Font) -> Option<FontMetrics> {
    //     CkFont::get(font).map(|x| x.metrics)
    // }

    // fn load_typeface(&self, typeface_name: &str, bytes: &[u8]) {
    //     CkTypeface::load(typeface_name, bytes)
    // }

    // fn image(&self, image_source: &ImageSource) -> Option<Image> {
    //     CkImage::get(image_source).map(|x| x.image())
    // }

    // fn path_contains_xy(&self, path: &Path, paint: Option<&Paint>, xy: Xy<Px>) -> bool {
    //     CkPath::get(path).contains(paint, xy)
    // }

    // fn path_bounding_box(&self, path: &Path, paint: Option<&Paint>) -> Option<Rect<Px>> {
    //     CkPath::get(path).bounding_box(paint)
    // }

    // fn encode_loaded_image_to_png(
    //     &self,
    //     image: &Image,
    // ) -> std::pin::Pin<Box<dyn std::future::Future<Output = Vec<u8>>>> {
    //     let ck_image = CkImage::get(&image.src).unwrap();
    //     Box::pin(async move { ck_image.encode_to_png().await })
    // }
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
