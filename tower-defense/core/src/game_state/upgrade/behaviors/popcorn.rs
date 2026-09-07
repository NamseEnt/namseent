use super::UpgradeBehavior;
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PopcornUpgradeState {
    pub max_multiplier_raw: i64,
    pub duration_waves: usize,
    pub active_multiplier_raw: i64,
    pub waves_remaining: usize,
}

impl PopcornUpgradeState {
    pub(crate) fn damage_bonus_raw(self, waves_remaining: usize) -> i64 {
        if waves_remaining == 0 {
            return 0;
        }
        let duration = self.duration_waves.max(1);
        let elapsed = duration.saturating_sub(waves_remaining);
        let multiplier = if duration <= 1 {
            self.max_multiplier_raw
        } else {
            let step = self
                .max_multiplier_raw
                .saturating_sub(crate::RATIO_SCALE)
                .max(0)
                .saturating_div((duration - 1).min(i64::MAX as usize) as i64);
            self.max_multiplier_raw
                .saturating_sub(step.saturating_mul(elapsed.min(i64::MAX as usize) as i64))
                .max(crate::RATIO_SCALE)
        };
        multiplier.saturating_sub(crate::RATIO_SCALE)
    }
}

fn stage_start_popcorn(
    _: &mut super::super::UpgradeTriggerContext,
    entry: &mut super::super::UpgradeEntry,
    _: usize,
) -> bool {
    let state = entry.popcorn_mut();
    let active = state.damage_bonus_raw(state.waves_remaining);
    let changed = state.active_multiplier_raw != active;
    state.active_multiplier_raw = active;
    changed
}

fn stage_end_popcorn(
    _: &mut super::super::UpgradeTriggerContext,
    upgrade: &mut super::super::UpgradeEntry,
    _: bool,
    _: usize,
    _: usize,
) -> (bool, usize) {
    let state = upgrade.popcorn_mut();
    if state.waves_remaining == 0 {
        return (false, 0);
    }
    state.waves_remaining -= 1;
    state.active_multiplier_raw = state.damage_bonus_raw(state.waves_remaining);
    (true, 0)
}

fn tower_bonus_popcorn(upgrade: &super::super::UpgradeEntry, _: &crate::TowerState) -> i64 {
    upgrade.popcorn().active_multiplier_raw
}

fn tower_template_bonus_popcorn(
    upgrade: &super::super::UpgradeEntry,
    _: &crate::TowerTemplateState,
) -> i64 {
    upgrade.popcorn().active_multiplier_raw
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) struct Behavior;

impl UpgradeBehavior for Behavior {
    fn kind(&self) -> crate::UpgradeKind {
        crate::UpgradeKind::Popcorn
    }
    fn rarity(&self) -> crate::Rarity {
        crate::Rarity::Rare
    }
    fn generate(&self) -> super::super::UpgradeRuntimeState {
        super::super::UpgradeRuntimeState::Popcorn(super::super::codec_impl::PopcornUpgradeState {
            max_multiplier_raw: 5_000_000,
            duration_waves: 5,
            active_multiplier_raw: 0,
            waves_remaining: 5,
        })
    }
    fn tower_bonus(&self, entry: &super::super::UpgradeEntry, tower: &crate::TowerState) -> i64 {
        tower_bonus_popcorn(entry, tower)
    }
    fn tower_bonus_for_template(
        &self,
        entry: &super::super::UpgradeEntry,
        template: &crate::TowerTemplateState,
    ) -> i64 {
        tower_template_bonus_popcorn(entry, template)
    }
    fn stage_start(
        &self,
        context: &mut super::super::UpgradeTriggerContext,
        entry: &mut super::super::UpgradeEntry,
        stage: usize,
    ) -> bool {
        stage_start_popcorn(context, entry, stage)
    }
    fn stage_end(
        &self,
        context: &mut super::super::UpgradeTriggerContext,
        entry: &mut super::super::UpgradeEntry,
        perfect_clear: bool,
        gold: usize,
        item_count: usize,
    ) -> (bool, usize) {
        stage_end_popcorn(context, entry, perfect_clear, gold, item_count)
    }
}
