use crate::game_state::{
    GameState, MonsterKind, PresentationEvent, TILE_PX_SIZE,
    monster::{MONSTER_HP_BAR_HEIGHT, Monster, monster_hp_bar::MonsterHpBar},
};
use namui::*;

impl Component for &Monster {
    fn render(self, ctx: &RenderCtx) {
        render_monster_pose(
            ctx,
            RenderMonsterPose {
                kind: self.kind,
                hp: self.hp,
                max_hp: self.max_hp,
                rotation: self.animation.rotation,
                y_offset: self.animation.y_offset,
                hit_offset: Xy::new(0.0, 0.0),
                hit_flash: 0.0,
                camera_zoom_level: 1.0,
            },
        );
    }
}

pub(crate) struct RenderMonsterPose {
    pub(crate) kind: MonsterKind,
    pub(crate) hp: crate::Health,
    pub(crate) max_hp: crate::Health,
    pub(crate) rotation: Angle,
    pub(crate) y_offset: f32,
    pub(crate) hit_offset: Xy<f32>,
    pub(crate) hit_flash: f32,
    pub(crate) camera_zoom_level: f32,
}

impl Component for RenderMonsterPose {
    fn render(self, ctx: &RenderCtx) {
        render_monster_pose(ctx, self);
    }
}

fn render_monster_pose(ctx: &RenderCtx, pose: RenderMonsterPose) {
    let RenderMonsterPose {
        kind,
        hp,
        max_hp,
        rotation,
        y_offset,
        hit_offset,
        hit_flash,
        camera_zoom_level,
    } = pose;
    let image = kind.image();
    let monster_wh = monster_wh(kind);
    let flash_alpha = (hit_flash.clamp(0.0, 1.0) * 255.0).round() as u8;
    let image_ctx = ctx
        .translate(Xy::new(
            TILE_PX_SIZE.width * 0.5,
            TILE_PX_SIZE.height - monster_wh.height * 0.5 + TILE_PX_SIZE.height * y_offset,
        ))
        .rotate(rotation)
        .translate(Xy::new(
            px(hit_offset.x / camera_zoom_level),
            px(hit_offset.y / camera_zoom_level),
        ));
    let image_rect = Rect::from_xy_wh(monster_wh.to_xy() * -0.5, monster_wh);
    if flash_alpha > 0 {
        image_ctx.add(namui::image(ImageParam {
            rect: image_rect,
            image,
            style: ImageStyle {
                fit: ImageFit::Contain,
                paint: Some(
                    Paint::new(Color::WHITE).set_color_filter(ColorFilter::Blend {
                        color: Color::from_u8(255, 128, 128, flash_alpha),
                        blend_mode: BlendMode::SrcIn,
                    }),
                ),
            },
        }));
    }
    image_ctx.add(namui::image(ImageParam {
        rect: image_rect,
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
    const HIT_OFFSET_DAMPING: f32 = 18.0;
    const HIT_OFFSET_ANGULAR_SPEED: f32 = 42.0;
    const MAX_HIT_OFFSET_PX: f32 = 16.0;

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
        runtime.hit_elapsed_secs = (runtime.hit_elapsed_secs + dt.as_secs_f32())
            .min(crate::game_state::MONSTER_HIT_OFFSET_DURATION_SECS);
        if runtime.hit_elapsed_secs < crate::game_state::MONSTER_HIT_OFFSET_DURATION_SECS {
            let amplitude = MAX_HIT_OFFSET_PX
                * (-HIT_OFFSET_DAMPING * runtime.hit_elapsed_secs).exp()
                * (HIT_OFFSET_ANGULAR_SPEED * runtime.hit_elapsed_secs).cos();
            runtime.hit_offset = runtime.hit_direction * amplitude;
        } else {
            runtime.hit_offset = Xy::new(0.0, 0.0);
        }
        runtime.hit_flash = (1.0
            - runtime.hit_elapsed_secs / crate::game_state::MONSTER_HIT_FLASH_DURATION_SECS)
            .clamp(0.0, 1.0)
            .powi(2);
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
