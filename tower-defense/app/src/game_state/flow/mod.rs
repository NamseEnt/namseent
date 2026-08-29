use super::GameState;
use crate::{shop::Shop, *};

#[derive(Clone, Debug, State)]
#[allow(clippy::large_enum_variant)]
pub enum GameFlow {
    Initializing,
    Shopping(ShoppingFlow),
    SelectingTower(SelectingTowerFlow),
    PlacingTower,
    Defense(DefenseFlow),
    TreasureSelection(TreasureSelectionFlow),
    Result { clear_rate: ClearRate },
}

impl GameFlow {
    pub(crate) fn to_core_state(&self) -> td_core::GameFlowState {
        match self {
            Self::Initializing => td_core::GameFlowState::Initializing,
            Self::Shopping(flow) => td_core::GameFlowState::Shopping(flow.shop.to_core_state()),
            Self::SelectingTower(_) => td_core::GameFlowState::SelectingTower,
            Self::PlacingTower => td_core::GameFlowState::PlacingTower,
            Self::Defense(flow) => td_core::GameFlowState::Defense(flow.to_core_state()),
            Self::TreasureSelection(flow) => td_core::GameFlowState::TreasureSelection {
                options: flow
                    .options
                    .iter()
                    .copied()
                    .map(|upgrade| {
                        crate::game_state::upgrade::UpgradeWithId {
                            id: crate::game_state::upgrade::UpgradeId(0),
                            upgrade,
                        }
                        .to_core_state()
                    })
                    .collect(),
                pending_selection: flow.pending_selection,
            },
            Self::Result { clear_rate } => td_core::GameFlowState::Result {
                clear_rate_raw: clear_rate.raw(),
            },
        }
    }

    pub(crate) fn from_core_state(
        state: td_core::GameFlowState,
        presentation_source: Option<&Self>,
    ) -> Option<Self> {
        Some(match state {
            td_core::GameFlowState::Initializing => Self::Initializing,
            td_core::GameFlowState::Shopping(shop) => {
                let source_shop = match presentation_source {
                    Some(Self::Shopping(flow)) => Some(&flow.shop),
                    _ => None,
                };
                Self::Shopping(ShoppingFlow {
                    shop: crate::shop::Shop::from_core_state(shop, source_shop)?,
                })
            }
            td_core::GameFlowState::SelectingTower => Self::SelectingTower(SelectingTowerFlow {}),
            td_core::GameFlowState::PlacingTower => Self::PlacingTower,
            td_core::GameFlowState::Defense(defense) => {
                Self::Defense(DefenseFlow::from_core_state(defense))
            }
            td_core::GameFlowState::TreasureSelection {
                options,
                pending_selection,
            } => Self::TreasureSelection(TreasureSelectionFlow {
                options: options
                    .into_iter()
                    .map(crate::game_state::upgrade::UpgradeWithId::from_core_state)
                    .collect::<Option<Vec<_>>>()?
                    .into_iter()
                    .map(|upgrade| upgrade.upgrade)
                    .collect(),
                pending_selection,
            }),
            td_core::GameFlowState::Result { clear_rate_raw } => Self::Result {
                clear_rate: ClearRate::from_ratio(FixedRatio::from_raw(clear_rate_raw)),
            },
        })
    }
}

#[derive(Clone, Debug, State)]
pub struct TreasureSelectionFlow {
    pub options: Vec<crate::game_state::upgrade::Upgrade>,
    pub pending_selection: Option<usize>,
}

impl TreasureSelectionFlow {
    fn update(&mut self) {}
}
impl GameFlow {
    pub(crate) fn update(&mut self, presentation_instant: PresentationInstant) {
        match self {
            GameFlow::Shopping(shopping_flow) => shopping_flow.update(presentation_instant),
            GameFlow::SelectingTower(selecting_tower) => selecting_tower.update(),
            GameFlow::TreasureSelection(treasure_flow) => treasure_flow.update(),
            _ => {}
        }
    }
}

impl GameState {
    pub(crate) fn set_treasure_pending_selection(&mut self, option_index: usize) -> bool {
        let mut raw = self.raw_core.state().clone();
        let can_select = match raw.flow() {
            td_core::GameFlowState::TreasureSelection {
                options,
                pending_selection,
            } => pending_selection.is_none() && option_index < options.len(),
            _ => false,
        };
        if !can_select {
            return false;
        }
        if raw
            .edit_snapshot(|parts| {
                if let td_core::GameFlowState::TreasureSelection {
                    pending_selection, ..
                } = &mut parts.flow
                {
                    *pending_selection = Some(option_index);
                }
            })
            .is_err()
        {
            return false;
        }
        self.restore_raw_core_projection(raw).is_ok()
    }
}

#[derive(Clone, Debug, State)]
pub struct ShoppingFlow {
    pub shop: Shop,
}

