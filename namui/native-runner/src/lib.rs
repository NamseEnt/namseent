use namui_rendering_tree::*;
use namui_type::*;
use winit::{
    application::ApplicationHandler,
    dpi::LogicalSize,
    event::WindowEvent,
    event_loop::{ActiveEventLoop, EventLoop},
    keyboard::PhysicalKey,
    window::{Window, WindowAttributes, WindowId},
};

unsafe extern "C" {
    fn namui_main();
    fn namui_init_system();
    fn namui_set_screen_size(width: u16, height: u16);
    fn namui_on_screen_redraw(out_ptr: *mut *const u8, out_len: *mut usize) -> u64;
    fn namui_on_screen_resize(width: u16, height: u16) -> u64;
    fn namui_on_mouse_move(x: f32, y: f32) -> u64;
    fn namui_on_mouse_down(button: u8, x: f32, y: f32) -> u64;
    fn namui_on_mouse_up(button: u8, x: f32, y: f32) -> u64;
    fn namui_on_mouse_wheel(delta_x: f32, delta_y: f32, mouse_x: f32, mouse_y: f32) -> u64;
    fn namui_on_key_down(code: u8) -> u64;
    fn namui_on_key_up(code: u8) -> u64;
    fn namui_on_blur() -> u64;
    fn namui_shutdown();
}

struct NamuiApp {
    window: Option<Window>,
    skia: Option<namui_skia::NativeSkia>,
}

std::thread_local! {
    static MOUSE_POS: std::cell::RefCell<(usize, usize)> = const { std::cell::RefCell::new((0, 0)) };
}

impl ApplicationHandler for NamuiApp {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        if self.window.is_some() {
            return;
        }

        eprintln!("[runner] resumed: creating window");
        let size = LogicalSize::new(1280, 720);
        let mut window_attributes = WindowAttributes::default();
        window_attributes.inner_size = Some(winit::dpi::Size::new(size));
        window_attributes.title = "namui".to_string();

        let window = event_loop
            .create_window(window_attributes)
            .expect("Failed to create window");

        let inner_size = window.inner_size();
        eprintln!("[runner] resumed: window created {}x{}", inner_size.width, inner_size.height);

        let window_wh = Wh::new(
            int_px(inner_size.width as i32),
            int_px(inner_size.height as i32),
        );

        eprintln!("[runner] resumed: init skia");
        let skia = namui_skia::init_skia(&window, window_wh)
            .expect("Failed to initialize Skia Metal backend");
        eprintln!("[runner] resumed: skia initialized");

        unsafe {
            eprintln!("[runner] resumed: calling namui_init_system");
            namui_init_system();
            eprintln!("[runner] resumed: calling namui_main");
            namui_main();
            eprintln!("[runner] resumed: calling namui_set_screen_size");
            namui_set_screen_size(inner_size.width as u16, inner_size.height as u16);
            eprintln!("[runner] resumed: calling namui_on_screen_resize");
            namui_on_screen_resize(inner_size.width as u16, inner_size.height as u16);
            eprintln!("[runner] resumed: FFI calls done");
        }

        self.window = Some(window);
        self.skia = Some(skia);

