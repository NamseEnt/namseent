mod deck;
mod settings;

#[cfg(feature = "debug-tools")]
use crate::game_state::debug_tools::DebugToolsModal;
use crate::game_state::modal::deck::DeckModal;
use crate::game_state::modal::settings::SettingsModal;
use namui::*;

#[derive(State)]
pub enum Modal {
    Settings,
    Deck,
    #[cfg(feature = "debug-tools")]
    DebugTools,
}

impl Component for &Modal {
    fn render(self, ctx: &RenderCtx) {
        match self {
            Modal::Settings => ctx.add(SettingsModal),
            Modal::Deck => ctx.add(DeckModal {
                deck_kind: deck::DeckKind::Deck,
            }),
            #[cfg(feature = "debug-tools")]
            Modal::DebugTools => ctx.add(DebugToolsModal),
        };
    }
}