impl ShoppingFlow {
    fn update(&mut self, presentation_instant: PresentationInstant) {
        self.shop.update(presentation_instant);
    }
}

#[derive(Clone, Debug, State)]
pub struct SelectingTowerFlow {}

impl SelectingTowerFlow {
    pub fn new(_game_state: &GameState) -> Self {
        SelectingTowerFlow {}
    }

    fn update(&mut self) {}
}

#[derive(Clone, Debug, State)]
pub struct DefenseFlow {
    pub stage_progress: StageProgress,
    pub took_damage: bool,
}

impl DefenseFlow {
    pub(crate) fn to_core_state(&self) -> td_core::DefenseFlowState {
        td_core::DefenseFlowState {
            start_total_hp_raw: self.stage_progress.start_total_hp.raw(),
            processed_hp_raw: self.stage_progress.processed_hp.raw(),
            took_damage: self.took_damage,
        }
    }

    pub(crate) fn from_core_state(state: td_core::DefenseFlowState) -> Self {
        Self {
            stage_progress: StageProgress {
                start_total_hp: Health::from_raw(state.start_total_hp_raw),
                processed_hp: Health::from_raw(state.processed_hp_raw),
            },
            took_damage: state.took_damage,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn defense_flow_raw_state_round_trip_preserves_payload() {
        let flow = DefenseFlow {
            stage_progress: StageProgress {
                start_total_hp: Health::from_raw(10_000),
                processed_hp: Health::from_raw(3_250),
            },
            took_damage: true,
        };

        let raw = flow.to_core_state();
        let restored = DefenseFlow::from_core_state(raw.clone());

        assert_eq!(restored.to_core_state(), raw);
    }

    #[test]
    fn game_flow_raw_state_round_trip_preserves_treasure_selection() {
        let flow = GameFlow::TreasureSelection(TreasureSelectionFlow {
            options: vec![
                crate::game_state::upgrade::Upgrade::Apple(
                    crate::game_state::upgrade::AppleUpgrade,
                ),
                crate::game_state::upgrade::Upgrade::Backpack(
                    crate::game_state::upgrade::BackpackUpgrade { add: 2 },
                ),
            ],
            pending_selection: Some(1),
        });

        let raw = flow.to_core_state();
        let restored = GameFlow::from_core_state(raw.clone(), None).expect("valid game flow");

        assert_eq!(restored.to_core_state(), raw);
    }

    #[test]
    fn game_flow_raw_state_preserves_simple_variants() {
        for flow in [
            GameFlow::Initializing,
            GameFlow::SelectingTower(SelectingTowerFlow {}),
            GameFlow::PlacingTower,
            GameFlow::Result {
                clear_rate: ClearRate::FULL,
            },
        ] {
            let raw = flow.to_core_state();
            let restored = GameFlow::from_core_state(raw.clone(), None).expect("valid game flow");
            assert_eq!(restored.to_core_state(), raw);
        }
    }

    #[test]
    fn headed_treasure_selection_sets_raw_pending_selection() {
        let mut game_state = crate::game_state::create_game_state_with_seed(7);
        game_state.apply_compatibility_action(
            crate::game_state::CompatibilityAction::StartTreasureSelection,
        );

        assert!(game_state.set_treasure_pending_selection(1));
        let pending_selection = match game_state.raw_core.flow() {
            td_core::GameFlowState::TreasureSelection {
                pending_selection, ..
            } => *pending_selection,
            _ => panic!("expected treasure selection flow"),
        };
        assert_eq!(pending_selection, Some(1));
        assert!(!game_state.set_treasure_pending_selection(2));
    }
}

impl DefenseFlow {
    pub fn new(game_state: &GameState) -> Self {
        Self::new_from_core(
            game_state.raw_core_state().progress().stage,
            &crate::config::GameConfig::from_core_state(
                game_state.raw_core_state().config().clone(),
            )
            .expect("raw game config must be restorable for defense flow"),
            &crate::game_state::stage_modifiers::StageModifiers::from_core_state(
                game_state.raw_core_state().stage_modifiers().clone(),
            )
            .expect("raw stage modifiers must be restorable for defense flow"),
        )
    }

    pub(crate) fn new_from_core(
        stage: usize,
        config: &crate::config::GameConfig,
        stage_modifiers: &crate::game_state::stage_modifiers::StageModifiers,
    ) -> Self {
        let start_total_hp = GameState::calculate_stage_total_hp(stage, config, stage_modifiers);
        let flow = Self {
            stage_progress: StageProgress {
                start_total_hp,
                processed_hp: Health::ZERO,
            },
            took_damage: false,
        };
        Self::from_core_state(flow.to_core_state())
    }
}

#[derive(Clone, Debug, State)]
pub struct StageProgress {
    pub start_total_hp: Health,
    pub processed_hp: Health,
}
