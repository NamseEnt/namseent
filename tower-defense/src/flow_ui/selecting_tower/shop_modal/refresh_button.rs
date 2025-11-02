use crate::game_state::{mutate_game_state, use_game_state};
use crate::icon::{Icon, IconKind, IconSize};
use crate::shop::refresh_shop;
use crate::theme::button::{Button, ButtonVariant};
use crate::theme::typography::{TextAlign, headline};
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

        let disabled = game_state.left_shop_refresh_chance == 0 || {
            let health_cost = game_state.stage_modifiers.get_shop_reroll_health_cost();
            (game_state.hp - health_cost as f32) < 1.0
        };

        let on_refresh = || {
            mutate_game_state(|game_state| {
                let health_cost = game_state.stage_modifiers.get_shop_reroll_health_cost();
                if (game_state.hp - health_cost as f32) < 1.0 {
                    return;
                }
                game_state.left_shop_refresh_chance -= 1;
                game_state.take_damage(health_cost as f32);
                refresh_shop(game_state);
            });
        };

        ctx.add(
            Button::new(wh, &on_refresh, &|wh, color, ctx| {
                let game_state = use_game_state(ctx);
                let health_cost = game_state.stage_modifiers.get_shop_reroll_health_cost();

                let mut text = format!(
                    "{}-{}",
                    Icon::new(IconKind::Refresh)
                        .size(IconSize::Large)
                        .wh(Wh::single(wh.height))
                        .as_tag(),
                    game_state.left_shop_refresh_chance
                );

                if health_cost > 0 {
                    text.push_str(&format!(
                        " {}",
                        Icon::new(IconKind::Health)
                            .size(IconSize::Small)
                            .wh(Wh::single(wh.height * 0.5))
                            .as_tag()
                    ));
                }

                ctx.add(
                    headline(text)
                        .color(color)
                        .align(TextAlign::Center { wh })
                        .build_rich(),
                );
            })
            .variant(ButtonVariant::Fab)
            .disabled(disabled),
        );
    }
}
