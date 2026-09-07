use super::UpgradeBehavior;
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct CameraUpgradeState;

const CAMERA_GOLD_REWARD: usize = 50;

fn tower_placed_camera(
    _: &mut super::super::UpgradeTriggerContext,
    _: &mut super::super::UpgradeEntry,
    _: u64,
    is_face: bool,
    _: &crate::TowerTemplateState,
    camera_reward: &mut usize,
) -> bool {
    if is_face {
        *camera_reward = camera_reward.saturating_add(CAMERA_GOLD_REWARD);
    }
    false
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) struct Behavior;

impl UpgradeBehavior for Behavior {
    fn kind(&self) -> crate::UpgradeKind {
        crate::UpgradeKind::Camera
    }
    fn rarity(&self) -> crate::Rarity {
        crate::Rarity::Epic
    }
    fn generate(&self) -> super::super::UpgradeRuntimeState {
        super::super::UpgradeRuntimeState::Camera(super::super::codec_impl::CameraUpgradeState)
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
        tower_placed_camera(context, entry, tower_id, is_face, template, reward)
    }
}
