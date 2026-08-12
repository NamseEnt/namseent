use crate::game_state::{GameStateAction, mutate_game_state};
use crate::icon::IconKind;
use crate::l10n::ui::FabTooltipText;
use crate::theme::fab::{FabPosition, FabSide, FabVerticalPosition, FloatingActionButton};
use crate::tooltip::TooltipContent;
use namui::*;

pub(super) struct ShopNextFab {
    pub screen_wh: Wh<Px>,
    pub visible: bool,
}

impl Component for ShopNextFab {
    fn render(self, ctx: &RenderCtx) {
        let Self { screen_wh, visible } = self;
        let next = || {
            if !visible {
                return;
            }
            mutate_game_state(|game_state| {
                game_state.action(GameStateAction::StartSelectingTower);
            });
        };

        ctx.add(FloatingActionButton {
            screen_wh,
            position: FabPosition::new(FabSide::Right, FabVerticalPosition::BottomPrimary),
            visible,
            icon: IconKind::Accept,
            disabled: false,
            on_click: &next,
            tooltip_content: Some(TooltipContent::Fab(FabTooltipText::ShopNext)),
        });
    }
}
