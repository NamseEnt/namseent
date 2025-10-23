use crate::{MapCoordF32, game_state::field_particle::FieldParticle};
use namui::*;
use rand::{Rng, thread_rng};

#[derive(State)]
pub struct MonsterDeathEmitter {
    monster_xy: MapCoordF32,
    emitted: bool,
}

impl MonsterDeathEmitter {
    pub fn new(monster_xy: MapCoordF32) -> Self {
        Self {
            monster_xy,
            emitted: false,
        }
    }

    fn map_coord_to_pixel(&self, coord: MapCoordF32) -> Xy<Px> {
        let tile_size = crate::game_state::TILE_PX_SIZE;
        tile_size.to_xy() * coord
    }
}

impl namui::particle::Emitter<FieldParticle> for MonsterDeathEmitter {
    fn emit(&mut self, now: Instant, _dt: Duration) -> Vec<FieldParticle> {
        if self.emitted {
            return vec![];
        }

        let xy = self.map_coord_to_pixel(self.monster_xy + MapCoordF32::new(0.5, 0.5));
        let rotation = thread_rng().gen_range(-15.0..15.0).deg();

        let particle = crate::game_state::field_particle::particle::MonsterDeathParticle::new(
            xy, now, rotation,
        );

        self.emitted = true;
        vec![FieldParticle::MonsterDeath { particle }]
    }

    fn is_done(&self, _now: Instant) -> bool {
        self.emitted
    }
}
