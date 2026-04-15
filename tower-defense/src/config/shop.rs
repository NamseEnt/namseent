use namui::*;

#[cfg_attr(feature = "simulator", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Debug, State)]
pub struct ShopConfig {
    pub base_cost: f32,
    pub value_cost_multiplier: f32,
    /// Distribution: [item%, tower_upgrade%, extra_upgrade%] out of 10
    pub slot_type_distribution: [usize; 3],
}
