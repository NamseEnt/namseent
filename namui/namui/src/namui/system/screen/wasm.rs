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
}

extern "C" {
    fn poll_event(ptr: *const u8) -> u8;
    fn initial_window_wh() -> u32;
}

pub(crate) fn run_event_hook_loop(component: impl 'static + Fn(&RenderCtx) + Send) {
    tokio::task::spawn_blocking(|| unsafe {
        let mut looper = Looper::new(component);
        loop {
            let buffer = [0u8; 16];
            let length = poll_event(buffer.as_ptr());
            let packet = &buffer[0..(length as usize)];

            let event_type: EventType = std::mem::transmute(packet[0]);

            let raw_event: RawEvent = match event_type {
                EventType::OnAnimationFrame => RawEvent::ScreenRedraw,
                EventType::ScreenResize => {
                    /*
                        body
                        - width: 16bits
                        - height: 16bits
                    */

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
