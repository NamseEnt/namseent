mod card_grid;
pub mod deck;
pub mod encyclopedia;
mod settings;
mod tower_details;

#[cfg(feature = "debug-tools")]
use crate::game_state::debug_tools::DebugToolsModal;
use crate::game_state::modal::settings::SettingsModal;
pub use deck::DeckModal;
use encyclopedia::EncyclopediaModal;
use namui::*;
pub use tower_details::TowerDetailsModal;

#[derive(Debug, Clone, State)]
pub enum SystemModal {
    Settings,
    Encyclopedia,
    #[cfg(feature = "debug-tools")]
    DebugTools,
}

#[derive(Debug, Clone, State, Default)]
pub struct OpenedModals {
    pub user: Option<UserModal>,
    pub system: Option<SystemModal>,
}

impl Component for &SystemModal {
    fn render(self, ctx: &RenderCtx) {
        match self {
            SystemModal::Settings => {
                ctx.add(SettingsModal);
            }
            SystemModal::Encyclopedia => {
                ctx.add(EncyclopediaModal);
            }
            #[cfg(feature = "debug-tools")]
            SystemModal::DebugTools => {
                ctx.add(DebugToolsModal);
            }
        }
    }
}

#[derive(Debug, Clone, State)]
pub enum UserModal {
    Deck(DeckModal),
    TowerDetails(TowerDetailsModal),
}

impl Component for &UserModal {
    fn render(self, ctx: &RenderCtx) {
        match self {
            UserModal::Deck(deck_modal) => ctx.add(deck_modal.clone()),
            UserModal::TowerDetails(tower_details_modal) => ctx.add(tower_details_modal.clone()),
        };
    }
}
