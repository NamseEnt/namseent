use crate::game_state::{GameStateAction, mutate_game_state};
use crate::icon::IconKind;
use crate::l10n::ui::FabTooltipText;
use crate::theme::fab::{FabPosition, FabSide, FabVerticalPosition, FloatingActionButton};
use crate::tooltip::TooltipContent;
use namui::*;

pub(super) struct PlacingTowerNextFab {
    pub screen_wh: Wh<Px>,
    pub visible: bool,
    pub has_unplaced_towers: bool,
}

impl Component for PlacingTowerNextFab {
    fn render(self, ctx: &RenderCtx) {
        let Self {
            screen_wh,
            visible,
            has_unplaced_towers,
        } = self;
        let next = || {
            if !visible {
                return;
            }
            mutate_game_state(|game_state| {
                game_state.action(GameStateAction::StartDefense);
            });
        };

        ctx.add(FloatingActionButton {
            screen_wh,
            position: FabPosition::new(FabSide::Right, FabVerticalPosition::BottomPrimary),
            visible,
            icon: IconKind::Play,
            disabled: false,
            long_press_time: has_unplaced_towers.then_some(Duration::from_millis(1200)),
            on_click: &next,
            tooltip_content: Some(TooltipContent::Fab {
                text: FabTooltipText::StartDefense,
                health_cost: None,
            }),
        });
    }
}
