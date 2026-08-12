use crate::game_state::{GameStateAction, mutate_game_state};
use crate::icon::IconKind;
use crate::l10n::ui::FabTooltipText;
use crate::theme::fab::{FabPosition, FabSide, FabVerticalPosition, FloatingActionButton};
use crate::tooltip::TooltipContent;
use namui::*;

pub(super) struct HandRerollFab {
    pub screen_wh: Wh<Px>,
    pub visible: bool,
    pub disabled: bool,
    pub health_cost: usize,
}

impl Component for HandRerollFab {
    fn render(self, ctx: &RenderCtx) {
        let Self {
            screen_wh,
            visible,
            disabled,
            health_cost,
        } = self;
        let reroll = || {
            if !visible || disabled {
                return;
            }
            mutate_game_state(|game_state| {
                game_state.action(GameStateAction::CardReroll);
            });
        };

        ctx.add(FloatingActionButton {
            screen_wh,
            position: FabPosition::new(FabSide::Right, FabVerticalPosition::BottomSecondary),
            visible,
            icon: IconKind::Refresh,
            disabled,
            long_press_time: None,
            on_click: &reroll,
            tooltip_content: Some(TooltipContent::Fab {
                text: FabTooltipText::RerollHand,
                health_cost: (health_cost > 0).then_some(health_cost),
            }),
        });
    }
}
