use crate::system::InitResult;
use crate::*;
use std::sync::OnceLock;
use std::sync::atomic::AtomicU32;

pub(crate) fn init() -> InitResult {
    let window_wh = unsafe { _initial_window_wh() };
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
#[derive(Debug)]
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
    fn _initial_window_wh() -> u32;
}

thread_local! {
    static RENDERING_TREE_BYTES: RefCell<Box<[u8]>> = Default::default();
}

#[unsafe(no_mangle)]
pub extern "C" fn _on_event(ptr: *const u8, len: usize, out_ptr: *mut u8, out_len: *mut u8) {
    TOKIO_RUNTIME.with(|tokio_runtime| {
        let _guard = tokio_runtime.enter();

        let packet = unsafe { std::slice::from_raw_parts(ptr, len) };
        let event_type: EventType = unsafe { std::mem::transmute(packet[0]) };
        let event = parse_event(event_type, packet, on_resize);

        LOOPER.with_borrow_mut(|looper_cell| {
            let Some(rendering_tree) = looper_cell.as_mut().unwrap().tick(event) else {
                unsafe { std::slice::from_raw_parts_mut(out_ptr, std::mem::size_of::<usize>()) }
                    .copy_from_slice(&(0_usize).to_le_bytes());
                unsafe { std::slice::from_raw_parts_mut(out_len, std::mem::size_of::<usize>()) }
                    .copy_from_slice(&(0_usize).to_le_bytes());
                return;
            };

            RENDERING_TREE_BYTES.with_borrow_mut(|bytes| {
                *bytes = bincode::encode_to_vec(rendering_tree, bincode::config::standard())
                    .unwrap()
                    .into_boxed_slice();

                let len = bytes.len();
                unsafe { std::slice::from_raw_parts_mut(out_ptr, std::mem::size_of::<usize>()) }
                    .copy_from_slice(&(bytes.as_ptr() as usize).to_le_bytes());
                unsafe { std::slice::from_raw_parts_mut(out_len, std::mem::size_of::<usize>()) }
                    .copy_from_slice(&(len).to_le_bytes());
            })
        })
    })
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
            todo!()
        }
        EventType::Wheel => {
            todo!()
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
    SIZE.get_or_init(|| AtomicU32::new(unsafe { _initial_window_wh() }))
        .store(
            (width as u32) << 16 | height as u32,
            std::sync::atomic::Ordering::Relaxed,
        );

    let wh = crate::Wh {
        width: (width as i32).int_px(),
        height: (height as i32).int_px(),
    };

    // crate::hooks::on_raw_event(RawEvent::ScreenResize { wh });
}

pub fn size() -> crate::Wh<IntPx> {
    let size = SIZE
        .get_or_init(|| AtomicU32::new(unsafe { _initial_window_wh() }))
        .load(std::sync::atomic::Ordering::Relaxed);
    crate::Wh {
        width: ((size >> 16) as i32).int_px(),
        height: ((size & 0xffff) as i32).int_px(),
    }
}
