mod settings;
mod upgrade_board;

#[cfg(feature = "debug-tools")]
use crate::game_state::debug_tools::DebugToolsModal;
use crate::game_state::modal::{settings::SettingsModal, upgrade_board::UpgradeBoardModal};
use namui::*;

#[derive(State)]
pub enum Modal {
    UpgradeBoard,
    Settings,
    #[cfg(feature = "debug-tools")]
    DebugTools,
}

impl Component for &Modal {
    fn render(self, ctx: &RenderCtx) {
        match self {
            Modal::UpgradeBoard => ctx.add(UpgradeBoardModal),
            Modal::Settings => ctx.add(SettingsModal),
            #[cfg(feature = "debug-tools")]
            Modal::DebugTools => ctx.add(DebugToolsModal),
        };
    }
}
