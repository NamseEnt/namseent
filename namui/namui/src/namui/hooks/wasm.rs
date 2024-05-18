use namui_hooks::*;
use namui_skia::RawEvent;
use namui_type::*;
use std::sync::{atomic::AtomicUsize, Mutex, OnceLock};

static WORLD: OnceLock<World> = OnceLock::new();
static ROOT_COMPONENT: OnceLock<Box<dyn 'static + Send + Sync + Fn(&RenderCtx)>> = OnceLock::new();

pub(crate) fn run_loop(root_component: impl 'static + Send + Sync + Fn(&RenderCtx)) {
    let mut world = World::init(crate::time::now, crate::system::skia::sk_calculate());
    let rendering_tree = world.run(root_component);
    crate::system::skia::request_draw_rendering_tree(rendering_tree);

    WORLD.set(world).map_err(|_| ()).unwrap();
    ROOT_COMPONENT
        .set(Box::new(root_component))
        .map_err(|_| ())
        .unwrap();
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

    static mut ONE_SEC_TIMER: Instant = Instant::now();
    static mut ONE_SEC_RENDER_COUNT: usize = 0;
    static mut RENDER_TIME_SUM: Duration = 0.ms();
    static mut RENDER_TIME_WORST: Duration = 0.ms();
    static mut EVENT_TYPE_COUNT: [(EventType, usize); 13] = [
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

    unsafe {
        let world = WORLD.get().unwrap();
        ONE_SEC_RENDER_COUNT += 1;
        EVENT_TYPE_COUNT
            .iter_mut()
            .find(|(event_type, _)| *event_type == EventType::from_raw_event(&event))
            .unwrap()
            .1 += 1;

        let now = crate::time::now();

        let root_component = ROOT_COMPONENT.get().unwrap();
        let rendering_tree = world.run_with_event(root_component, event);
        crate::system::skia::request_draw_rendering_tree(rendering_tree);

        let elapsed = crate::time::now() - now;
        if elapsed > 33.ms() {
            println!("Warning: Rendering took {elapsed:?}. Keep it short as possible.",);
        }

        RENDER_TIME_SUM += elapsed;
        RENDER_TIME_WORST = RENDER_TIME_WORST.max(elapsed);

        if ONE_SEC_TIMER - Instant::now() > Duration::from_secs(1) {
            println!(
                "Render count: {}/sec | Render avg time: {:?} | Worst render time: {:?}",
                ONE_SEC_RENDER_COUNT,
                RENDER_TIME_SUM / ONE_SEC_RENDER_COUNT,
                RENDER_TIME_WORST,
            );
            for (event_type, count) in EVENT_TYPE_COUNT.iter_mut() {
                if *count > 0 {
                    println!("- {:?}: {}", event_type, count);
                    *count = 0;
                }
            }
            ONE_SEC_RENDER_COUNT = 0;
            ONE_SEC_TIMER = Instant::now();
            RENDER_TIME_SUM = 0.ms();
        }
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
