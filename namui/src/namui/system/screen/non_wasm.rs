use crate::{system::InitResult, *};
use std::sync::OnceLock;

static WINDOW: OnceLock<winit::window::Window> = OnceLock::new();

pub(crate) async fn init() -> InitResult {
    let event_loop = winit::event_loop::EventLoop::new()?;
    let winit_window_builder = winit::window::WindowBuilder::new()
        .with_title("namui")
        .with_inner_size(winit::dpi::LogicalSize::new(800, 800));

    let window = winit_window_builder.build(&event_loop)?;
    WINDOW.set(window).unwrap();

    Ok(())
}

pub fn size() -> crate::Wh<IntPx> {
    let window = WINDOW.get().unwrap();
    crate::Wh {
        width: (window.inner_size().width as i32).int_px(),
        height: (window.inner_size().height as i32).int_px(),
    }
}
