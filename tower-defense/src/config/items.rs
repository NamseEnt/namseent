use namui::*;

#[cfg_attr(feature = "simulator", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Debug, State)]
pub struct ItemCandidateConfig {
    pub weight: f32,
    pub min_value: f32,
    pub max_value: f32,
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
        heal: ItemCandidateConfig {
            weight: 100.0,
            min_value: 10.0,
            max_value: 14.0,
        },
        extra_reroll: ItemCandidateConfig {
            weight: 10.0,
            min_value: 0.0,
            max_value: 0.0,
        },
        shield: ItemCandidateConfig {
            weight: 10.0,
            min_value: 15.0,
            max_value: 25.0,
        },
        damage_reduction: ItemCandidateConfig {
            weight: 10.0,
            min_value: 0.8,
            max_value: 0.85,
        },
        grant_barricades: ItemCandidateConfig {
            weight: 45.0,
            min_value: 5.0,
            max_value: 10.0,
        },
        grant_card: ItemCandidateConfig {
            weight: 35.0,
            min_value: 0.0,
            max_value: 0.0,
        },
    }
}
