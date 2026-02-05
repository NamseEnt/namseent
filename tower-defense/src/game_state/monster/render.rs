use crate::game_state::{
    GameState, MonsterKind, TILE_PX_SIZE,
    monster::{MONSTER_HP_BAR_HEIGHT, Monster, monster_hp_bar::MonsterHpBar},
};
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
        | MonsterKind::Boss11 => TILE_PX_SIZE * 1.2,
        _ => TILE_PX_SIZE * 0.8,
    }
}

pub fn monster_animation_tick(game_state: &mut GameState, dt: Duration) {
    // STIFFNESS represents the spring constant in the physics simulation.
    // A negative value is used to simulate a restoring force that pulls the tower back to its equilibrium position.
    const STIFFNESS: f32 = 750.0;

    // DAMPING represents the damping coefficient, which reduces oscillations over time.
    // A negative value is used to simulate a force opposing the velocity of the tower's animation.
    const DAMPING: f32 = -5.0;

    const GRAVITY: f32 = 10.0;

    game_state.monsters.iter_mut().for_each(|monster| {
        let target_rotation = match monster.animation.rotated_side {
            MonsterAnimationRotatedSide::Left => (-15.0).deg(),
            MonsterAnimationRotatedSide::Right => 15.0.deg(),
        };
        let rotation_difference = target_rotation - monster.animation.rotation;
        let rotation_acceleration = STIFFNESS * rotation_difference.as_degrees()
            + DAMPING * monster.animation.rotation_velocity;
        monster.animation.rotation_velocity += rotation_acceleration * dt.as_secs_f32();
        monster.animation.rotation +=
            (monster.animation.rotation_velocity * dt.as_secs_f32()).deg();

        monster.animation.y_offset_velocity += GRAVITY * dt.as_secs_f32();
        monster.animation.y_offset += monster.animation.y_offset_velocity * dt.as_secs_f32();

        if monster.animation.y_offset >= 0.0 {
            monster.animation.y_offset = 0.0;
            let movement_speed =
                monster.move_on_route.velocity() * 1.sec() * monster.get_speed_multiplier();

            monster.animation.y_offset_velocity =
                (-3.25 + ((movement_speed - 1.5) / (4.0 - 1.5)) * (-1.5)).clamp(-3.25, -1.75);
            monster.animation.rotated_side = match monster.animation.rotated_side {
                MonsterAnimationRotatedSide::Left => MonsterAnimationRotatedSide::Right,
                MonsterAnimationRotatedSide::Right => MonsterAnimationRotatedSide::Left,
            };
        }
    });
}

#[derive(State, Clone)]
pub struct MonsterAnimation {
    pub rotation: Angle,
    rotation_velocity: f32,
    pub y_offset: f32,
    y_offset_velocity: f32,
    rotated_side: MonsterAnimationRotatedSide,
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
            rotated_side: MonsterAnimationRotatedSide::Left,
        }
    }
}

#[derive(State, Clone)]
enum MonsterAnimationRotatedSide {
    Left,
    Right,
}
