use crate::game_state::{
    GameEffectEvent, GameState, MonsterKind, TILE_PX_SIZE,
    monster::{MONSTER_HP_BAR_HEIGHT, Monster, monster_hp_bar::MonsterHpBar},
};
use crate::sound;
use namui::*;

impl Component for &Monster {
    fn render(self, ctx: &RenderCtx) {
        let Monster {
            kind, animation, ..
        } = self;

        let image = kind.image();
        let monster_wh = monster_wh(*kind);

        ctx.translate(Xy::new(
            TILE_PX_SIZE.width * 0.5,
            TILE_PX_SIZE.height - monster_wh.height * 0.5
                + TILE_PX_SIZE.height * animation.y_offset,
        ))
        .rotate(animation.rotation)
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
            progress: self.hp / self.max_hp,
        });
    }
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

    // DAMPING represents the damping coefficient, which reduces oscillations over time.
    // A negative value is used to simulate a force opposing the velocity of the tower's animation.
    const DAMPING: f32 = -5.0;

    const GRAVITY: f32 = 10.0;

    game_state.monsters.iter_mut().for_each(|monster| {
        monster.animation.y_offset_velocity += GRAVITY * dt.as_secs_f32();
        monster.animation.y_offset += monster.animation.y_offset_velocity * dt.as_secs_f32();

        if monster.animation.y_offset >= 0.0 {
            monster.animation.y_offset = 0.0;
            game_state.effect_events.push(GameEffectEvent::PlaySound(
                sound::EmitSoundParams::one_shot(
                    sound::random_cloth_footstep(),
                    sound::SoundGroup::Sfx,
                    sound::VolumePreset::Minimum,
                    sound::SpatialMode::NonSpatial,
                ),
            ));
            let movement_speed =
                monster.move_on_route.velocity() * 1.sec() * monster.get_speed_multiplier();

            monster.animation.y_offset_velocity =
                (-3.0 + ((movement_speed - 1.0) / (0.25)) * 0.4).clamp(-3.5, -1.85);
            monster.animation.next_descending_left = !monster.animation.next_descending_left;
        }

        let target_rotation = if monster.animation.y_offset_velocity < 0.0 {
            0.0.deg()
        } else if monster.animation.next_descending_left {
            (-10.0).deg()
        } else {
            10.0.deg()
        };
        let rotation_difference = target_rotation - monster.animation.rotation;
        let rotation_acceleration = STIFFNESS * rotation_difference.as_degrees()
            + DAMPING * monster.animation.rotation_velocity;
        monster.animation.rotation_velocity += rotation_acceleration * dt.as_secs_f32();
        monster.animation.rotation +=
            (monster.animation.rotation_velocity * dt.as_secs_f32()).deg();
    });
}

#[derive(State, Clone)]
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
