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
    fn _init_system();
    fn _set_screen_size(width: u16, height: u16);
    fn _shutdown();
    fn _on_animation_frame() -> *const u8;
    fn _on_screen_resize(width: u16, height: u16) -> *const u8;
    fn _on_mouse_down(x: f32, y: f32, button: u8, buttons: u8) -> *const u8;
    fn _on_mouse_move(x: f32, y: f32, button: u8, buttons: u8) -> *const u8;
    fn _on_mouse_up(x: f32, y: f32, button: u8, buttons: u8) -> *const u8;
    fn _on_mouse_wheel(delta_x: f32, delta_y: f32, x: f32, y: f32) -> *const u8;
    fn _on_key_down(code: u8) -> *const u8;
    fn _on_key_up(code: u8) -> *const u8;
    fn _on_blur() -> *const u8;
    fn _on_visibility_change() -> *const u8;
    fn _dylib_image_buffer_list(out: *mut usize, max_count: usize) -> usize;
    fn _dylib_register_font(
        name_ptr: *const u8,
        name_len: usize,
        buffer_ptr: *const u8,
        buffer_len: usize,
    );
    fn _dylib_set_image_infos(ptr: *const u8, count: usize);
}

/// Decode response from namui FFI: `[len: u32 LE][data...]`
/// Returns None if ptr is null (no change).
/// Returns Some(slice) where slice is the data portion.
/// An empty slice means "redraw with previous rendering tree".
unsafe fn decode_response(ptr: *const u8) -> Option<&'static [u8]> {
    if ptr.is_null() {
        return None;
    }
    let len = unsafe { (ptr as *const u32).read_unaligned() } as usize;
    if len == 0 {
        return Some(&[]);
    }
    Some(unsafe { std::slice::from_raw_parts(ptr.add(4), len) })
}

struct NamuiApp {
    window: Option<Window>,
    skia: Option<namui_skia::NativeSkia>,
}

std::thread_local! {
    static MOUSE_STATE: std::cell::RefCell<MouseState> = const { std::cell::RefCell::new(MouseState::new()) };
}

struct MouseState {
    x: f32,
    y: f32,
    /// DOM-convention bitmask: bit0=left, bit1=right, bit2=middle
    buttons: u8,
}

impl MouseState {
    const fn new() -> Self {
        Self {
            x: 0.0,
            y: 0.0,
            buttons: 0,
        }
    }
}

/// Convert winit MouseButton to DOM convention button value.
/// DOM: 0=left, 1=middle, 2=right
fn winit_button_to_dom(button: winit::event::MouseButton) -> Option<u8> {
    match button {
        winit::event::MouseButton::Left => Some(0),
        winit::event::MouseButton::Middle => Some(1),
        winit::event::MouseButton::Right => Some(2),
        _ => None,
    }
}

/// Convert DOM button value to bitmask bit.
/// DOM buttons bitmask: bit0=left, bit1=right, bit2=middle
fn dom_button_to_bitmask(button: u8) -> u8 {
    match button {
        0 => 1 << 0, // left
        1 => 1 << 2, // middle
        2 => 1 << 1, // right
        _ => 0,
    }
}

impl ApplicationHandler for NamuiApp {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        if self.window.is_some() {
            return;
        }

        let size = LogicalSize::new(1280, 720);
        let mut window_attributes = WindowAttributes::default();
        window_attributes.inner_size = Some(winit::dpi::Size::new(size));
        window_attributes.title = "namui".to_string();

        let window = event_loop
            .create_window(window_attributes)
            .expect("Failed to create window");

        let inner_size = window.inner_size();

        let window_wh = Wh::new(
            int_px(inner_size.width as i32),
            int_px(inner_size.height as i32),
        );

        let skia = namui_skia::init_skia(&window, window_wh)
            .expect("Failed to initialize Skia Metal backend");

        unsafe {
            _init_system();
            namui_main();

            // Re-register images from dylib into runner's IMAGES map
            let mut buf = vec![0usize; 1000 * 3];
            let count = _dylib_image_buffer_list(buf.as_mut_ptr(), 1000);
            for i in 0..count {
                let id = buf[i * 3];
                let ptr = buf[i * 3 + 1] as *const u8;
                let len = buf[i * 3 + 2];
                register_image(id, ptr, len);
            }

            // Forward image infos to dylib's IMAGE_INFOS map (like web's _set_image_infos)
            let image_info_size = 14; // id(4) + alpha_type(1) + color_type(1) + width(4) + height(4)
            let max_images = 1000;
            let mut info_buf = vec![0u8; max_images * image_info_size];
            let info_count = _image_infos(info_buf.as_mut_ptr(), max_images);
            if info_count > 0 {
                _dylib_set_image_infos(info_buf.as_ptr(), info_count);
            }

            _set_screen_size(inner_size.width as u16, inner_size.height as u16);
            _on_screen_resize(inner_size.width as u16, inner_size.height as u16);
        }

