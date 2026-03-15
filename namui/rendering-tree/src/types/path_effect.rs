use crate::*;
use std::hash::Hash;

#[derive(Debug, Clone, State)]
pub enum PathEffect {
    Dash { on: f32, off: f32, phase: f32 },
}

impl Hash for PathEffect {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        match self {
            PathEffect::Dash { on, off, phase } => {
                on.to_bits().hash(state);
                off.to_bits().hash(state);
                phase.to_bits().hash(state);
            }
        }
    }
}

impl PartialEq for PathEffect {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (
                PathEffect::Dash {
                    on: on1,
                    off: off1,
                    phase: phase1,
                },
                PathEffect::Dash {
                    on: on2,
                    off: off2,
                    phase: phase2,
                },
            ) => {
                on1.to_bits() == on2.to_bits()
                    && off1.to_bits() == off2.to_bits()
                    && phase1.to_bits() == phase2.to_bits()
            }
        }
    }
}

impl Eq for PathEffect {}
