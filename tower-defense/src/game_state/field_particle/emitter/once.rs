use crate::game_state::field_particle::FieldParticle;
use namui::*;

type EmitFn = dyn FnOnce(Instant, Duration) -> Vec<FieldParticle> + Send + Sync;

pub struct OnceEmitter {
    emit_fn: Option<Box<EmitFn>>,
}
impl OnceEmitter {
    pub fn new<F>(emit_fn: F) -> Self
    where
        F: FnOnce(Instant, Duration) -> Vec<FieldParticle> + Send + Sync + 'static,
    {
        Self {
            emit_fn: Some(Box::new(emit_fn)),
        }
    }

    pub fn emit(&mut self, now: Instant, dt: Duration) -> Vec<FieldParticle> {
        let Some(emit_fn) = self.emit_fn.take() else {
            return vec![];
        };
        emit_fn(now, dt)
    }

    pub fn is_done(&self, _now: Instant) -> bool {
        self.emit_fn.is_none()
    }
}