        self.window = Some(window);
        self.skia = Some(skia);

        self.window.as_ref().unwrap().request_redraw();
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
                unsafe {
                    _shutdown();
                }
                event_loop.exit();
            }
            WindowEvent::Resized(size) => {
                let wh = Wh::new(int_px(size.width as i32), int_px(size.height as i32));
                skia.on_resize(wh);
                unsafe {
                    _on_screen_resize(size.width as u16, size.height as u16);
                }
                self.window.as_ref().unwrap().request_redraw();
            }
            WindowEvent::RedrawRequested => {
                let response = unsafe { decode_response(_on_animation_frame()) };

                let (mx, my) = MOUSE_STATE.with(|s| {
                    let s = s.borrow();
                    (s.x as usize, s.y as usize)
                });

                skia.move_to_next_frame();
                skia.surface().canvas().clear(Color::WHITE);

                match response {
                    Some(data) if !data.is_empty() => {
                        let (rendering_tree, _): (namui_rendering_tree::RenderingTree, usize) =
                            bincode::decode_from_slice(data, bincode::config::standard()).unwrap();
                        namui_drawer::draw_rendering_tree(skia, rendering_tree, mx, my);
                    }
                    _ => {
                        namui_drawer::redraw(skia, mx, my);
                    }
                }

                skia.surface().flush();

                self.window.as_ref().unwrap().request_redraw();
            }
            WindowEvent::MouseInput { state, button, .. } => {
                let Some(dom_button) = winit_button_to_dom(button) else {
                    return;
                };
                let bit = dom_button_to_bitmask(dom_button);

                MOUSE_STATE.with(|s| {
                    let mut s = s.borrow_mut();
                    if state.is_pressed() {
                        s.buttons |= bit;
                        unsafe {
                            _on_mouse_down(s.x, s.y, dom_button, s.buttons);
                        }
                    } else {
                        s.buttons &= !bit;
                        unsafe {
                            _on_mouse_up(s.x, s.y, dom_button, s.buttons);
                        }
                    }
                });
                self.window.as_ref().unwrap().request_redraw();
            }
            WindowEvent::CursorMoved { position, .. } => {
                MOUSE_STATE.with(|s| {
                    let mut s = s.borrow_mut();
                    s.x = position.x as f32;
                    s.y = position.y as f32;
                    unsafe {
                        _on_mouse_move(s.x, s.y, 0, s.buttons);
                    }
                });
            }
            WindowEvent::MouseWheel { delta, .. } => {
                let (dx, dy) = match delta {
                    winit::event::MouseScrollDelta::LineDelta(x, y) => (x, y),
                    winit::event::MouseScrollDelta::PixelDelta(d) => (d.x as f32, d.y as f32),
                };
                let (mx, my) = MOUSE_STATE.with(|s| {
                    let s = s.borrow();
                    (s.x, s.y)
                });
                unsafe {
                    _on_mouse_wheel(dx, dy, mx, my);
                }
                self.window.as_ref().unwrap().request_redraw();
            }
            WindowEvent::KeyboardInput { event, .. } => {
                if let PhysicalKey::Code(key_code) = event.physical_key {
                    if let Some(code) = winit_key_to_code(key_code) {
                        if event.state.is_pressed() {
                            unsafe {
                                _on_key_down(code);
                            }
                        } else {
                            unsafe {
                                _on_key_up(code);
                            }
                        }
                        self.window.as_ref().unwrap().request_redraw();
                    }
                }
            }
            WindowEvent::Focused(false) => {
                unsafe {
                    _on_blur();
                }
            }
            WindowEvent::Occluded(_) => {
                unsafe {
                    _on_visibility_change();
                }
            }
            _ => {}
        }
    }
}

pub fn run(font_dir: &std::path::Path) {
    load_fonts(font_dir);

    let event_loop = EventLoop::new().expect("Failed to create event loop");

    let mut app = NamuiApp {
        window: None,
        skia: None,
    };

    objc2::rc::autoreleasepool(|_| {
        event_loop.run_app(&mut app).expect("Event loop failed");
    });
}

