use super::UpgradeBehavior;
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct NameTagUpgradeState {
    pub bonus_raw: i64,
    pub tower_id: Option<u64>,
}

fn tower_placed_mirror(
    _: &mut super::super::UpgradeTriggerContext,
    entry: &mut super::super::UpgradeEntry,
    tower_id: u64,
    _: bool,
    _: &crate::TowerTemplateState,
    _: &mut usize,
) -> bool {
    if entry.name_tag().tower_id.is_none() {
        entry.name_tag_mut().tower_id = Some(tower_id);
        return true;
    }
    false
}

fn tower_bonus_mirror(upgrade: &super::super::UpgradeEntry, tower: &crate::TowerState) -> i64 {
    let state = upgrade.name_tag();
    if state.tower_id == tower.id {
        state.bonus_raw
    } else {
        0
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) struct Behavior;

impl UpgradeBehavior for Behavior {
    fn kind(&self) -> crate::UpgradeKind {
        crate::UpgradeKind::NameTag
    }
    fn rarity(&self) -> crate::Rarity {
        crate::Rarity::Epic
    }
    fn generate(&self) -> super::super::UpgradeRuntimeState {
        super::super::UpgradeRuntimeState::NameTag(super::super::codec_impl::NameTagUpgradeState {
            bonus_raw: 2_000_000,
            tower_id: None,
        })
    }
    fn tower_bonus(&self, entry: &super::super::UpgradeEntry, tower: &crate::TowerState) -> i64 {
        tower_bonus_mirror(entry, tower)
    }
    fn tower_placed(
        &self,
        context: &mut super::super::UpgradeTriggerContext,
        entry: &mut super::super::UpgradeEntry,
        tower_id: u64,
        is_face: bool,
        template: &crate::TowerTemplateState,
        reward: &mut usize,
    ) -> bool {
        tower_placed_mirror(context, entry, tower_id, is_face, template, reward)
    }
}
