use namui_hooks::*;
use namui_skia::RawEvent;
use namui_type::*;

pub(crate) struct Looper {
    world: World,
    one_sec_timer: std::time::Instant,
    one_sec_render_count: i32,
    render_time_sum: Duration,
    render_time_worst: Duration,
    event_type_count: Vec<(EventType, i32)>,
    root_component: Box<dyn Fn(&RenderCtx)>,
}
impl Looper {
    pub(crate) fn new(root_component: impl 'static + Fn(&RenderCtx)) -> Looper {
        let mut world = World::init(crate::time::now, crate::system::skia::sk_calculate_arc());
        let rendering_tree = world.run(&root_component);
        crate::system::skia::request_draw_rendering_tree(rendering_tree);

        Looper {
            world,
            one_sec_timer: std::time::Instant::now(),
            one_sec_render_count: 0,
            render_time_sum: 0.ms(),
            render_time_worst: 0.ms(),
            event_type_count: vec![
                (EventType::MouseDown, 0),
                (EventType::MouseMove, 0),
                (EventType::MouseUp, 0),
                (EventType::Wheel, 0),
                (EventType::KeyDown, 0),
                (EventType::KeyUp, 0),
                (EventType::Blur, 0),
                (EventType::VisibilityChange, 0),
                (EventType::ScreenResize, 0),
                (EventType::ScreenRedraw, 0),
                (EventType::TextInput, 0),
                (EventType::TextInputKeyDown, 0),
                (EventType::TextInputSelectionChange, 0),
            ],
            root_component: Box::new(root_component),
        }
    }

    pub(crate) fn tick(&mut self, event: RawEvent) {
        self.one_sec_render_count += 1;
        self.event_type_count
            .iter_mut()
            .find(|(event_type, _)| *event_type == EventType::from_raw_event(&event))
            .unwrap()
            .1 += 1;

        let now = crate::time::now();

        let rendering_tree = self.world.run_with_event(&self.root_component, event);
        crate::system::skia::request_draw_rendering_tree(rendering_tree);

        let elapsed = crate::time::now() - now;
        if elapsed > 33.ms() {
            println!("Warning: Rendering took {elapsed:?}. Keep it short as possible.",);
        }

        self.render_time_sum += elapsed;
        self.render_time_worst = self.render_time_worst.max(elapsed);

        if self.one_sec_timer.elapsed() > std::time::Duration::from_secs(1) {
            let mut text = format!(
                "{:?} rps {} / avg {:?} / worst {:?}",
                SystemTime::now(),
                self.one_sec_render_count,
                self.render_time_sum / self.one_sec_render_count,
                self.render_time_worst,
            );
            for (event_type, count) in &mut self.event_type_count {
                if *count > 0 {
                    text += &format!(" / {:?}: {}", event_type, count);
                    *count = 0;
                }
            }
            println!("\u{0081}{}", text);
            self.one_sec_render_count = 0;
            self.one_sec_timer = std::time::Instant::now();
            self.render_time_sum = 0.ms();
            self.render_time_worst = 0.ms();
        }
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
    ScreenRedraw,
    TextInput,
    TextInputKeyDown,
    TextInputSelectionChange,
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
            RawEvent::TextInput { .. } => EventType::TextInput,
            RawEvent::TextInputKeyDown { .. } => EventType::TextInputKeyDown,
            RawEvent::TextInputSelectionChange { .. } => EventType::TextInputSelectionChange,
        }
    }
}
