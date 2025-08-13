use namui::*;
use namui_prebuilt::*;

pub fn main() {
    namui::start(game);
}

fn game(ctx: &RenderCtx) {
    let screen_wh = screen::size().into_type::<Px>();
    let (parrying_fsm, set_parrying_fsm) = ctx.state(|| ParryingFsm::Idle);

    ctx.interval("parrying", 3.sec(), |_dt| {
        if parrying_fsm.as_ref() != &ParryingFsm::Idle {
            return;
        }
        set_parrying_fsm.set(ParryingFsm::Signal { at: Instant::now() });
    });

    ctx.on_raw_event(|event| match parrying_fsm.as_ref() {
        ParryingFsm::Idle => return,
        ParryingFsm::Signal { at } => {
            let now = Instant::now();
            let dt = now - at;
            let is_out_of_time = dt > 0.5.sec();
            if is_out_of_time {
                set_parrying_fsm.set(ParryingFsm::Idle);
                return;
            }
            if let RawEvent::KeyDown { event } = event
                && event.code == Code::ArrowUp
            {
                set_parrying_fsm.set(ParryingFsm::Combo {
                    start_at: now,
                    index: 0,
                });
            }
        }
        ParryingFsm::Combo { start_at, index } => {
            let time_each_combo = 0.5.sec();
            let now = Instant::now();
            let dt = now - start_at;
            let is_out_of_time = dt > time_each_combo * 4;
            if is_out_of_time {
                set_parrying_fsm.set(ParryingFsm::Idle);
                return;
            }
            let RawEvent::KeyDown { event } = event else {
                return;
            };
            if ![Code::ArrowLeft, Code::ArrowRight].contains(&event.code) {
                return;
            }
            set_parrying_fsm.set(if index + 1 >= 4 {
                ParryingFsm::Idle
            } else {
                ParryingFsm::Combo {
                    start_at: now,
                    index: index + 1,
                }
            });
        }
    });

    ctx.add(namui_prebuilt::typography::center_text(
        screen_wh,
        match parrying_fsm.as_ref() {
            ParryingFsm::Idle => "Idle".to_string(),
            ParryingFsm::Signal { at: _ } => "Signal".to_string(),
            ParryingFsm::Combo { start_at: _, index } => format!("Combo {index}"),
        },
        Color::WHITE,
        100.int_px(),
    ));

    ctx.add(simple_rect(
        screen_wh,
        Color::TRANSPARENT,
        0.px(),
        Color::BLACK,
    ));
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ParryingFsm {
    Idle,
    Signal { at: Instant },
    Combo { start_at: Instant, index: usize },
}
