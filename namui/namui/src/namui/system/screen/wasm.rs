use crate::system::InitResult;
use crate::*;
use std::sync::atomic::AtomicU32;
use std::sync::OnceLock;

pub(crate) async fn init() -> InitResult {
    let window_wh = unsafe { initial_window_wh() };
    on_resize((window_wh >> 16) as u16, (window_wh & 0xffff) as u16);

    Ok(())
}

#[repr(u8)]
#[allow(dead_code)]
enum EventType {
    OnAnimationFrame = 0,
    ScreenResize,
    KeyDown,
    KeyUp,
    MouseDown,
    MouseMove,
    MouseUp,
    Wheel,
    Blur,
    VisibilityChange,
}

// event type and body
// - 0x00: on animation frame
// - 0x01: on resize
//     - u16: width
//     - u16: height
// - 0x02 ~ 0x03: on key down, up
//     - u8: code byte length
//     - bytes: code
// - 0x04 ~ 0x06: on mouse down, move, up
//     - u8: button
//     - u8: buttons
//     - u16: x
//     - u16: y
// - 0x07: on wheel
//     - f32: delta x
//     - f32: delta y
//     - u16: mouse x
//     - u16: mouse y
// - 0x08: on blur
// - 0x09: on visibility change

extern "C" {
    fn poll_event(ptr: *const u8) -> u8;
    fn initial_window_wh() -> u32;
}

pub(crate) fn run_event_hook_loop(component: impl 'static + Fn(&RenderCtx) + Send) {
    tokio::task::spawn_blocking(|| unsafe {
        let mut looper = Looper::new(component);
        loop {
            let buffer = [0u8; 32];
            let length = poll_event(buffer.as_ptr());
            let packet = &buffer[0..(length as usize)];

            let event_type: EventType = std::mem::transmute(packet[0]);

            let raw_event: RawEvent = match event_type {
                EventType::OnAnimationFrame => RawEvent::ScreenRedraw,
                EventType::ScreenResize => {
                    let width =
                        u16::from_be_bytes(packet[1..3].try_into().expect("invalid width bytes"));
                    let height =
                        u16::from_be_bytes(packet[3..5].try_into().expect("invalid height bytes"));

                    on_resize(width, height);

                    let wh = crate::Wh {
                        width: (width as i32).int_px(),
                        height: (height as i32).int_px(),
                    };
                    RawEvent::ScreenResize { wh }
                }
                EventType::KeyDown | EventType::KeyUp => {
                    let code_length = packet[1] as usize;
                    let code_str = std::str::from_utf8(&packet[2..(2 + code_length)])
                        .expect("invalid code bytes");
                    let func = match event_type {
                        EventType::KeyDown => crate::keyboard::on_key_down,
                        EventType::KeyUp => crate::keyboard::on_key_up,
                        _ => unreachable!(),
                    };
                    func(code_str)
                }
                EventType::MouseDown | EventType::MouseMove | EventType::MouseUp => {
                    let button: u8 = packet[1];
                    let buttons: u8 = packet[2];
                    let x = u16::from_be_bytes(packet[3..5].try_into().expect("invalid x bytes"));
                    let y = u16::from_be_bytes(packet[5..7].try_into().expect("invalid y bytes"));

                    let func = match event_type {
                        EventType::MouseDown => crate::mouse::on_mouse_down,
                        EventType::MouseMove => crate::mouse::on_mouse_move,
                        EventType::MouseUp => crate::mouse::on_mouse_up,
                        _ => unreachable!(),
                    };

                    func(x, y, button, buttons)
                }
                EventType::Wheel => {
                    let delta_x =
                        f32::from_be_bytes(packet[1..5].try_into().expect("invalid x bytes"));
                    let delta_y =
                        f32::from_be_bytes(packet[5..9].try_into().expect("invalid y bytes"));
                    let x = u16::from_be_bytes(packet[9..11].try_into().expect("invalid x bytes"));
                    let y = u16::from_be_bytes(packet[11..13].try_into().expect("invalid y bytes"));

                    crate::mouse::on_mouse_wheel(delta_x, delta_y, x, y)
                }
                EventType::Blur => {
                    crate::keyboard::on_blur();
                    RawEvent::Blur
                }
                EventType::VisibilityChange => {
                    crate::keyboard::on_visibility_change();
                    RawEvent::VisibilityChange
                }
            };

            looper.tick(raw_event);
        }
    });
}

/*
    event packet
    - header: 1byte (event type)
    - body: depends on event type
*/

// width 16bits, height 16bits
static SIZE: OnceLock<AtomicU32> = OnceLock::new();

fn on_resize(width: u16, height: u16) {
    SIZE.get_or_init(|| AtomicU32::new(unsafe { initial_window_wh() }))
        .store(
            (width as u32) << 16 | height as u32,
            std::sync::atomic::Ordering::Relaxed,
        );

    let wh = crate::Wh {
        width: (width as i32).int_px(),
        height: (height as i32).int_px(),
    };

    skia::on_window_resize(wh);

    // crate::hooks::on_raw_event(RawEvent::ScreenResize { wh });
}

pub fn size() -> crate::Wh<IntPx> {
    let size = SIZE
        .get_or_init(|| AtomicU32::new(unsafe { initial_window_wh() }))
        .load(std::sync::atomic::Ordering::Relaxed);
    crate::Wh {
        width: ((size >> 16) as i32).int_px(),
        height: ((size & 0xffff) as i32).int_px(),
    }
}
