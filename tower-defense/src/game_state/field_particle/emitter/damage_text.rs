use crate::{MapCoordF32, game_state::field_particle::DamageTextParticle};
use namui::*;
use rand::Rng;

#[derive(State)]
pub struct DamageTextEmitter {
    monster_xy: MapCoordF32,
    damage: f32,
    emitted: bool,
}

impl DamageTextEmitter {
    pub fn new(monster_xy: MapCoordF32, damage: f32) -> Self {
        Self {
            monster_xy,
            damage,
            emitted: false,
        }
    }

    fn map_coord_to_pixel(&self, coord: MapCoordF32) -> Xy<Px> {
        let tile_size = crate::game_state::TILE_PX_SIZE;
        tile_size.to_xy() * coord
    }
}

impl namui::particle::Emitter<crate::game_state::field_particle::FieldParticle>
    for DamageTextEmitter
{
    fn emit(
        &mut self,
        now: Instant,
        _dt: Duration,
    ) -> Vec<crate::game_state::field_particle::FieldParticle> {
        if self.emitted {
            return vec![];
        }

        let mut rng = rand::thread_rng();
        let xy = self.monster_xy
            + MapCoordF32::new(rng.gen_range(0.25..=0.75), rng.gen_range(0.25..=0.75));
        let xy = self.map_coord_to_pixel(xy);

        let particle = DamageTextParticle::new(xy, self.damage, now);

        self.emitted = true;
        vec![crate::game_state::field_particle::FieldParticle::DamageText { particle }]
    }
    fn is_done(&self, _now: Instant) -> bool {
        self.emitted
    }
}
