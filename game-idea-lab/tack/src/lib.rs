use namui::*;
use namui_prebuilt::*;

pub fn main() {
    namui::start(game);
}

fn game(ctx: &RenderCtx) {
    let screen_wh = screen::size().into_type::<Px>();
    let now = Instant::now();

    let (motion_fsm, set_motion_fsm) = ctx.state(|| MotionFsm::Idle);
    let (first_tick, set_first_tick) = ctx.state(|| true);
    let (last_key, set_last_key) = ctx.state(|| None);
    let (monster_state, set_monster_state) = ctx.state(|| MonsterState::Idle);

    let parrying_time = 0.5.sec();
    let rollback_time = 0.3.sec();
    let monster_idle_xy = Xy::new(screen_wh.width / 2., screen_wh.height / 4.);
    let player_xy = Xy::new(screen_wh.width / 2., screen_wh.height * 3 / 4.);

    let monster_xy = {
        match monster_state.as_ref() {
            MonsterState::Idle => monster_idle_xy,
            MonsterState::Attacking { start_at } => {
                let velocity = Per::new(player_xy - monster_idle_xy, parrying_time);
                monster_idle_xy + velocity * (now - start_at)
            }
            MonsterState::Parried { start_xy, start_at } => {
                let velocity = Per::new(monster_idle_xy - start_xy, rollback_time);
                *start_xy + velocity * (now - start_at)
            }
        }
    };

    ctx.interval(
        "handle monster timeout",
        (60. / 1000.).sec(),
        |_dt| match monster_state.as_ref() {
            MonsterState::Attacking { start_at } => {
                if now - start_at > parrying_time {
                    set_monster_state.set(MonsterState::Idle);
                }
            }
            MonsterState::Parried { start_at, .. } => {
                if now - start_at > rollback_time {
                    set_monster_state.set(MonsterState::Idle);
                }
            }
            _ => {}
        },
    );

    ctx.interval("parrying", 3.sec(), |_dt| {
        if *first_tick {
            set_first_tick.set(false);
            return;
        }
        set_monster_state.set(MonsterState::Attacking { start_at: now });
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

        let MonsterState::Attacking { start_at } = monster_state.as_ref() else {
            return;
        };
        let is_up = event.code == Code::ArrowUp;
        let signal_dt = now - start_at;

        if !is_up && signal_dt < 0.5.sec() {
            // do nothing. give chance.
            return;
        }

        set_monster_state.set(MonsterState::Parried {
            start_xy: monster_xy,
            start_at: now,
        });

        let parrying_success = is_up && signal_dt < parrying_time;
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

        if let MonsterState::Attacking { .. } = monster_state.as_ref() {
            return;
        }

        let is_left_or_right = [Code::ArrowLeft, Code::ArrowRight].contains(&event.code);
        if !is_left_or_right {
            return;
        }

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
    let keyboard_text = match last_key.as_ref() {
        None => "_".to_string(),
        Some(code) => code.to_string(),
    };

    ctx.translate(monster_xy).add(simple_rect(
        Wh::single(10.px()),
        Color::TRANSPARENT,
        0.px(),
        match monster_state.as_ref() {
            MonsterState::Idle => Color::GREEN,
            MonsterState::Attacking { .. } => Color::RED,
            MonsterState::Parried { .. } => Color::from_f01(1., 1., 0., 1.),
        },
    ));

    ctx.add(namui_prebuilt::typography::center_text(
        screen_wh,
        format!("{motion_text}\n{keyboard_text}"),
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

enum MonsterState {
    Idle,
    Attacking { start_at: Instant },
    Parried { start_xy: Xy<Px>, start_at: Instant },
}
