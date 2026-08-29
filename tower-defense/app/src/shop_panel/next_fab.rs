use crate::game_state::{PlayerCommand, dispatch_player_command};
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
            dispatch_player_command(PlayerCommand::StartSelectingTower);
        };

        ctx.add(FloatingActionButton {
            screen_wh,
            position: FabPosition::new(FabSide::Right, FabVerticalPosition::BottomPrimary),
            visible,
            icon: IconKind::Accept,
            long_press_time: None,
            disabled: false,
            on_click: &next,
            tooltip_content: Some(TooltipContent::Fab {
                text: FabTooltipText::ShopNext,
                health_cost: None,
            }),
        });
    }
}
