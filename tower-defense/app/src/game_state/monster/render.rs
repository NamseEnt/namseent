use crate::game_state::{
    GameState, MonsterKind, PresentationEvent, TILE_PX_SIZE,
    monster::{MONSTER_HP_BAR_HEIGHT, Monster, monster_hp_bar::MonsterHpBar},
};
use namui::*;

impl Component for &Monster {
    fn render(self, ctx: &RenderCtx) {
        render_monster_pose(
            ctx,
            self.kind,
            self.hp,
            self.max_hp,
            self.animation.rotation,
            self.animation.y_offset,
        );
    }
}

pub(crate) struct RenderMonsterPose {
    pub(crate) kind: MonsterKind,
    pub(crate) hp: crate::Health,
    pub(crate) max_hp: crate::Health,
    pub(crate) rotation: Angle,
    pub(crate) y_offset: f32,
}

impl Component for RenderMonsterPose {
    fn render(self, ctx: &RenderCtx) {
        render_monster_pose(
            ctx,
            self.kind,
            self.hp,
            self.max_hp,
            self.rotation,
            self.y_offset,
        );
    }
}

fn render_monster_pose(
    ctx: &RenderCtx,
    kind: MonsterKind,
    hp: crate::Health,
    max_hp: crate::Health,
    rotation: Angle,
    y_offset: f32,
) {
    let image = kind.image();
    let monster_wh = monster_wh(kind);

    ctx.translate(Xy::new(
        TILE_PX_SIZE.width * 0.5,
        TILE_PX_SIZE.height - monster_wh.height * 0.5 + TILE_PX_SIZE.height * y_offset,
    ))
    .rotate(rotation)
    .add(namui::image(ImageParam {
        rect: Rect::from_xy_wh(monster_wh.to_xy() * -0.5, monster_wh),
        image,
        style: ImageStyle {
            fit: ImageFit::Contain,
            paint: None,
        },
    }));

    let hp_bar_wh = Wh::new(monster_wh.width, MONSTER_HP_BAR_HEIGHT);
    ctx.translate(Xy::new(
        TILE_PX_SIZE.width * 0.5,
        TILE_PX_SIZE.width * 0.5 + monster_wh.height * 0.6,
    ))
    .add(MonsterHpBar {
        wh: hp_bar_wh,
        progress: if max_hp.is_zero() {
            0.0
        } else {
            hp.as_f32() / max_hp.as_f32()
        },
    });
}

pub fn monster_wh(kind: MonsterKind) -> Wh<Px> {
    match kind {
        MonsterKind::Boss01
        | MonsterKind::Boss02
        | MonsterKind::Boss03
        | MonsterKind::Boss04
        | MonsterKind::Boss05
        | MonsterKind::Boss06
        | MonsterKind::Boss07
        | MonsterKind::Boss08
        | MonsterKind::Boss09
        | MonsterKind::Boss10
        | MonsterKind::Boss11
        | MonsterKind::Boss12
        | MonsterKind::Boss13
        | MonsterKind::Boss14 => TILE_PX_SIZE * 1.4,
        _ => TILE_PX_SIZE * 0.9,
    }
}

pub fn monster_animation_tick(game_state: &mut GameState, dt: Duration) {
    // STIFFNESS represents the spring constant in the physics simulation.
    // A negative value is used to simulate a restoring force that pulls the tower back to its equilibrium position.
    const STIFFNESS: f32 = 350.0;

    const GRAVITY: f32 = 10.0;

    let raw_monsters = game_state.raw_core_state().monsters().to_vec();
    let presentation_events = &mut game_state.pending_presentation_events;
    for raw_monster in &raw_monsters {
        let Some(cache) = game_state
            .presentation_metadata
            .monsters
            .iter_mut()
            .find(|cache| cache.id.raw() == raw_monster.id)
        else {
            continue;
        };
        let Some(runtime) = game_state
            .monster_animation_runtime
            .iter_mut()
            .find(|runtime| runtime.id.raw() == raw_monster.id)
        else {
            continue;
        };
        runtime.y_offset_velocity += GRAVITY * dt.as_secs_f32();
        cache.y_offset += runtime.y_offset_velocity * dt.as_secs_f32();

        if cache.y_offset >= 0.0 {
            cache.y_offset = 0.0;
            presentation_events.push(PresentationEvent::PlaySoundCue {
                cue: crate::game_state::SoundCue::MonsterFootstep,
                position: None,
                volume: crate::game_state::SoundVolume::Minimum,
                max_duration_ms: None,
            });
            let speed_multiplier = crate::game_state::Monster::from_core_state(raw_monster.clone())
                .map(|monster| monster.get_speed_multiplier().as_f32())
                .unwrap_or(1.0);
            let movement_speed: f32 = raw_monster.move_on_route.velocity_raw as f32
                / crate::world::WORLD_UNITS_PER_TILE as f32
                * speed_multiplier;

            runtime.y_offset_velocity =
                (-3.0 + ((movement_speed - 1.0) / (0.25)) * 0.4).clamp(-3.5, -1.85);
            runtime.next_descending_left = !runtime.next_descending_left;
        }

        let target_rotation = if runtime.y_offset_velocity < 0.0 {
            0.0.deg()
        } else if runtime.next_descending_left {
            (-10.0).deg()
        } else {
            10.0.deg()
        };
        let rotation_difference = target_rotation - cache.rotation;
        let rotation_acceleration =
            STIFFNESS * rotation_difference.as_degrees() - 5.0 * runtime.rotation_velocity;
        runtime.rotation_velocity += rotation_acceleration * dt.as_secs_f32();
        cache.rotation += (runtime.rotation_velocity * dt.as_secs_f32()).deg();
    }
}

#[derive(State, Clone, PartialEq)]
pub struct MonsterAnimation {
    pub rotation: Angle,
    rotation_velocity: f32,
    pub y_offset: f32,
    y_offset_velocity: f32,
    next_descending_left: bool,
}
impl Default for MonsterAnimation {
    fn default() -> Self {
        Self::new()
    }
}

impl MonsterAnimation {
    pub fn new() -> Self {
        Self {
            rotation: 0.deg(),
            rotation_velocity: 0.0,
            y_offset: 0.0,
            y_offset_velocity: 0.0,
            next_descending_left: false,
        }
    }
}
