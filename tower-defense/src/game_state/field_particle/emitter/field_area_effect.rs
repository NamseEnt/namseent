use crate::game_state::{
    field_area_effect::{FieldAreaEffectEnd, FieldAreaEffectKind},
    field_particle::{
        FieldParticle,
        emitter::EmitSchedule,
        particle::{FieldAreaParticleShape, FieldDamageAreaParticle},
    },
};
use namui::*;

pub struct FieldAreaEffectEmitter {
    kind: FieldAreaEffectKind,
    emit_schedule: EmitSchedule,
}
impl FieldAreaEffectEmitter {
    pub fn new(now: Instant, kind: FieldAreaEffectKind, end_at: FieldAreaEffectEnd) -> Self {
        let emit_interval = match kind {
            FieldAreaEffectKind::RoundDamage { .. } => Duration::from_millis(100),
            FieldAreaEffectKind::RoundDamageOverTime { tick_interval, .. } => tick_interval,
            FieldAreaEffectKind::LinearDamage { .. } => Duration::from_millis(100),
            FieldAreaEffectKind::LinearDamageOverTime { tick_interval, .. } => tick_interval,
        };
        let emit_count = match end_at {
            FieldAreaEffectEnd::AtTime { end_at } => {
                let duration = end_at - now;
                (duration.as_millis() / emit_interval.as_millis()) as usize
            }
            FieldAreaEffectEnd::Once { .. } => 1,
        };
        let emit_schedule = EmitSchedule::new(emit_interval, emit_count, now);
        Self {
            kind,
            emit_schedule,
        }
    }
    pub fn emit(&mut self, now: Instant, _dt: Duration) -> Vec<FieldParticle> {
        if !self.emit_schedule.try_emit(now) {
            return vec![];
        }

        match self.kind {
            FieldAreaEffectKind::RoundDamage {
                rank: _,
                suit: _,
                damage: _,
                xy,
                radius,
            } => {
                let shape = FieldAreaParticleShape::Circle { center: xy, radius };
                let particle = FieldDamageAreaParticle::new(now, shape);
                vec![FieldParticle::FieldDamageArea { particle }]
            }
            FieldAreaEffectKind::RoundDamageOverTime {
                rank: _,
                suit: _,
                damage_per_tick: _,
                xy,
                radius,
                tick_interval: _,
                next_tick_at: _,
            } => {
                let shape = FieldAreaParticleShape::Circle { center: xy, radius };
                let particle = FieldDamageAreaParticle::new(now, shape);
                vec![FieldParticle::FieldDamageArea { particle }]
            }
            FieldAreaEffectKind::LinearDamage {
                rank: _,
                suit: _,
                damage: _,
                center_xy,
                target_xy,
                thickness,
            } => {
                let shape = FieldAreaParticleShape::Line {
                    start: center_xy,
                    end: target_xy,
                    thickness,
                };
                let particle = FieldDamageAreaParticle::new(now, shape);
                vec![FieldParticle::FieldDamageArea { particle }]
            }
            FieldAreaEffectKind::LinearDamageOverTime {
                rank: _,
                suit: _,
                damage_per_tick: _,
                center_xy,
                target_xy,
                thickness,
                tick_interval: _,
                next_tick_at: _,
            } => {
                let shape = FieldAreaParticleShape::Line {
                    start: center_xy,
                    end: target_xy,
                    thickness,
                };
                let particle = FieldDamageAreaParticle::new(now, shape);
                vec![FieldParticle::FieldDamageArea { particle }]
            }
        }
    }

    pub fn is_done(&self, now: Instant) -> bool {
        self.emit_schedule.is_done(now)
    }
}
