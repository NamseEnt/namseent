use crate::{
    game_state::debug_tools::DebugToolsModal, game_state::start_confirm_modal::StartConfirmModal,
    settings::SettingsModal, upgrade_board::UpgradeBoardModal,
};
use namui::*;

#[derive(State)]
pub enum Modal {
    UpgradeBoard,
    Settings,
    StartConfirm,
    DebugTools,
}

impl Component for &Modal {
    fn render(self, ctx: &RenderCtx) {
        match self {
            Modal::UpgradeBoard => ctx.add(UpgradeBoardModal),
            Modal::Settings => ctx.add(SettingsModal),
            Modal::StartConfirm => ctx.add(StartConfirmModal),
            Modal::DebugTools => ctx.add(DebugToolsModal),
        };
    }
}
