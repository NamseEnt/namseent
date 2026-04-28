use namui::*;

#[cfg_attr(feature = "simulator", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Debug, State)]
pub struct ItemCandidateConfig {
    pub weight: f32,
}

#[cfg_attr(feature = "simulator", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Debug, State)]
pub struct ItemConfig {
    pub heal: ItemCandidateConfig,
    pub extra_reroll: ItemCandidateConfig,
    pub shield: ItemCandidateConfig,
    pub damage_reduction: ItemCandidateConfig,
    pub grant_barricades: ItemCandidateConfig,
    pub grant_card: ItemCandidateConfig,
}

pub fn default_item_config() -> ItemConfig {
    ItemConfig {
        heal: ItemCandidateConfig { weight: 100.0 },
        extra_reroll: ItemCandidateConfig { weight: 10.0 },
        shield: ItemCandidateConfig { weight: 10.0 },
        damage_reduction: ItemCandidateConfig { weight: 10.0 },
        grant_barricades: ItemCandidateConfig { weight: 45.0 },
        grant_card: ItemCandidateConfig { weight: 35.0 },
    }
}
