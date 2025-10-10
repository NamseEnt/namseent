use super::{Tower, TowerKind};
use crate::{asset_loader::get_tower_asset, game_state::GameState};
use namui::*;

impl Component for &Tower {
    fn render(self, ctx: &RenderCtx) {
        let image = get_tower_asset((self.kind, self.animation.kind));

        if let Some(image) = image {
            let image_wh = image.info().wh();
            let scale = Xy::new(
                1.0 + self.animation.y_ratio_offset * -0.5,
                1.0 + self.animation.y_ratio_offset,
            );
            ctx.translate((image_wh.width * 0.5, image_wh.height))
                .scale(scale)
                .add(namui::image(ImageParam {
                    rect: Rect::from_xy_wh(
                        Xy::new(-image_wh.width * 0.5, -image_wh.height),
                        image.info().wh(),
                    ),
                    image,
                    style: ImageStyle {
                        fit: ImageFit::None,
                        paint: None,
                    },
                }));
        }
    }
}

pub fn tower_animation_tick(game_state: &mut GameState, now: Instant) {
    // STIFFNESS represents the spring constant in the physics simulation.
    // A negative value is used to simulate a restoring force that pulls the tower back to its equilibrium position.
    const STIFFNESS: f32 = -1500.0;

    // DAMPING represents the damping coefficient, which reduces oscillations over time.
    // A negative value is used to simulate a force opposing the velocity of the tower's animation.
    const DAMPING: f32 = -10.0;

    game_state.towers.iter_mut().for_each(|tower| {
        let Tower {
            animation,
            template,
            ..
        } = tower;
        let kind = template.kind;
        if let TowerKind::Barricade = kind {
            return;
        }

        let delta_time = (now - animation.tick_at).as_secs_f32();
        animation.tick_at = now;

        if now - animation.transited_at > animation.duration() {
            animation.transition(
                match animation.kind {
                    AnimationKind::Idle1 => AnimationKind::Idle2,
                    AnimationKind::Idle2 => AnimationKind::Idle1,
                    AnimationKind::Attack => AnimationKind::Idle1,
                },
                now,
            );
        }

        let transit_force_expired = animation
            .transit_force
            .is_some_and(|transit_force| transit_force.end_at < now);
        let transit_force = animation
            .transit_force
            .map(|transit_force| transit_force.force)
            .unwrap_or(0.0);
        let spring_force = STIFFNESS * animation.y_ratio_offset;
        let damping_force = DAMPING * animation.y_ratio_velocity;
        let acceleration = spring_force + damping_force + transit_force;
        animation.y_ratio_velocity += acceleration * delta_time;
        animation.y_ratio_offset += animation.y_ratio_velocity * delta_time;

        if transit_force_expired {
            animation.transit_force = None;
        }
    });
}

#[derive(Clone, PartialEq, State)]
pub(super) struct Animation {
    kind: AnimationKind,
    transited_at: Instant,
    transit_force: Option<TransitForce>,
    tick_at: Instant,
    y_ratio_offset: f32,
    y_ratio_velocity: f32,
}

impl Animation {
    pub(super) fn new(now: Instant) -> Self {
        Self {
            kind: AnimationKind::Idle1,
            transited_at: now,
            transit_force: None,
            tick_at: now,
            y_ratio_offset: 0.0,
            y_ratio_velocity: 0.0,
        }
    }

    pub(super) fn transition(&mut self, kind: AnimationKind, now: Instant) {
        const IDLE_TRANSIT_FORCE: f32 = -100.0;
        const ATTACK_TRANSIT_FORCE: f32 = -500.0;
        const FORCE_DURATION: Duration = Duration::from_millis(33);

        if let AnimationKind::Attack = kind {
            self.transit_force = Some(TransitForce {
                force: ATTACK_TRANSIT_FORCE,
                end_at: now + FORCE_DURATION,
            });
        } else if let AnimationKind::Attack = self.kind {
            // Ignore transit force for transition from attack to idle
        } else {
            self.transit_force = Some(TransitForce {
                force: IDLE_TRANSIT_FORCE,
                end_at: now + FORCE_DURATION,
            });
        }

        self.kind = kind;
        self.transited_at = now;
    }

    fn duration(&self) -> Duration {
        self.kind.duration()
    }
}

#[derive(Clone, Copy, PartialEq, State)]
struct TransitForce {
    force: f32,
    end_at: Instant,
}

#[derive(Clone, Copy, PartialEq, State)]
pub enum AnimationKind {
    Idle1,
    Idle2,
    Attack,
}
impl AnimationKind {
    pub fn asset_id(&self) -> &str {
        match self {
            Self::Idle1 => "idle1",
            Self::Idle2 => "idle2",
            Self::Attack => "attack",
        }
    }
    fn duration(&self) -> Duration {
        match self {
            Self::Idle1 => Duration::from_millis(1500),
            Self::Idle2 => Duration::from_millis(1500),
            Self::Attack => Duration::from_millis(333),
        }
    }
}
