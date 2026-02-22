use crate::MapCoordF32;
use crate::game_state::field_particle::ICONS;
use crate::game_state::field_particle::particle::{IconParticle, IconParticleBehavior};
use crate::game_state::monster::MonsterStatusEffectKind;
use crate::icon::{Icon, IconAttribute, IconAttributePosition, IconKind, IconSize};
use namui::*;
use rand::Rng;

const MONSTER_DEBUFF_ICON_SIZE: f32 = 24.0;
const MONSTER_DEBUFF_FADE_DURATION_MS: i64 = 3000;
const MONSTER_DEBUFF_MIN_SPEED: f32 = 6.0;
const MONSTER_DEBUFF_MAX_SPEED: f32 = 12.0;
const MONSTER_DEBUFF_INITIAL_OPACITY: f32 = 0.9;

const MIN_INSTANT_PARTICLE_COUNT: usize = 1;
const MAX_INSTANT_PARTICLE_COUNT: usize = 2;

pub fn spawn_monster_status_effect_icons(
    now: Instant,
    monster_xy: MapCoordF32,
    debuff_kind: MonsterStatusEffectKind,
) {
    let mut rng = rand::thread_rng();
    let particle_count = rng.gen_range(MIN_INSTANT_PARTICLE_COUNT..=MAX_INSTANT_PARTICLE_COUNT);

    let debuff_icon = create_monster_debuff_icon(debuff_kind);

    for _ in 0..particle_count {
        let xy =
            monster_xy + MapCoordF32::new(rng.gen_range(0.25..=0.75), rng.gen_range(0.25..=0.75));
        let position = map_coord_to_pixel_f32(xy);

        let behavior = IconParticleBehavior::FadeRise {
            duration: Duration::from_millis(MONSTER_DEBUFF_FADE_DURATION_MS),
            speed: rng.gen_range(MONSTER_DEBUFF_MIN_SPEED..=MONSTER_DEBUFF_MAX_SPEED),
            created_at: now,
            initial_opacity: MONSTER_DEBUFF_INITIAL_OPACITY,
        };

        let icon_particle = IconParticle {
            icon: debuff_icon.clone(),
            xy: Xy::new(px(position.x), px(position.y)),
            rotation: 0.0.deg(),
            behavior,
        };

        ICONS.spawn(icon_particle);
    }
}

fn create_monster_debuff_icon(debuff_kind: MonsterStatusEffectKind) -> Icon {
    let (icon_kind, attribute_icon) = match &debuff_kind {
        MonsterStatusEffectKind::SpeedMul { mul } => {
            if *mul < 1.0 {
                (IconKind::MoveSpeed, Some(IconKind::Down))
            } else {
                (IconKind::MoveSpeed, Some(IconKind::Up))
            }
        }
        MonsterStatusEffectKind::Invincible => (IconKind::Invincible, None),
        MonsterStatusEffectKind::ImmuneToSlow => (IconKind::Shield, None),
    };

    Icon {
        kind: icon_kind,
        size: IconSize::Custom {
            size: px(MONSTER_DEBUFF_ICON_SIZE),
        },
        attributes: if let Some(attr_icon) = attribute_icon {
            vec![IconAttribute {
                icon_kind: attr_icon,
                position: IconAttributePosition::TopRight,
            }]
        } else {
            vec![]
        },
        wh: Wh::single(px(MONSTER_DEBUFF_ICON_SIZE)),
        opacity: MONSTER_DEBUFF_INITIAL_OPACITY,
    }
}

fn map_coord_to_pixel_f32(coord: MapCoordF32) -> Xy<f32> {
    let tile_size = crate::game_state::TILE_PX_SIZE;
    let pixel = tile_size.to_xy() * coord;
    Xy {
        x: pixel.x.as_f32(),
        y: pixel.y.as_f32(),
    }
}
