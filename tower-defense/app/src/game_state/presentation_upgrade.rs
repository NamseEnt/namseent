use super::{
    presentation_transition::{PresentationTransition, diff_projection},
    upgrade::{UpgradeState, UpgradeWithId},
};
use crate::PresentationInstant;
use namui::*;

#[derive(Clone, Debug, State)]
pub(crate) struct PresentationUpgradeEntry {
    pub upgrade: UpgradeWithId,
    pub order: usize,
    pub exit_started_at: Option<PresentationInstant>,
}

impl PresentationUpgradeEntry {
    pub fn is_exiting(&self) -> bool {
        self.exit_started_at.is_some()
    }
}

#[derive(Clone, Debug, Default, State)]
pub(crate) struct PresentationUpgradeList {
    pub entries: Vec<PresentationUpgradeEntry>,
}

impl PresentationUpgradeList {
    pub fn sync(
        &mut self,
        state: &UpgradeState,
        presentation_instant: PresentationInstant,
        restore: bool,
    ) -> PresentationTransition<super::upgrade::UpgradeId> {
        let before = self
            .entries
            .iter()
            .filter(|entry| !entry.is_exiting())
            .map(|entry| (entry.upgrade.id, entry.upgrade.upgrade))
            .collect::<Vec<_>>();
        let after = state
            .upgrades
            .iter()
            .map(|upgrade| (upgrade.id, upgrade.upgrade))
            .collect::<Vec<_>>();
        let transition = diff_projection(&before, &after);

        if restore {
            self.entries = state
                .upgrades
                .iter()
                .enumerate()
                .map(|(order, upgrade)| PresentationUpgradeEntry {
                    upgrade: *upgrade,
                    order,
                    exit_started_at: None,
                })
                .collect();
            return transition;
        }

        for entry in &mut self.entries {
            if let Some((order, upgrade)) = state
                .upgrades
                .iter()
                .enumerate()
                .find(|(_, upgrade)| upgrade.id == entry.upgrade.id)
            {
                entry.upgrade = *upgrade;
                entry.order = order;
                entry.exit_started_at = None;
            } else if entry.exit_started_at.is_none() {
                entry.exit_started_at = Some(presentation_instant);
            }
        }

        for (order, upgrade) in state.upgrades.iter().enumerate() {
            if !self
                .entries
                .iter()
                .any(|entry| entry.upgrade.id == upgrade.id)
            {
                self.entries.push(PresentationUpgradeEntry {
                    upgrade: *upgrade,
                    order,
                    exit_started_at: None,
                });
            }
        }

        self.entries
            .sort_by_key(|entry| (entry.is_exiting(), entry.order));
        transition
    }

    pub fn update(&mut self, presentation_instant: PresentationInstant) {
        self.entries.retain(|entry| {
            entry
                .exit_started_at
                .is_none_or(|started| (presentation_instant - started).as_secs_f32() < 0.5)
        });
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::game_state::upgrade::{AppleUpgrade, Upgrade};

    #[test]
    fn removed_upgrades_exit_without_remaining_interaction_entries() {
        let upgrade = UpgradeState::with_upgrades(vec![Upgrade::Apple(AppleUpgrade)]);
        let mut presentation = PresentationUpgradeList::default();
        let instant = PresentationInstant::zero();

        presentation.sync(&upgrade, instant, true);
        presentation.sync(&UpgradeState::default(), instant, false);

        assert_eq!(presentation.entries.len(), 1);
        assert!(presentation.entries[0].is_exiting());
    }
}
