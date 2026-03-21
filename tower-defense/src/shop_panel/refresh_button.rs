use crate::game_state::{mutate_game_state, use_game_state};
use crate::icon::IconKind;
use crate::shop::refresh_shop;
use crate::theme::button::{Button, ButtonVariant};
use crate::theme::typography::memoized_text;
use namui::*;

pub struct RefreshButton {
    pub wh: Wh<Px>,
}

impl RefreshButton {
    pub fn new(wh: Wh<Px>) -> Self {
        Self { wh }
    }
}

impl Component for RefreshButton {
    fn render(self, ctx: &RenderCtx) {
        let Self { wh } = self;
        let game_state = use_game_state(ctx);

        let disabled = game_state.left_dice == 0 || {
            let health_cost = game_state.stage_modifiers.get_reroll_health_cost();
            (game_state.hp - health_cost as f32) < 1.0
        };

        let on_refresh = || {
            mutate_game_state(|game_state| {
                let health_cost = game_state.stage_modifiers.get_reroll_health_cost();
                if (game_state.hp - health_cost as f32) < 1.0 {
                    return;
                }
                game_state.left_dice -= 1;
                game_state.take_damage(health_cost as f32);
                refresh_shop(game_state);
            });
        };

        ctx.add(
            Button::new(wh, &on_refresh, &|wh, color, ctx| {
                let game_state = use_game_state(ctx);
                let health_cost = game_state.stage_modifiers.get_reroll_health_cost();
                let left_dice = game_state.left_dice;

                ctx.add(memoized_text(
                    (&color, &left_dice, &health_cost),
                    |mut builder| {
                        let mut builder = builder
                            .headline()
                            .size(crate::theme::typography::FontSize::Large)
                            .color(color)
                            .icon(IconKind::Refresh);

                        if health_cost > 0 {
                            builder = builder.space().icon(IconKind::Health);
                        }

                        builder.render_center(wh)
                    },
                ));
            })
            .variant(ButtonVariant::Fab)
            .disabled(disabled),
        );
    }
}
