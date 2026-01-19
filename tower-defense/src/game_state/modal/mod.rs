mod challenge_modal;
mod settings;
mod upgrade_board;

#[cfg(feature = "debug-tools")]
use crate::game_state::debug_tools::DebugToolsModal;
use crate::game_state::modal::{
    challenge_modal::ChallengeModal, settings::SettingsModal, upgrade_board::UpgradeBoardModal,
};
use namui::*;

#[derive(State)]
pub enum Modal {
    UpgradeBoard,
    Challenge,
    Settings,
    #[cfg(feature = "debug-tools")]
    DebugTools,
}

impl Component for &Modal {
    fn render(self, ctx: &RenderCtx) {
        match self {
            Modal::UpgradeBoard => ctx.add(UpgradeBoardModal),
            Modal::Challenge => ctx.add(ChallengeModal),
            Modal::Settings => ctx.add(SettingsModal),
            #[cfg(feature = "debug-tools")]
            Modal::DebugTools => ctx.add(DebugToolsModal),
        };
    }
}
