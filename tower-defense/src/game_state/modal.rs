#[cfg(feature = "debug-tools")]
use crate::game_state::debug_tools::DebugToolsModal;
use crate::{
    game_state::start_confirm_modal::StartConfirmModal, settings::SettingsModal,
    upgrade_board::UpgradeBoardModal,
};
use namui::*;

#[derive(State)]
pub enum Modal {
    UpgradeBoard,
    Settings,
    StartConfirm,
    #[cfg(feature = "debug-tools")]
    DebugTools,
}

impl Component for &Modal {
    fn render(self, ctx: &RenderCtx) {
        match self {
            Modal::UpgradeBoard => ctx.add(UpgradeBoardModal),
            Modal::Settings => ctx.add(SettingsModal),
            Modal::StartConfirm => ctx.add(StartConfirmModal),
            #[cfg(feature = "debug-tools")]
            Modal::DebugTools => ctx.add(DebugToolsModal),
        };
    }
}
