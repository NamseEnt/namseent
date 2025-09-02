use crate::game_state::effect::Effect;

pub struct EffectText<'a> {
    effect: &'a Effect,
}

impl<'a> EffectText<'a> {
    pub fn new(effect: &'a Effect) -> Self {
        Self { effect }
    }

    pub fn to_korean(&self) -> String {
        match self.effect {
            Effect::Dummy => "TODO".to_string(),
        }
    }

    pub fn to_english(&self) -> String {
        match self.effect {
            Effect::Dummy => "TODO".to_string(),
        }
    }
}
