use super::UpgradeBehavior;
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct CrockUpgradeState {
    pub damage_steps: usize,
}
use super::support::push_acquired_upgrade;

const CROCK_DAMAGE_PER_STEP_RAW: i64 = 250_000;

const CROCK_GOLD_PER_DAMAGE: usize = 100;

fn acquire_crock(core: &mut crate::CoreState, mut upgrade: super::super::UpgradeEntry) -> usize {
    upgrade.crock_mut().damage_steps = core.progress.gold / CROCK_GOLD_PER_DAMAGE;
    push_acquired_upgrade(core, upgrade)
}

fn update_crock(
    context: &mut super::super::UpgradeTriggerContext,
    entry: &mut super::super::UpgradeEntry,
) -> bool {
    let state = entry.crock_mut();
    let next_step = context.progress.gold / CROCK_GOLD_PER_DAMAGE;
    let changed = state.damage_steps != next_step;
    state.damage_steps = next_step;
    changed
}

fn update_crock_shop(
    context: &mut super::super::UpgradeTriggerContext,
    entry: &mut super::super::UpgradeEntry,
    _: bool,
) -> bool {
    update_crock(context, entry)
}

fn tower_bonus_crock(upgrade: &super::super::UpgradeEntry, _: &crate::TowerState) -> i64 {
    upgrade
        .crock()
        .damage_steps
        .min(i64::MAX as usize)
        .try_into()
        .unwrap_or(i64::MAX)
        .saturating_mul(CROCK_DAMAGE_PER_STEP_RAW)
}

fn tower_template_bonus_crock(
    upgrade: &super::super::UpgradeEntry,
    _: &crate::TowerTemplateState,
) -> i64 {
    upgrade
        .crock()
        .damage_steps
        .min(i64::MAX as usize)
        .try_into()
        .unwrap_or(i64::MAX)
        .saturating_mul(CROCK_DAMAGE_PER_STEP_RAW)
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) struct Behavior;

impl UpgradeBehavior for Behavior {
    fn kind(&self) -> crate::UpgradeKind {
        crate::UpgradeKind::Crock
    }
    fn rarity(&self) -> crate::Rarity {
        crate::Rarity::Epic
    }
    fn generate(&self) -> super::super::UpgradeRuntimeState {
        super::super::UpgradeRuntimeState::Crock(super::super::codec_impl::CrockUpgradeState {
            damage_steps: 0,
        })
    }
    fn acquire(&self, core: &mut crate::CoreState, upgrade: super::super::UpgradeEntry) -> usize {
        acquire_crock(core, upgrade)
    }
    fn tower_bonus(&self, entry: &super::super::UpgradeEntry, tower: &crate::TowerState) -> i64 {
        tower_bonus_crock(entry, tower)
    }
    fn tower_bonus_for_template(
        &self,
        entry: &super::super::UpgradeEntry,
        template: &crate::TowerTemplateState,
    ) -> i64 {
        tower_template_bonus_crock(entry, template)
    }
    fn gold_earned(
        &self,
        context: &mut super::super::UpgradeTriggerContext,
        entry: &mut super::super::UpgradeEntry,
    ) -> bool {
        update_crock(context, entry)
    }
    fn shop_purchase(
        &self,
        context: &mut super::super::UpgradeTriggerContext,
        entry: &mut super::super::UpgradeEntry,
        item_purchase: bool,
    ) -> bool {
        update_crock_shop(context, entry, item_purchase)
    }
}
