use namui::*;
use namui_prebuilt::*;

pub fn main() {
    namui::start(game);
}

fn game(ctx: &RenderCtx) {
    let screen_wh = screen::size().into_type::<Px>();
    let (scene, set_scene) = ctx.state(|| Scene::Ready);

    match scene.as_ref() {
        Scene::Ready => ctx.add((ready_scene, |scene| set_scene.set(scene))),
        Scene::Fight => ctx.add(fight_scene),
    };

    ctx.add(simple_rect(
        screen_wh,
        Color::TRANSPARENT,
        0.px(),
        Color::BLACK,
    ));
}

enum Scene {
    Ready,
    Fight,
}

fn ready_scene(ctx: &RenderCtx, transition: impl FnOnce(Scene)) {
    let screen_wh = screen::size().into_type::<Px>();
    ctx.add(namui_prebuilt::typography::center_text(
        screen_wh,
        "Press Enter to start",
        Color::WHITE,
        32.int_px(),
    ))
    .attach_event(|event| {
        let Event::KeyDown { event } = event else {
            return;
        };
        if Code::Enter == event.code {
            transition(Scene::Fight);
        }
    });
}

fn fight_scene(ctx: &RenderCtx) {
    struct State {
        monster: Monster,
        attack_motion_machine: AttackMotionMachine,
    }

    struct Monster {
        hp: f32,
    }

    let screen_wh = screen::size().into_type::<Px>();
    let (state, set_state) = ctx.state(|| State {
        monster: Monster { hp: 100. },
        attack_motion_machine: AttackMotionMachine::new(),
    });

    ctx.on_raw_event(|event| {
        let RawEvent::KeyDown { event } = event else {
            return;
        };
        if Code::ArrowLeft == event.code {
            set_state.mutate(|state| state.attack_motion_machine.push());
        }
    });

    ctx.interval("tick", 16.ms(), |dt| {
        set_state.mutate(move |state| {
            let attacked = state.attack_motion_machine.tick_and_attacked(dt);
            if attacked {
                state.monster.hp -= 1.;
            }
        });
    });

    let attack_motion_text = match &state.attack_motion_machine.phase {
        None => "_".to_string(),
        Some(phase) => format!("{}", phase.phase_number),
    };

    ctx.add(namui_prebuilt::typography::center_text(
        screen_wh,
        [
            format!("Monster HP: {}", state.monster.hp),
            format!("Attack Motion: {attack_motion_text}"),
            "Attack Key: '←'".to_string(),
        ]
        .join("\n"),
        Color::WHITE,
        32.int_px(),
    ));
}

struct AttackMotionMachine {
    phase: Option<Phase>,
    pushed: bool,
}

impl AttackMotionMachine {
    fn new() -> Self {
        Self {
            phase: None,
            pushed: false,
        }
    }
    fn push(&mut self) {
        self.pushed = true;
    }
    fn tick_and_attacked(&mut self, dt: Duration) -> bool {
        if let Some(phase) = self.phase.as_mut() {
            phase.delay_left -= dt;
            if phase.delay_left > Duration::ZERO {
                return false;
            }
            if !self.pushed {
                self.phase = None;
                return false;
            }
            self.pushed = false;
            phase.phase_number += 1;
            if phase.phase_number >= DELAY_DURATIONS.len() {
                phase.phase_number = 0;
            }
            phase.delay_left = DELAY_DURATIONS[phase.phase_number];

            return true;
        }

        if self.pushed {
            self.pushed = false;
            self.phase = Some(Phase {
                phase_number: 0,
                delay_left: DELAY_DURATIONS[0],
            });
            return true;
        }
        false
    }
}

struct Phase {
    phase_number: usize,
    delay_left: Duration,
}

const DELAY_DURATIONS: [Duration; 5] = [
    Duration::from_millis(500),
    Duration::from_millis(500),
    Duration::from_millis(250),
    Duration::from_millis(250),
    Duration::from_millis(500),
];