        self.window.as_ref().unwrap().request_redraw();
        eprintln!("[runner] resumed: done");
    }

    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        _window_id: WindowId,
        event: WindowEvent,
    ) {
        let Some(skia) = self.skia.as_mut() else {
            return;
        };

        match event {
            WindowEvent::CloseRequested => {
                unsafe { namui_shutdown(); }
                event_loop.exit();
            }
            WindowEvent::Resized(size) => {
                let wh = Wh::new(int_px(size.width as i32), int_px(size.height as i32));
                skia.on_resize(wh);
                unsafe {
                    namui_on_screen_resize(size.width as u16, size.height as u16);
                }
                self.window.as_ref().unwrap().request_redraw();
            }
            WindowEvent::RedrawRequested => {
                // on_event returns: 0 = no change, 1 = tree changed, 2 = mouse moved
                let mut ptr: *const u8 = std::ptr::null();
                let mut len: usize = 0;
                let result = unsafe { namui_on_screen_redraw(&mut ptr, &mut len) };

                let (mx, my) = MOUSE_POS.with(|pos| *pos.borrow());

                skia.move_to_next_frame();
                skia.surface().canvas().clear(Color::WHITE);

                if result == 1 {
                    let slice = unsafe { std::slice::from_raw_parts(ptr, len) };
                    let (rendering_tree, _): (namui_rendering_tree::RenderingTree, usize) =
                        bincode::decode_from_slice(slice, bincode::config::standard()).unwrap();
                    namui_drawer::draw_rendering_tree(skia, rendering_tree, mx, my);
                } else {
                    namui_drawer::redraw(skia, mx, my);
                }

                skia.surface().flush();

                self.window.as_ref().unwrap().request_redraw();
            }
            WindowEvent::MouseInput { state, button, .. } => {
                let btn = match button {
                    winit::event::MouseButton::Left => 0u8,
                    winit::event::MouseButton::Right => 1u8,
                    winit::event::MouseButton::Middle => 2u8,
                    _ => return,
                };
                let (mx, my) = MOUSE_POS.with(|pos| {
                    let pos = pos.borrow();
                    (pos.0 as f32, pos.1 as f32)
                });
                if state.is_pressed() {
                    unsafe { namui_on_mouse_down(btn, mx, my) };
                } else {
                    unsafe { namui_on_mouse_up(btn, mx, my) };
                }
                self.window.as_ref().unwrap().request_redraw();
            }
            WindowEvent::CursorMoved { position, .. } => {
                MOUSE_POS.with(|pos| {
                    *pos.borrow_mut() = (position.x as usize, position.y as usize);
                });
                unsafe { namui_on_mouse_move(position.x as f32, position.y as f32) };
            }
            WindowEvent::MouseWheel { delta, .. } => {
                let (dx, dy) = match delta {
                    winit::event::MouseScrollDelta::LineDelta(x, y) => (x, y),
                    winit::event::MouseScrollDelta::PixelDelta(d) => (d.x as f32, d.y as f32),
                };
                let (mx, my) = MOUSE_POS.with(|pos| {
                    let pos = pos.borrow();
                    (pos.0 as f32, pos.1 as f32)
                });
                unsafe { namui_on_mouse_wheel(dx, dy, mx, my) };
                self.window.as_ref().unwrap().request_redraw();
            }
            WindowEvent::KeyboardInput { event, .. } => {
                if let PhysicalKey::Code(key_code) = event.physical_key {
                    if let Some(code) = winit_key_to_code(key_code) {
                        if event.state.is_pressed() {
                            unsafe { namui_on_key_down(code) };
                        } else {
                            unsafe { namui_on_key_up(code) };
                        }
                        self.window.as_ref().unwrap().request_redraw();
                    }
                }
            }
            WindowEvent::Focused(false) => {
                unsafe { namui_on_blur() };
            }
            _ => {}
        }
    }
}

pub fn run() {
    eprintln!("[runner] run(): creating event loop");
    let event_loop = EventLoop::new().expect("Failed to create event loop");
    eprintln!("[runner] run(): event loop created");

    let mut app = NamuiApp {
        window: None,
        skia: None,
    };

    eprintln!("[runner] run(): entering autoreleasepool + run_app");
    objc2::rc::autoreleasepool(|_| {
        event_loop.run_app(&mut app).expect("Event loop failed");
    });
    eprintln!("[runner] run(): done");
}

