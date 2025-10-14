use crate::MapCoordF32;
use crate::game_state::field_particle::{FieldParticle, particle::MonsterSpiritParticle};
use namui::*;
use rand::Rng;
use std::f32::consts::PI;

const SOUL_PARTICLE_DURATION_MS: i64 = 2000;
const SOUL_PARTICLE_SPEED_RANGE: (f32, f32) = (256.0, 512.0);
const SOUL_PARTICLE_SIZE: Px = px(128.0);
const SOUL_ANGLE_RANGE: f32 = PI / 6.0; // 30 degrees in radians

pub struct MonsterSpiritParticleEmitter {
    monster_xy: MapCoordF32,
    has_emitted: bool,
}

impl MonsterSpiritParticleEmitter {
    pub fn new(monster_xy: MapCoordF32) -> Self {
        Self {
            monster_xy,
            has_emitted: false,
        }
    }

    fn map_coord_to_pixel_f32(&self, coord: MapCoordF32) -> Xy<f32> {
        let tile_size = crate::game_state::TILE_PX_SIZE;
        let pixel = tile_size.to_xy() * coord;
        // Add center offset to spawn from monster center
        let center_offset = tile_size.to_xy() * 0.5;
        Xy {
            x: (pixel.x + center_offset.x).as_f32(),
            y: (pixel.y + center_offset.y).as_f32(),
        }
    }

    fn create_soul_particle(&self, now: Instant) -> FieldParticle {
        let mut rng = rand::thread_rng();

        let position = self.map_coord_to_pixel_f32(self.monster_xy);
        let angle = rng.gen_range(-SOUL_ANGLE_RANGE..=SOUL_ANGLE_RANGE).rad();
        let speed = rng.gen_range(SOUL_PARTICLE_SPEED_RANGE.0..=SOUL_PARTICLE_SPEED_RANGE.1);

        let spirit_particle = MonsterSpiritParticle::new(
            Xy::new(px(position.x), px(position.y)),
            angle,
            speed,
            SOUL_PARTICLE_SIZE,
            Duration::from_millis(SOUL_PARTICLE_DURATION_MS),
            now,
        );

        FieldParticle::MonsterSpirit {
            particle: spirit_particle,
        }
    }
    pub fn emit(&mut self, now: Instant, _dt: Duration) -> Vec<FieldParticle> {
        if self.has_emitted {
            return vec![];
        }

        let particle = self.create_soul_particle(now);

        self.has_emitted = true;
        vec![particle]
    }

    pub fn is_done(&self, _now: Instant) -> bool {
        self.has_emitted
    }
}
