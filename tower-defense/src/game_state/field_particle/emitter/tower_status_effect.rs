use crate::MapCoordF32;
use crate::game_state::TILE_PX_SIZE;
use crate::game_state::field_particle::ICONS;
use crate::game_state::field_particle::particle::{IconParticle, IconParticleBehavior};
use crate::game_state::tower::TowerStatusEffectKind;
use crate::icon::{Icon, IconAttribute, IconAttributePosition, IconKind, IconSize};
use namui::*;
use rand::Rng;

const TOWER_BUFF_ICON_SIZE: f32 = 36.0;
const TOWER_BUFF_FADE_DURATION_MS: i64 = 4000;
const TOWER_BUFF_MIN_SPEED: f32 = 8.0;
const TOWER_BUFF_MAX_SPEED: f32 = 15.0;
const TOWER_BUFF_INITIAL_OPACITY: f32 = 0.8;

const MIN_INSTANT_PARTICLE_COUNT: usize = 1;
const MAX_INSTANT_PARTICLE_COUNT: usize = 2;

pub fn spawn_tower_status_effect_icons(
    now: Instant,
    tower_xy: MapCoordF32,
    buff_kind: TowerStatusEffectKind,
) {
    let mut rng = rand::thread_rng();
    let particle_count = rng.gen_range(MIN_INSTANT_PARTICLE_COUNT..=MAX_INSTANT_PARTICLE_COUNT);

    let buff_icon = create_tower_buff_icon(buff_kind);
    let tower_pixel = map_coord_to_pixel_f32(tower_xy);

    for _ in 0..particle_count {
        let offset_range = 0.75;
        let offset_x = TILE_PX_SIZE.width.as_f32() * rng.gen_range(-offset_range..=offset_range);
        let offset_y = TILE_PX_SIZE.height.as_f32() * rng.gen_range(-offset_range..=offset_range);

        let position = Xy {
            x: tower_pixel.x + offset_x,
            y: tower_pixel.y + offset_y,
        };

        let behavior = IconParticleBehavior::FadeRise {
            duration: Duration::from_millis(TOWER_BUFF_FADE_DURATION_MS),
            speed: rng.gen_range(TOWER_BUFF_MIN_SPEED..=TOWER_BUFF_MAX_SPEED),
            created_at: now,
            initial_opacity: TOWER_BUFF_INITIAL_OPACITY,
        };

        let icon_particle = IconParticle {
            icon: buff_icon.clone(),
            xy: Xy::new(px(position.x), px(position.y)),
            rotation: 0.0.deg(),
            behavior,
        };

        ICONS.spawn(icon_particle);
    }
}

fn create_tower_buff_icon(buff_kind: TowerStatusEffectKind) -> Icon {
    let (icon_kind, attribute_icon) = match buff_kind {
        TowerStatusEffectKind::DamageAdd { .. } | TowerStatusEffectKind::DamageMul { .. } => {
            (IconKind::Damage, IconKind::Up)
        }
    };

    Icon {
        kind: icon_kind,
        size: IconSize::Custom {
            size: px(TOWER_BUFF_ICON_SIZE),
        },
        attributes: vec![IconAttribute {
            icon_kind: attribute_icon,
            position: IconAttributePosition::BottomRight,
        }],
        wh: Wh::single(px(TOWER_BUFF_ICON_SIZE)),
        opacity: TOWER_BUFF_INITIAL_OPACITY,
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