/// Convert winit KeyCode to namui Code u8 value.
/// Must match the Code enum repr(u8) ordering in rendering-tree/src/types/codes.rs
fn winit_key_to_code(key_code: winit::keyboard::KeyCode) -> Option<u8> {
    use winit::keyboard::KeyCode as K;

    let code: u8 = match key_code {
        K::Escape => 0,
        K::Digit1 => 1,
        K::Digit2 => 2,
        K::Digit3 => 3,
        K::Digit4 => 4,
        K::Digit5 => 5,
        K::Digit6 => 6,
        K::Digit7 => 7,
        K::Digit8 => 8,
        K::Digit9 => 9,
        K::Digit0 => 10,
        K::Minus => 11,
        K::Equal => 12,
        K::Backspace => 13,
        K::Tab => 14,
        K::KeyQ => 15,
        K::KeyW => 16,
        K::KeyE => 17,
        K::KeyR => 18,
        K::KeyT => 19,
        K::KeyY => 20,
        K::KeyU => 21,
        K::KeyI => 22,
        K::KeyO => 23,
        K::KeyP => 24,
        K::BracketLeft => 25,
        K::BracketRight => 26,
        K::Enter => 27,
        K::ControlLeft => 28,
        K::KeyA => 29,
        K::KeyS => 30,
        K::KeyD => 31,
        K::KeyF => 32,
        K::KeyG => 33,
        K::KeyH => 34,
        K::KeyJ => 35,
        K::KeyK => 36,
        K::KeyL => 37,
        K::Semicolon => 38,
        K::Quote => 39,
        K::Backquote => 40,
        K::ShiftLeft => 41,
        K::Backslash => 42,
        K::KeyZ => 43,
        K::KeyX => 44,
        K::KeyC => 45,
        K::KeyV => 46,
        K::KeyB => 47,
        K::KeyN => 48,
        K::KeyM => 49,
        K::Comma => 50,
        K::Period => 51,
        K::Slash => 52,
        K::ShiftRight => 53,
        K::AltLeft => 54,
        K::Space => 55,
        K::CapsLock => 56,
        K::F1 => 57,
        K::F2 => 58,
        K::F3 => 59,
        K::F4 => 60,
        K::F5 => 61,
        K::F6 => 62,
        K::F7 => 63,
        K::F8 => 64,
        K::F9 => 65,
        K::F10 => 66,
        K::Pause => 67,
        K::ScrollLock => 68,
        K::IntlBackslash => 69,
        K::F11 => 70,
        K::F12 => 71,
        K::ControlRight => 72,
        K::PrintScreen => 73,
        K::AltRight => 74,
        K::NumLock => 75,
        K::Home => 76,
        K::ArrowUp => 77,
        K::PageUp => 78,
        K::ArrowLeft => 79,
        K::ArrowRight => 80,
        K::End => 81,
        K::ArrowDown => 82,
        K::PageDown => 83,
        K::Insert => 84,
        K::Delete => 85,
        K::ContextMenu => 86,
        K::IntlRo => 87,
        K::IntlYen => 88,
        K::SuperLeft => 89,
        K::SuperRight => 90,
        K::Convert => 91,
        K::KanaMode => 92,
        K::Lang1 => 93,
        K::Lang2 => 94,
        K::Lang3 => 95,
        K::Lang4 => 96,
        K::Lang5 => 97,
        K::NonConvert => 98,
        K::Help => 99,
        K::Numpad0 => 100,
        K::Numpad1 => 101,
        K::Numpad2 => 102,
        K::Numpad3 => 103,
        K::Numpad4 => 104,
        K::Numpad5 => 105,
        K::Numpad6 => 106,
        K::Numpad7 => 107,
        K::Numpad8 => 108,
        K::Numpad9 => 109,
        K::NumpadAdd => 110,
        K::NumpadBackspace => 111,
        K::NumpadClear => 112,
        K::NumpadClearEntry => 113,
        K::NumpadComma => 114,
        K::NumpadDecimal => 115,
        K::NumpadDivide => 116,
        K::NumpadEnter => 117,
        K::NumpadEqual => 118,
        K::NumpadHash => 119,
        K::NumpadMemoryAdd => 120,
        K::NumpadMemoryClear => 121,
        K::NumpadMemoryRecall => 122,
        K::NumpadMemoryStore => 123,
        K::NumpadMemorySubtract => 124,
        K::NumpadMultiply => 125,
        K::NumpadParenLeft => 126,
        K::NumpadParenRight => 127,
        K::NumpadStar => 128,
        K::NumpadSubtract => 129,
        K::Fn => 130,
        K::FnLock => 131,
        K::BrowserBack => 132,
        K::BrowserFavorites => 133,
        K::BrowserForward => 134,
        K::BrowserHome => 135,
        K::BrowserRefresh => 136,
        K::BrowserSearch => 137,
        K::BrowserStop => 138,
        K::Eject => 139,
        K::LaunchApp1 => 140,
        K::LaunchApp2 => 141,
        K::LaunchMail => 142,
        K::MediaPlayPause => 143,
        K::MediaSelect => 144,
        K::MediaStop => 145,
        K::MediaTrackNext => 146,
        K::MediaTrackPrevious => 147,
        K::Power => 148,
        K::Sleep => 149,
        K::AudioVolumeDown => 150,
        K::AudioVolumeMute => 151,
        K::AudioVolumeUp => 152,
        K::WakeUp => 153,
        K::Meta => 154,
        K::Hyper => 155,
        K::Turbo => 156,
        K::Abort => 157,
        K::Resume => 158,
        K::Suspend => 159,
        K::Again => 160,
        K::Copy => 161,
        K::Cut => 162,
        K::Find => 163,
        K::Open => 164,
        K::Paste => 165,
        K::Props => 166,
        K::Select => 167,
        K::Undo => 168,
        K::Hiragana => 169,
        K::Katakana => 170,
        K::F13 => 171,
        K::F14 => 172,
        K::F15 => 173,
        K::F16 => 174,
        K::F17 => 175,
        K::F18 => 176,
        K::F19 => 177,
        K::F20 => 178,
        K::F21 => 179,
        K::F22 => 180,
        K::F23 => 181,
        K::F24 => 182,
        K::F25 => 183,
        K::F26 => 184,
        K::F27 => 185,
        K::F28 => 186,
        K::F29 => 187,
        K::F30 => 188,
        K::F31 => 189,
        K::F32 => 190,
        K::F33 => 191,
        K::F34 => 192,
        K::F35 => 193,
        _ => return None,
    };
    Some(code)
}
