use super::{
    item::ItemWithId, presentation_deck::PresentationDeckZones,
    presentation_inventory::PresentationInventory, presentation_upgrade::PresentationUpgradeList,
    upgrade::UpgradeState,
};
use crate::PresentationInstant;
use crate::card::Deck;

pub(crate) struct PresentationReconcileInput<'a> {
    pub(crate) inventory: &'a mut PresentationInventory,
    pub(crate) upgrades: &'a mut PresentationUpgradeList,
    pub(crate) deck_zones: &'a mut PresentationDeckZones,
    pub(crate) items: &'a [ItemWithId],
    pub(crate) upgrade_state: &'a UpgradeState,
    pub(crate) deck: &'a Deck,
    pub(crate) presentation_instant: PresentationInstant,
    pub(crate) restore: bool,
}

pub(crate) struct PresentationReconciler;

impl PresentationReconciler {
    pub(crate) fn reconcile(input: PresentationReconcileInput<'_>) {
        input
            .inventory
            .sync(input.items, input.presentation_instant, input.restore);
        input.upgrades.sync(
            input.upgrade_state,
            input.presentation_instant,
            input.restore,
        );
        input
            .deck_zones
            .sync(input.deck, input.presentation_instant, input.restore);
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn treasure_selection_reconciles_the_acquired_upgrade() {
        let mut game_state = crate::game_state::create_game_state_with_seed(7);
        game_state.apply_compatibility_action(
            crate::game_state::CompatibilityAction::StartTreasureSelection,
        );

        game_state
            .apply_player_command_at(
                crate::game_state::HeadedPlayerCommand::SelectTreasure { option_index: 0 },
                crate::PresentationInstant::zero(),
            )
            .expect("treasure selection should be accepted");

        let active_ids = game_state
            .presentation_upgrade_entries_snapshot()
            .into_iter()
            .filter(|entry| !entry.is_exiting())
            .map(|entry| entry.upgrade.id.0)
            .collect::<Vec<_>>();
        let core_ids = game_state
            .raw_core_state()
            .upgrades()
            .upgrades
            .iter()
            .map(|upgrade| upgrade.id)
            .collect::<Vec<_>>();

        assert_eq!(active_ids.len(), core_ids.len());
        assert!(core_ids.iter().all(|id| active_ids.contains(id)));
    }
}
