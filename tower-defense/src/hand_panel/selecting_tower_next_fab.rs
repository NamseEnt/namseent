use crate::game_state::{PlayerCommand, mutate_game_state};
use crate::icon::IconKind;
use crate::l10n::ui::FabTooltipText;
use crate::theme::fab::{FabPosition, FabSide, FabVerticalPosition, FloatingActionButton};
use crate::tooltip::TooltipContent;
use namui::*;

pub(super) struct SelectingTowerNextFab {
    pub screen_wh: Wh<Px>,
    pub visible: bool,
    pub tower_template_available: bool,
    pub selected_slot_indices: Vec<usize>,
}

impl Component for SelectingTowerNextFab {
    fn render(self, ctx: &RenderCtx) {
        let Self {
            screen_wh,
            visible,
            tower_template_available,
            selected_slot_indices,
        } = self;
        let next = || {
            if !visible {
                return;
            }
            if !tower_template_available {
                return;
            }
            let selected_slot_indices = selected_slot_indices.clone();
            mutate_game_state(|game_state| {
                let _ = game_state.apply_player_command(PlayerCommand::SelectTower {
                    selected_slot_indices,
                });
            });
        };

        ctx.add(FloatingActionButton {
            screen_wh,
            position: FabPosition::new(FabSide::Right, FabVerticalPosition::BottomPrimary),
            visible,
            icon: IconKind::Accept,
            disabled: !tower_template_available,
            long_press_time: None,
            on_click: &next,
            tooltip_content: Some(TooltipContent::Fab {
                text: FabTooltipText::CreateTower,
                health_cost: None,
            }),
        });
    }
}
