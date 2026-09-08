use crate::{FixedRatio, SimTick};
use namui::*;

#[derive(Debug, State, Clone)]
pub struct UserStatusEffect {
    pub kind: UserStatusEffectKind,
    pub end_at: SimTick,
}

#[derive(Debug, State, Clone)]
pub enum UserStatusEffectKind {
    DamageReduction { damage_multiply: FixedRatio },
}

impl UserStatusEffect {
    pub(crate) fn to_core_status_effect(&self) -> td_core::UserStatusEffect {
        td_core::UserStatusEffect {
            kind: match self.kind {
                UserStatusEffectKind::DamageReduction { damage_multiply } => {
                    td_core::UserStatusEffectKind::DamageReduction {
                        damage_multiply_raw: damage_multiply.raw(),
                    }
                }
            },
            end_at: self.end_at.ticks(),
        }
    }
}
