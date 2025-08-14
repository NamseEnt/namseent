use namui::*;
use namui_prebuilt::*;

pub fn main() {
    namui::start(game);
}

fn game(ctx: &RenderCtx) {
    let screen_wh = screen::size().into_type::<Px>();
    let (motion_fsm, set_motion_fsm) = ctx.state(|| MotionFsm::Idle);
    let (parrying_signal, set_parrying_signal) = ctx.state::<Option<Instant>>(|| None);
    let (first_tick, set_first_tick) = ctx.state(|| true);
    let (last_key, set_last_key) = ctx.state(|| None);

    ctx.interval("parrying", 3.sec(), |_dt| {
        if *first_tick {
            set_first_tick.set(false);
            return;
        }
        if parrying_signal.is_some() {
            set_parrying_signal.set(None);
            set_motion_fsm.set(MotionFsm::Attacked {
                start_at: Instant::now(),
            });
            return;
        }
        set_parrying_signal.set(Some(Instant::now()));
    });

    ctx.on_raw_event(|event| {
        let RawEvent::KeyDown { event } = event else {
            return;
        };
        set_last_key.set(Some(event.code));
    });

    // handle parrying
    ctx.on_raw_event(|event| {
        let RawEvent::KeyDown { event } = event else {
            return;
        };

        let Some(parrying_signal) = parrying_signal.as_ref() else {
            return;
        };
        let is_up = event.code == Code::ArrowUp;
        let now = Instant::now();
        let signal_dt = now - parrying_signal;

        if !is_up && signal_dt < 0.5.sec() {
            // do nothing. give chance.
            return;
        }

        set_parrying_signal.set(None);

        let parrying_success = is_up && signal_dt < 1.sec();
        set_motion_fsm.set(if parrying_success {
            MotionFsm::Parring
        } else {
            MotionFsm::Attacked { start_at: now }
        });
    });

    // handle motion
    ctx.on_raw_event(|event| {
        let RawEvent::KeyDown { event } = event else {
            return;
        };

        if parrying_signal.is_some() {
            return;
        }

        let is_left_or_right = [Code::ArrowLeft, Code::ArrowRight].contains(&event.code);
        if !is_left_or_right {
            return;
        }

        let now = Instant::now();

        match motion_fsm.as_ref() {
            MotionFsm::Idle => set_motion_fsm.set(MotionFsm::BasicAttack1),
            MotionFsm::Attacked { start_at } => {
                if now - start_at < 1.sec() {
                    return;
                }
                set_motion_fsm.set(MotionFsm::Idle)
            }
            MotionFsm::BasicAttack1 => set_motion_fsm.set(MotionFsm::BasicAttack2),
            MotionFsm::BasicAttack2 => set_motion_fsm.set(MotionFsm::BasicAttack3),
            MotionFsm::BasicAttack3 => set_motion_fsm.set(MotionFsm::BasicAttack4),
            MotionFsm::BasicAttack4 => set_motion_fsm.set(MotionFsm::BasicAttack5),
            MotionFsm::BasicAttack5 => set_motion_fsm.set(MotionFsm::BasicAttack1),
            MotionFsm::Parring => set_motion_fsm.set(MotionFsm::PostParingCombo1),
            MotionFsm::PostParingCombo1 => set_motion_fsm.set(MotionFsm::PostParingCombo2),
            MotionFsm::PostParingCombo2 => set_motion_fsm.set(MotionFsm::PostParingCombo3),
            MotionFsm::PostParingCombo3 => set_motion_fsm.set(MotionFsm::PostParingCombo4),
            MotionFsm::PostParingCombo4 => set_motion_fsm.set(MotionFsm::BasicAttack1),
        }
    });

    let motion_text = match motion_fsm.as_ref() {
        MotionFsm::Idle => "Idle",
        MotionFsm::Attacked { start_at: _ } => "Attacked",
        MotionFsm::BasicAttack1 => "BasicAttack1",
        MotionFsm::BasicAttack2 => "BasicAttack2",
        MotionFsm::BasicAttack3 => "BasicAttack3",
        MotionFsm::BasicAttack4 => "BasicAttack4",
        MotionFsm::BasicAttack5 => "BasicAttack5",
        MotionFsm::Parring => "Parring",
        MotionFsm::PostParingCombo1 => "PostParingCombo1",
        MotionFsm::PostParingCombo2 => "PostParingCombo2",
        MotionFsm::PostParingCombo3 => "PostParingCombo3",
        MotionFsm::PostParingCombo4 => "PostParingCombo4",
    };
    let parrying_text = match parrying_signal.as_ref() {
        None => "_",
        Some(_) => "Parrying!!",
    };
    let keyboard_text = match last_key.as_ref() {
        None => "_".to_string(),
        Some(code) => code.to_string(),
    };
    ctx.add(namui_prebuilt::typography::center_text(
        screen_wh,
        format!("{}\n{}\n{}", motion_text, parrying_text, keyboard_text),
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

enum MotionFsm {
    Idle,
    Attacked { start_at: Instant },
    BasicAttack1,
    BasicAttack2,
    BasicAttack3,
    BasicAttack4,
    BasicAttack5,
    Parring,
    PostParingCombo1,
    PostParingCombo2,
    PostParingCombo3,
    PostParingCombo4,
}
