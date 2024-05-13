use namui_hooks::*;
use namui_skia::RawEvent;
use namui_type::*;
use std::{
    cell::RefCell,
    sync::{atomic::AtomicUsize, Mutex, OnceLock},
};

type RootComponentBox = Box<dyn 'static + Fn(&RenderCtx)>;

thread_local! {
    static WORLD: RefCell<World> = RefCell::new(World::init(crate::time::now, crate::system::skia::sk_calculate()));
    static ROOT_COMPONENT: OnceLock<RootComponentBox> = OnceLock::new();
}

pub(crate) fn run_loop(root_component: impl 'static + Fn(&RenderCtx)) {
    WORLD.with_borrow_mut(|world| {
        let rendering_tree = world.run(&root_component);
        crate::system::skia::request_draw_rendering_tree(rendering_tree);

        ROOT_COMPONENT
            .with(|lock| lock.set(Box::new(root_component)))
            .map_err(|_| ())
            .unwrap();
    })
}

pub(crate) fn on_raw_event(event: RawEvent) {
    if !crate::system::system_initialized() {
        return;
    }

    {
        static EVENT_COUNT: AtomicUsize = AtomicUsize::new(0);
        EVENT_COUNT.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
        static TIME: OnceLock<Mutex<Instant>> = OnceLock::new();

        let mut one_sec_timer = TIME
            .get_or_init(|| Mutex::new(Instant::now()))
            .lock()
            .unwrap();

        if Instant::now() - *one_sec_timer > Duration::from_secs(1) {
            crate::log!(
                "Event recv count {}/sec",
                EVENT_COUNT.swap(0, std::sync::atomic::Ordering::Relaxed)
            );
            *one_sec_timer = Instant::now();
        }
    }

    thread_local! {
        static STAT: RefCell<Stat> = RefCell::new(Stat {
            one_sec_timer: Instant::now(),
            one_sec_render_count: 0,
            render_time_sum: Duration::from_secs(0),
            render_time_worst: Duration::from_secs(0),
            event_type_count: [
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
            ],
        });
    }

    STAT.with_borrow_mut(|stat| {
        WORLD.with_borrow_mut(|world| {
            ROOT_COMPONENT
                .with(|root_component| tick(stat, world, root_component.get().unwrap(), event))
        })
    })
}

struct Stat {
    one_sec_timer: Instant,
    one_sec_render_count: usize,
    render_time_sum: Duration,
    render_time_worst: Duration,
    event_type_count: [(EventType, usize); 13],
}

fn tick(
    stat: &mut Stat,
    world: &mut World,
    root_component: &(impl 'static + Fn(&RenderCtx)),
    event: RawEvent,
) {
    stat.one_sec_render_count += 1;
    stat.event_type_count
        .iter_mut()
        .find(|(event_type, _)| *event_type == EventType::from_raw_event(&event))
        .unwrap()
        .1 += 1;

    let now = crate::time::now();

    let rendering_tree = world.run_with_event(root_component, event);
    crate::system::skia::request_draw_rendering_tree(rendering_tree);

    let elapsed = crate::time::now() - now;
    if elapsed > 33.ms() {
        crate::log!("Warning: Rendering took {elapsed:?}. Keep it short as possible.",);
    }

    stat.render_time_sum += elapsed;
    stat.render_time_worst = stat.render_time_worst.max(elapsed);

    if Instant::now() - stat.one_sec_timer > Duration::from_secs(1) {
        crate::log!(
            "Render count: {}/sec | Render avg time: {:?} | Worst render time: {:?}",
            stat.one_sec_render_count,
            stat.render_time_sum / stat.one_sec_render_count,
            stat.render_time_worst,
        );
        for (event_type, count) in stat.event_type_count.iter_mut() {
            if *count > 0 {
                crate::log!("- {:?}: {}", event_type, count);
                *count = 0;
            }
        }
        stat.one_sec_render_count = 0;
        stat.one_sec_timer = Instant::now();
        stat.render_time_sum = 0.ms();
    }
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
