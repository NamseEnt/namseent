#[cfg(not(all(target_os = "windows", feature = "d3d")))]
fn main() {
    println!("This example requires the `d3d` feature to be enabled on Windows.");
    println!("Run it with `cargo run --example d3d-window --features d3d`");
}

#[cfg(all(target_os = "windows", feature = "d3d"))]
fn main() -> anyhow::Result<()> {
    // NOTE: Most of code is from https://github.com/microsoft/windows-rs/blob/02db74cf5c4796d970e6d972cdc7bc3967380079/crates/samples/windows/direct3d12/src/main.rs

    use skia_safe::{
        gpu::{
            d3d::{BackendContext, TextureResourceInfo},
            surfaces, BackendRenderTarget, DirectContext, Protected, SurfaceOrigin,
        },
        paint, Color, ColorType, Paint, Rect,
    };
    use windows::{
        core::Interface,
        Win32::{
            Foundation::HWND,
            Graphics::{
                Direct3D::D3D_FEATURE_LEVEL_11_0,
                Direct3D12::{D3D12CreateDevice, ID3D12Device, D3D12_RESOURCE_STATE_COMMON},
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
    use winit::{
        event::{Event, WindowEvent},
        keyboard::{Key, NamedKey},
    };

    const BUFFER_COUNT: u32 = 2;

    let event_loop = winit::event_loop::EventLoop::new()?;
    let window = winit::window::WindowBuilder::new()
        .with_title("rust-skia-gl-window")
        .with_inner_size(winit::dpi::LogicalSize::new(800, 800))
        .build(&event_loop)?;

    let hwnd = HWND(u64::from(window.id()) as isize);
    let (width, height) = window.inner_size().into();

    let factory: IDXGIFactory4 = unsafe { CreateDXGIFactory1() }?;
    let (adapter, device) = get_hardware_adapter_and_device(&factory)?;
    let queue = unsafe { device.CreateCommandQueue(&Default::default()) }?;

    let backend_context = BackendContext {
        adapter,
        device,
        queue,
        memory_allocator: None,
        protected_context: Protected::No,
    };
    let mut direct_context = unsafe { DirectContext::new_d3d(&backend_context, None) }.unwrap();

    let swap_chain: IDXGISwapChain3 = unsafe {
        factory.CreateSwapChainForHwnd(
            &backend_context.queue,
            hwnd,
            &DXGI_SWAP_CHAIN_DESC1 {
                Width: width,
                Height: height,
                Format: DXGI_FORMAT_R8G8B8A8_UNORM,
                BufferUsage: DXGI_USAGE_RENDER_TARGET_OUTPUT,
                BufferCount: BUFFER_COUNT,
                SwapEffect: DXGI_SWAP_EFFECT_FLIP_DISCARD,
                SampleDesc: DXGI_SAMPLE_DESC {
                    Count: 1,
                    Quality: 0,
                },
                ..Default::default()
            },
            None,
            None,
        )
    }?
    .cast()?;

    let mut surfaces: [_; BUFFER_COUNT as _] = std::array::from_fn(|i| {
        let resource = unsafe { swap_chain.GetBuffer(i as u32).unwrap() };

        let backend_render_target = BackendRenderTarget::new_d3d(
            window.inner_size().into(),
            &TextureResourceInfo {
                resource,
                alloc: None,
                resource_state: D3D12_RESOURCE_STATE_COMMON,
                format: DXGI_FORMAT_R8G8B8A8_UNORM,
                sample_count: 1,
                level_count: 0,
                sample_quality_pattern: DXGI_STANDARD_MULTISAMPLE_QUALITY_PATTERN,
                protected: Protected::No,
            },
        );

        let surface = surfaces::wrap_backend_render_target(
            &mut direct_context,
            &backend_render_target,
            SurfaceOrigin::BottomLeft,
            ColorType::RGBA8888,
            None,
            None,
        )
        .unwrap();

        (surface, backend_render_target)
    });

    fn get_hardware_adapter_and_device(
        factory: &IDXGIFactory4,
    ) -> windows::core::Result<(IDXGIAdapter1, ID3D12Device)> {
        for i in 0.. {
            let adapter = unsafe { factory.EnumAdapters1(i) }?;

            let mut adapter_desc = Default::default();
            unsafe { adapter.GetDesc1(&mut adapter_desc) }?;

            if (DXGI_ADAPTER_FLAG(adapter_desc.Flags as _) & DXGI_ADAPTER_FLAG_SOFTWARE)
                != DXGI_ADAPTER_FLAG_NONE
            {
                continue; // Don't select the Basic Render Driver adapter.
            }

            let mut device = None;
            if unsafe { D3D12CreateDevice(&adapter, D3D_FEATURE_LEVEL_11_0, &mut device) }.is_ok() {
                return Ok((adapter, device.unwrap()));
            }
        }
        unreachable!()
    }

    println!("Skia initialized with {} surfaces.", surfaces.len());
    println!("Use Arrow Keys to move the rectangle.");

    struct State {
        x: f32,
        y: f32,
    }
    let mut state = State { x: 100.0, y: 100.0 };

    let mut render = |state: &State| {
        let index = unsafe { swap_chain.GetCurrentBackBufferIndex() };
        let (surface, _) = &mut surfaces[index as usize];
        let canvas = surface.canvas();

        canvas.clear(Color::BLUE);

        let mut paint = Paint::default();
        paint.set_color(Color::RED);
        paint.set_style(paint::Style::StrokeAndFill);
        paint.set_anti_alias(true);
        paint.set_stroke_width(10.0);

        canvas.draw_rect(Rect::from_xywh(state.x, state.y, 200.0, 200.0), &paint);

        direct_context.flush_and_submit_surface(surface, None);

        unsafe { swap_chain.Present(1, 0) }.unwrap();

        // NOTE: If you get some error when you render, you can check it with:
        // unsafe {
        //     device.GetDeviceRemovedReason().ok().unwrap();
        // }
    };

    event_loop.run(move |event, window_target| {
        if let Event::WindowEvent { event, .. } = event {
            match event {
                WindowEvent::KeyboardInput { event, .. } => {
                    match event.logical_key {
                        Key::Named(NamedKey::ArrowLeft) => state.x -= 10.0,
                        Key::Named(NamedKey::ArrowRight) => state.x += 10.0,
                        Key::Named(NamedKey::ArrowUp) => state.y += 10.0,
                        Key::Named(NamedKey::ArrowDown) => state.y -= 10.0,
                        Key::Named(NamedKey::Escape) => window_target.exit(),
                        _ => return,
                    }
                    window.request_redraw();
                }
                WindowEvent::RedrawRequested => render(&state),
                WindowEvent::CloseRequested => window_target.exit(),
                _ => {}
            }
        };
    })?;

    Ok(())
}
