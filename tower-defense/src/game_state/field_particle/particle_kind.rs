#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum ParticleKind {
    MonsterSpirit,
}

impl ParticleKind {
    pub fn resource_location(&self) -> &'static str {
        match self {
            ParticleKind::MonsterSpirit => "asset/image/particle/monster_spirit.png",
        }
    }
}