fn load_fonts(font_dir: &std::path::Path) {
    let map_path = font_dir.join("map.json");
    let map_str = std::fs::read_to_string(&map_path)
        .unwrap_or_else(|e| panic!("Failed to read {:?}: {e}", map_path));

    // map.json format: { "Lang": { "weight": "bundle:__system__/font/Lang/File.woff2", ... }, ... }
    // Extract all font file paths from the values.
    for line in map_str.lines() {
        let line = line.trim();
        if !line.contains("bundle:__system__/") {
            continue;
        }
        // Extract path after "bundle:__system__/"
        let Some(start) = line.find("bundle:__system__/") else {
            continue;
        };
        let rest = &line[start + "bundle:__system__/".len()..];
        let end = rest.find('"').unwrap_or(rest.len());
        let rel_path = &rest[..end];

        let font_path = font_dir.parent().unwrap().join(rel_path);
        let name = font_path
            .file_stem()
            .unwrap()
            .to_string_lossy()
            .to_string();
        let data = std::fs::read(&font_path)
            .unwrap_or_else(|e| panic!("Failed to read font {:?}: {e}", font_path));
        // Register in runner's own TYPEFACE_MAP (for namui_drawer rendering)
        NativeTypeface::load(&name, &data)
            .unwrap_or_else(|e| panic!("Failed to load font {name}: {e}"));
        // Register in dylib's TYPEFACE_MAP (for text measurement in app code)
        unsafe {
            _dylib_register_font(name.as_ptr(), name.len(), data.as_ptr(), data.len());
        }
    }
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

#[cfg(test)]
mod tests {
    use namui_rendering_tree::*;
    use namui_type::*;
    use std::sync::Once;

    static INIT: Once = Once::new();

    fn init_font() {
        INIT.call_once(|| {
            let font_path = concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/../namui-cli/system_bundle/font/Ko/NotoSansKR-Regular.woff2"
            );
            let data = std::fs::read(font_path)
                .unwrap_or_else(|e| panic!("Failed to read font file: {e}"));
            NativeTypeface::load("NotoSansKR-Regular", &data)
                .unwrap_or_else(|e| panic!("Failed to load font: {e}"));
        });
    }

    fn make_font(size: i32) -> Font {
        Font {
            name: "NotoSansKR-Regular".to_string(),
            size: int_px(size),
        }
    }

    fn default_paint() -> Paint {
        Paint::new(Color::BLACK)
    }

    #[test]
    fn font_metrics_are_valid() {
        init_font();
        let font = make_font(16);
        let metrics = font.font_metrics();
        assert!(
            metrics.ascent.as_f32() < 0.0,
            "ascent should be negative, got {}",
            metrics.ascent.as_f32()
        );
        assert!(
            metrics.descent.as_f32() > 0.0,
            "descent should be positive, got {}",
            metrics.descent.as_f32()
        );
        assert!(
            metrics.height().as_f32() > 0.0,
            "height should be positive, got {}",
            metrics.height().as_f32()
        );
    }

    #[test]
    fn text_width_is_positive() {
        init_font();
        let font = make_font(16);
        let paint = default_paint();
        let width = font.width("안녕하세요", &paint);
        assert!(
            width.as_f32() > 0.0,
            "width of non-empty text should be positive, got {}",
            width.as_f32()
        );
    }

    #[test]
    fn empty_text_width_is_zero() {
        init_font();
        let font = make_font(16);
        let paint = default_paint();
        let width = font.width("", &paint);
        assert!(
            width.as_f32() == 0.0,
            "width of empty text should be zero, got {}",
            width.as_f32()
        );
    }

    #[test]
    fn text_width_scales_with_font_size() {
        init_font();
        let paint = default_paint();
        let text = "Hello";
        let width_16 = make_font(16).width(text, &paint);
        let width_32 = make_font(32).width(text, &paint);
        assert!(
            width_32.as_f32() > width_16.as_f32(),
            "32px font width ({}) should be greater than 16px font width ({})",
            width_32.as_f32(),
            width_16.as_f32()
        );
    }

    #[test]
    fn per_glyph_widths_count_matches() {
        init_font();
        let font = make_font(16);
        let paint = default_paint();
        let text = "안녕하세요";
        let widths = font.widths(text, &paint);
        let glyph_count = text.chars().count();
        assert_eq!(
            widths.len(),
            glyph_count,
            "widths count ({}) should match glyph count ({})",
            widths.len(),
            glyph_count
        );
    }

    #[test]
    fn text_bound_has_positive_dimensions() {
        init_font();
        let font = make_font(16);
        let paint = default_paint();
        let bound = font.bound("안녕하세요", &paint);
        assert!(
            bound.width().as_f32() > 0.0,
            "bound width should be positive, got {}",
            bound.width().as_f32()
        );
        assert!(
            bound.height().as_f32() > 0.0,
            "bound height should be positive, got {}",
            bound.height().as_f32()
        );
    }

    #[test]
    fn per_glyph_bounds_count_matches() {
        init_font();
        let font = make_font(16);
        let paint = default_paint();
        let text = "안녕하세요";
        let bounds = font.bounds(text, &paint);
        let glyph_count = text.chars().count();
        assert_eq!(
            bounds.len(),
            glyph_count,
            "bounds count ({}) should match glyph count ({})",
            bounds.len(),
            glyph_count
        );
    }

    #[test]
    fn ascii_and_korean_have_different_widths() {
        init_font();
        let font = make_font(16);
        let paint = default_paint();
        let width_a = font.width("A", &paint);
        let width_ga = font.width("가", &paint);
        assert!(
            (width_a.as_f32() - width_ga.as_f32()).abs() > f32::EPSILON,
            "ASCII 'A' width ({}) and Korean '가' width ({}) should differ",
            width_a.as_f32(),
            width_ga.as_f32()
        );
    }
}
