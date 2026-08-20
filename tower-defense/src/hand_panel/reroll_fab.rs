use crate::game_state::{PlayerCommand, mutate_game_state};
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
    pub selected_slot_indices: Vec<usize>,
}

impl Component for HandRerollFab {
    fn render(self, ctx: &RenderCtx) {
        let Self {
            screen_wh,
            visible,
            disabled,
            health_cost,
            selected_slot_indices,
        } = self;
        let reroll = || {
            if !visible || disabled {
                return;
            }
            let selected_slot_indices = selected_slot_indices.clone();
            mutate_game_state(|game_state| {
                let _ = game_state.apply_player_command(PlayerCommand::Reroll {
                    selected_slot_indices,
                });
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
