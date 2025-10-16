use crate::{system::InitResult, *};
use std::sync::OnceLock;

static WINDOW: OnceLock<winit::window::Window> = OnceLock::new();

pub(crate) fn init() -> InitResult {
    // TODO
    // while WINDOW.get().is_none() {
    //     tokio::time::sleep(std::time::Duration::from_millis(10)).await;
    // }

    Ok(())
}

pub fn size() -> crate::Wh<IntPx> {
    let window = WINDOW.get().unwrap();
    crate::Wh {
        width: (window.inner_size().width as i32).int_px(),
        height: (window.inner_size().height as i32).int_px(),
    }
}

pub(crate) fn window_id() -> usize {
    u64::from(WINDOW.get().unwrap().id()) as usize
}

// pub(crate) fn take_main_thread<Root: Component + Clone>(component: Root) {
//     let event_loop = winit::event_loop::EventLoopBuilder::new().build().unwrap();
//     let winit_window_builder = winit::window::WindowBuilder::new()
//         .with_title("namui")
//         .with_inner_size(winit::dpi::LogicalSize::new(800, 800));

//     let window = winit_window_builder.build(&event_loop).unwrap();
//     WINDOW.set(window).unwrap();

//     println!("Window created");

//     while !system::SYSTEM_INITIALIZED.load(std::sync::atomic::Ordering::SeqCst) {
//         std::thread::sleep(std::time::Duration::from_millis(10));
//     }

//     println!("Start event loop");

//     let mut looper = Looper::new(component);

//     event_loop
//         .run(|event, target| {
//             target.set_control_flow(winit::event_loop::ControlFlow::WaitUntil(
//                 std::time::Instant::now() + std::time::Duration::from_millis(10),
//             ));

//             let raw_event = match event {
//                 winit::event::Event::WindowEvent {
//                     window_id: _,
//                     event,
//                 } => match event {
//                     winit::event::WindowEvent::Resized(size) => {
//                         let wh = Wh {
//                             width: (size.width as i32).int_px(),
//                             height: (size.height as i32).int_px(),
//                         };
//                         system::skia::on_window_resize(wh);
//                         RawEvent::ScreenResize { wh }
//                     }
//                     winit::event::WindowEvent::CloseRequested
//                     | winit::event::WindowEvent::Destroyed => {
//                         std::process::exit(0);
//                     }
//                     winit::event::WindowEvent::KeyboardInput {
//                         device_id: _,
//                         event,
//                         is_synthetic: _,
//                     } => {
//                         let Some(raw_event) = system::keyboard::on_keyboard_input(event) else {
//                             return;
//                         };
//                         raw_event
//                     }
//                     winit::event::WindowEvent::CursorMoved {
//                         device_id: _,
//                         position,
//                     } => system::mouse::on_winit_cursor_moved(position),
//                     winit::event::WindowEvent::MouseWheel {
//                         device_id: _,
//                         delta,
//                         phase: _,
//                     } => system::mouse::on_winit_mouse_wheel(delta),
//                     winit::event::WindowEvent::MouseInput {
//                         device_id: _,
//                         state,
//                         button,
//                     } => {
//                         let namui_mouse_button = match button {
//                             winit::event::MouseButton::Left => MouseButton::Left,
//                             winit::event::MouseButton::Right => MouseButton::Right,
//                             winit::event::MouseButton::Middle => MouseButton::Middle,
//                             winit::event::MouseButton::Back
//                             | winit::event::MouseButton::Forward
//                             | winit::event::MouseButton::Other(_) => {
//                                 return;
//                             }
//                         };
//                         system::mouse::on_winit_mouse_input(state, namui_mouse_button)
//                     }
//                     winit::event::WindowEvent::RedrawRequested => RawEvent::ScreenRedraw,
//                     _ => {
//                         return;
//                     }
//                 },
//                 winit::event::Event::NewEvents(winit::event::StartCause::ResumeTimeReached {
//                     ..
//                 }) => RawEvent::ScreenRedraw,
//                 _ => {
//                     return;
//                 }
//             };

//             looper.tick(raw_event);
//         })
//         .unwrap();
//     println!("Event loop finished");
// }
