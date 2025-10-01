use crate::system::InitResult;
use crate::*;
use std::sync::OnceLock;
use std::sync::atomic::AtomicU32;

pub(crate) async fn init() -> InitResult {
    let window_wh = unsafe { initial_window_wh() };
    on_resize((window_wh >> 16) as u16, (window_wh & 0xffff) as u16);

    Ok(())
}

// event type and body
// - 0x00: on animation frame
// - 0x01: on resize
//     - u16: width
//     - u16: height
// - 0x02 ~ 0x03: on key down, up
//     - u8: code
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
// - 0x0A ~ 0x0B: on text input, selection change
//     - u16: text byte length
//     - bytes: text
//     - u8: selection direction. 0: none, 1: forward, 2: backward
//     - u16: selection start
//     - u16: selection end
// - 0x0C: on text input key down
//     - u16: text byte length
//     - bytes: text
//     - u8: selection direction. 0: none, 1: forward, 2: backward
//     - u16: selection start
//     - u16: selection end
//     - u8: code

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
    TextInput,
    TextInputSelectionChange,
    TextInputKeyDown,
}

unsafe extern "C" {
    fn poll_event(ptr: *const u8, wait_timeout_ms: usize) -> u8;
    fn initial_window_wh() -> u32;
}

pub(crate) fn run_event_hook_loop<Root: Component + Clone + Send + 'static>(component: Root) {
    tokio::task::spawn_blocking(|| {
        let mut looper = Looper::new(component);
        let buffer = [0u8; 8096];
        let mut next_raw_event = None;
        loop {
            let mut raw_event = next_raw_event
                .take()
                .unwrap_or_else(|| get_event(&buffer, usize::MAX).unwrap());

            while let Some(peek_raw_event) = get_event(&buffer, 0) {
                match (&mut raw_event, &peek_raw_event) {
                    (RawEvent::Wheel { event }, RawEvent::Wheel { event: peek_event }) => {
                        event.delta_xy += peek_event.delta_xy;
                        event.mouse_xy = peek_event.mouse_xy;
                    }
                    (RawEvent::MouseMove { .. }, RawEvent::MouseMove { .. })
                    | (RawEvent::MouseMove { .. }, RawEvent::MouseDown { .. })
                    | (RawEvent::MouseMove { .. }, RawEvent::MouseUp { .. })
                    | (RawEvent::ScreenResize { .. }, RawEvent::ScreenResize { .. })
                    | (RawEvent::ScreenRedraw, _)
                    | (
                        RawEvent::TextInputSelectionChange { .. },
                        RawEvent::TextInputSelectionChange { .. },
                    ) => {
                        raw_event = peek_raw_event;
                    }
                    _ => {
                        next_raw_event = Some(peek_raw_event);
                        break;
                    }
                }
            }

            looper.tick(raw_event);
        }
    });
}

fn get_event(buffer: &[u8], wait_timeout_ms: usize) -> Option<RawEvent> {
    unsafe {
        let length = poll_event(buffer.as_ptr(), wait_timeout_ms);
        if length == 0 {
            return None;
        }
        let packet = &buffer[0..(length as usize)];
        let event_type: EventType = std::mem::transmute(packet[0]);
        Some(parse_event(event_type, packet, on_resize))
    }
}

fn parse_event(event_type: EventType, packet: &[u8], on_resize: impl Fn(u16, u16)) -> RawEvent {
    match event_type {
        EventType::OnAnimationFrame => RawEvent::ScreenRedraw,
        EventType::ScreenResize => {
            let width = u16::from_be_bytes(packet[1..3].try_into().expect("invalid width bytes"));
            let height = u16::from_be_bytes(packet[3..5].try_into().expect("invalid height bytes"));

            on_resize(width, height);

            let wh = crate::Wh {
                width: (width as i32).int_px(),
                height: (height as i32).int_px(),
            };
            RawEvent::ScreenResize { wh }
        }
        EventType::KeyDown | EventType::KeyUp => {
            let code_number = packet[1];
            let code = Code::try_from(code_number).unwrap();
            let func = match event_type {
                EventType::KeyDown => crate::keyboard::on_key_down,
                EventType::KeyUp => crate::keyboard::on_key_up,
                _ => unreachable!(),
            };
            func(code)
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
            let delta_x = f32::from_be_bytes(packet[1..5].try_into().expect("invalid x bytes"));
            let delta_y = f32::from_be_bytes(packet[5..9].try_into().expect("invalid y bytes"));
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
        EventType::TextInput
        | EventType::TextInputKeyDown
        | EventType::TextInputSelectionChange => {
            let text_length =
                u16::from_be_bytes(packet[1..3].try_into().expect("invalid text length bytes"))
                    as usize;
            let text = std::str::from_utf8(&packet[3..(3 + text_length)])
                .expect("invalid text bytes")
                .to_string();
            let selection_direction = match packet[3 + text_length] {
                0 => SelectionDirection::None,
                1 => SelectionDirection::Forward,
                2 => SelectionDirection::Backward,
                _ => unreachable!(),
            };
            let selection_start = u16::from_be_bytes(
                packet[(4 + text_length)..(6 + text_length)]
                    .try_into()
                    .expect("invalid selection start bytes"),
            ) as usize;
            let selection_end = u16::from_be_bytes(
                packet[(6 + text_length)..(8 + text_length)]
                    .try_into()
                    .expect("invalid selection end bytes"),
            ) as usize;
            match event_type {
                EventType::TextInput => RawEvent::TextInput {
                    event: RawTextInputEvent {
                        text,
                        selection_direction,
                        selection_start,
                        selection_end,
                    },
                },
                EventType::TextInputSelectionChange => RawEvent::TextInputSelectionChange {
                    event: RawTextInputEvent {
                        text,
                        selection_direction,
                        selection_start,
                        selection_end,
                    },
                },
                EventType::TextInputKeyDown => {
                    let code_number = packet[8 + text_length];
                    let code = Code::try_from(code_number).unwrap();

                    RawEvent::TextInputKeyDown {
                        event: RawTextInputKeyDownEvent {
                            text,
                            selection_direction,
                            selection_start,
                            selection_end,
                            code,
                        },
                    }
                }
                _ => unreachable!(),
            }
        }
    }
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
