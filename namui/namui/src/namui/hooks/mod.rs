mod image;

pub use image::*;
pub use namui_hooks::*;
use namui_skia::RawEvent;
use namui_type::*;
use std::sync::{atomic::AtomicUsize, Mutex, OnceLock};

static RAW_EVENT_TX: OnceLock<std::sync::mpsc::Sender<RawEventWithSentTime>> = OnceLock::new();

pub(crate) fn run_loop<C: Component>(root_component: impl 'static + Fn() -> C) {
    let (raw_event_tx, raw_event_rx) = std::sync::mpsc::channel();
    RAW_EVENT_TX.set(raw_event_tx).unwrap();

    let mut world = World::init(crate::time::now, crate::system::skia::sk_calculate());
    let rendering_tree = world.run(root_component());
    crate::system::drawer::request_draw_rendering_tree(rendering_tree);

    let mut one_sec_timer = std::time::Instant::now();
    let mut one_sec_render_count = 0;
    let mut event_handle_delay_sum = 0.ms();
    let mut render_time_sum = 0.ms();
    let mut render_time_worst = 0.ms();
    let mut event_type_count = vec![
        (EventType::MouseDown, 0),
        (EventType::MouseMove, 0),
        (EventType::MouseUp, 0),
        (EventType::Wheel, 0),
        (EventType::KeyDown, 0),
        (EventType::KeyUp, 0),
        (EventType::Blur, 0),
        (EventType::VisibilityChange, 0),
        (EventType::ScreenResize, 0),
        (EventType::TextInputTextUpdated, 0),
        (EventType::TextInputKeyDown, 0),
        (EventType::SelectionChange, 0),
        (EventType::ScreenRedraw, 0),
    ];

    while let Ok(RawEventWithSentTime { event, sent }) = raw_event_rx.recv() {
        one_sec_render_count += 1;
        event_handle_delay_sum += Duration::from(sent.elapsed());
        event_type_count
            .iter_mut()
            .find(|(event_type, _)| *event_type == EventType::from_raw_event(&event))
            .unwrap()
            .1 += 1;

        let now = crate::time::now();

        let rendering_tree = world.run_with_event(root_component(), event);
        crate::system::drawer::request_draw_rendering_tree(rendering_tree);

        let elapsed = crate::time::now() - now;
        if elapsed > 33.ms() {
            println!("Warning: Rendering took {elapsed:?}. Keep it short as possible.",);
        }

        render_time_sum += elapsed;
        render_time_worst = render_time_worst.max(elapsed);

        if one_sec_timer.elapsed() > std::time::Duration::from_secs(1) {
            println!(
                "Render count: {}/sec | Event handle avg delay: {:?} | Render avg time: {:?} | Worst render time: {:?}",
                one_sec_render_count,
                event_handle_delay_sum / one_sec_render_count,
                render_time_sum / one_sec_render_count,
                render_time_worst,
            );
            for (event_type, count) in &mut event_type_count {
                if *count > 0 {
                    println!("- {:?}: {}", event_type, count);
                    *count = 0;
                }
            }
            one_sec_render_count = 0;
            one_sec_timer = std::time::Instant::now();
            event_handle_delay_sum = 0.ms();
            render_time_sum = 0.ms();
        }
    }
}

pub(crate) fn on_raw_event(event: RawEvent) {
    {
        static EVENT_COUNT: AtomicUsize = AtomicUsize::new(0);
        EVENT_COUNT.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
        static TIME: OnceLock<Mutex<std::time::Instant>> = OnceLock::new();

        let mut one_sec_timer = TIME
            .get_or_init(|| Mutex::new(std::time::Instant::now()))
            .lock()
            .unwrap();

        if one_sec_timer.elapsed() > std::time::Duration::from_secs(1) {
            println!(
                "Event recv count {}/sec",
                EVENT_COUNT.swap(0, std::sync::atomic::Ordering::Relaxed)
            );
            *one_sec_timer = std::time::Instant::now();
        }
    }

    if let Some(tx) = RAW_EVENT_TX.get() {
        tx.send(RawEventWithSentTime {
            event,
            sent: std::time::Instant::now(),
        })
        .unwrap()
    }
}

struct RawEventWithSentTime {
    event: RawEvent,
    sent: std::time::Instant,
}

#[derive(Debug, PartialEq)]
enum EventType {
    MouseDown,
    MouseMove,
    MouseUp,
    Wheel,
    KeyDown,
    KeyUp,
    Blur,
    VisibilityChange,
    ScreenResize,
    TextInputTextUpdated,
    TextInputKeyDown,
    SelectionChange,
    ScreenRedraw,
}

impl EventType {
    fn from_raw_event(event: &RawEvent) -> Self {
        match event {
            RawEvent::MouseDown { .. } => EventType::MouseDown,
            RawEvent::MouseMove { .. } => EventType::MouseMove,
            RawEvent::MouseUp { .. } => EventType::MouseUp,
            RawEvent::Wheel { .. } => EventType::Wheel,
            RawEvent::KeyDown { .. } => EventType::KeyDown,
            RawEvent::KeyUp { .. } => EventType::KeyUp,
            RawEvent::Blur => EventType::Blur,
            RawEvent::VisibilityChange => EventType::VisibilityChange,
            RawEvent::ScreenResize { .. } => EventType::ScreenResize,
            RawEvent::ScreenRedraw => EventType::ScreenRedraw,
        }
    }
}
