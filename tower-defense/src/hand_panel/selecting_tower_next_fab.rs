use crate::game_state::{GameStateAction, mutate_game_state, tower::TowerTemplate};
use crate::icon::IconKind;
use crate::l10n::ui::FabTooltipText;
use crate::theme::fab::{FabPosition, FabSide, FabVerticalPosition, FloatingActionButton};
use crate::tooltip::TooltipContent;
use namui::*;

pub(super) struct SelectingTowerNextFab {
    pub screen_wh: Wh<Px>,
    pub visible: bool,
    pub tower_template: Option<TowerTemplate>,
}

impl Component for SelectingTowerNextFab {
    fn render(self, ctx: &RenderCtx) {
        let Self {
            screen_wh,
            visible,
            tower_template,
        } = self;
        let next = || {
            if !visible {
                return;
            }
            let Some(tower_template) = tower_template.clone() else {
                return;
            };
            mutate_game_state(|game_state| {
                game_state.action(GameStateAction::StartPlacingTower(tower_template));
            });
        };

        ctx.add(FloatingActionButton {
            screen_wh,
            position: FabPosition::new(FabSide::Right, FabVerticalPosition::BottomPrimary),
            visible,
            icon: IconKind::Accept,
            disabled: tower_template.is_none(),
            long_press_time: None,
            on_click: &next,
            tooltip_content: Some(TooltipContent::Fab {
                text: FabTooltipText::CreateTower,
                health_cost: None,
            }),
        });
    }
}
