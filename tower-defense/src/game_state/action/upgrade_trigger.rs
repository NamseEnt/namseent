use crate::game_state::{
    GameState,
    tower::Tower,
    upgrade::{Upgrade, UpgradeBehavior, UpgradeUpdateFlags},
};

pub(super) enum UpgradeTriggerEvent<'a> {
    UpgradeAcquired {
        upgrade: Upgrade,
    },
    StageStart {
        stage: usize,
    },
    TowerPlaced {
        tower: &'a Tower,
    },
    TowerRemoved,
    ItemBought,
    GoldEarned {
        amount: usize,
    },
    GoldSpent {
        amount: usize,
    },
    CardReroll,
    StageEnd {
        perfect_clear: bool,
        gold: usize,
        item_count: usize,
    },
}

impl GameState {
    fn refresh_upgrade_trigger_side_effects(&mut self, flags: UpgradeUpdateFlags) {
        if flags != UpgradeUpdateFlags::NONE {
            self.upgrade_state.revision = self.upgrade_state.revision.wrapping_add(1);
        }

        if flags.contains(UpgradeUpdateFlags::CACHE) {
            self.upgrade_state.rebuild_cache();
        }

        if flags.contains(UpgradeUpdateFlags::TOWER_STATS) {
            let upgrade_bonuses = self.upgrade_state.tower_upgrade_damage_bonuses();
            for tower in self.towers.iter_mut() {
                tower.refresh_cached_upgrade_damage(self.upgrade_state.revision, &upgrade_bonuses);
            }
        }

        if flags.contains(UpgradeUpdateFlags::HEAL_TO_FULL) {
            self.hp = self.max_hp();
        }
    }

    fn foreach_upgrades<F>(&mut self, mut f: F) -> UpgradeUpdateFlags
    where
        F: FnMut(&mut Upgrade, &mut GameState) -> UpgradeUpdateFlags,
    {
        let mut upgrades = std::mem::take(&mut self.upgrade_state.upgrades);

        let flags = upgrades
            .iter_mut()
            .fold(UpgradeUpdateFlags::NONE, |acc, upgrade| {
                acc | f(upgrade, self)
            });
        self.upgrade_state.upgrades = upgrades;

        flags
    }

    pub(super) fn handle_upgrade_trigger(&mut self, event: UpgradeTriggerEvent<'_>) {
        let flags = match event {
            UpgradeTriggerEvent::UpgradeAcquired { upgrade } => upgrade.acquire(self),
            UpgradeTriggerEvent::TowerPlaced { tower } => self
                .foreach_upgrades(|upgrade, game_state| upgrade.on_tower_placed(game_state, tower)),
            UpgradeTriggerEvent::StageStart { stage } => self
                .foreach_upgrades(|upgrade, game_state| upgrade.on_stage_start(game_state, stage)),
            UpgradeTriggerEvent::TowerRemoved => {
                self.foreach_upgrades(|upgrade, game_state| upgrade.on_tower_removed(game_state))
            }
            UpgradeTriggerEvent::ItemBought => {
                self.foreach_upgrades(|upgrade, game_state| upgrade.on_item_bought(game_state))
            }
            UpgradeTriggerEvent::GoldEarned { amount } => self
                .foreach_upgrades(|upgrade, game_state| upgrade.on_gold_earned(game_state, amount)),
            UpgradeTriggerEvent::GoldSpent { amount } => self
                .foreach_upgrades(|upgrade, game_state| upgrade.on_gold_spent(game_state, amount)),
            UpgradeTriggerEvent::CardReroll => {
                self.foreach_upgrades(|upgrade, game_state| upgrade.on_card_reroll(game_state))
            }
            UpgradeTriggerEvent::StageEnd {
                perfect_clear,
                gold,
                item_count,
            } => self.foreach_upgrades(|upgrade, game_state| {
                upgrade.on_stage_end(game_state, perfect_clear, gold, item_count)
            }),
        };
        self.refresh_upgrade_trigger_side_effects(flags);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::card::{Rank, Suit};
    use crate::game_state::create_initial_game_state;
    use crate::game_state::flow::GameFlow;
    use crate::game_state::tower::{Tower, TowerKind, TowerTemplate};
    use namui::Instant;

    #[test]
    fn camera_gold_earning_does_not_refresh_shop_when_selecting_tower() {
        let mut game_state = create_initial_game_state();
        game_state.action(crate::game_state::GameStateAction::Upgrade(
            Upgrade::Camera(crate::game_state::upgrade::CameraUpgrade),
            None,
        ));

        let old_ids: Vec<_> = match &game_state.flow {
            GameFlow::SelectingTower(flow) => flow.shop.slots.iter().map(|slot| slot.id).collect(),
            _ => panic!("expected selecting tower flow"),
        };

        let tower_template = TowerTemplate::new(TowerKind::Barricade, Suit::Spades, Rank::Jack);
        let tower = Tower::new(&tower_template, crate::MapCoord::new(0, 0), Instant::now());

        game_state.handle_upgrade_trigger(UpgradeTriggerEvent::TowerPlaced { tower: &tower });

        let new_ids: Vec<_> = match &game_state.flow {
            GameFlow::SelectingTower(flow) => flow.shop.slots.iter().map(|slot| slot.id).collect(),
            _ => panic!("expected selecting tower flow"),
        };

        assert_eq!(new_ids, old_ids);
        assert_eq!(game_state.gold, game_state.config.player.starting_gold + 50);
    }

    #[test]
    fn no_flag_does_not_increment_upgrade_revision() {
        let mut game_state = create_initial_game_state();
        let before = game_state.upgrade_state.revision;

        game_state.refresh_upgrade_trigger_side_effects(
            crate::game_state::upgrade::UpgradeUpdateFlags::NONE,
        );

        assert_eq!(game_state.upgrade_state.revision, before);
    }
}
