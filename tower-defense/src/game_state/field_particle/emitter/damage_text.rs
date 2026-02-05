use crate::MapCoordF32;
use crate::game_state::field_particle::DamageTextParticle;
use namui::*;

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

        let particle = DamageTextParticle::new(self.monster_xy, self.damage, now);

        self.emitted = true;
        vec![crate::game_state::field_particle::FieldParticle::DamageText { particle }]
    }
    fn is_done(&self, _now: Instant) -> bool {
        self.emitted
    }
}
