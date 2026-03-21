use crate::animation::with_spring;
use crate::game_state::{mutate_game_state, use_game_state};
use crate::icon::IconKind;
use crate::shop::refresh_shop;
use crate::theme::button::{Button, ButtonVariant};
use crate::theme::typography::memoized_text;
use crate::tooltip::reroll_health_cost_warning_tooltip::RerollHealthCostWarningTooltip;
use namui::*;
use namui_prebuilt::simple_rect;

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

        let health_cost = game_state.stage_modifiers.get_reroll_health_cost();
        let disabled = game_state.left_dice == 0 || (game_state.hp - health_cost as f32) < 1.0;

        let (hovering, set_hovering) = ctx.state(|| false);

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
            simple_rect(wh, Color::TRANSPARENT, 0.px(), Color::TRANSPARENT).attach_event(
                move |event| {
                    let Event::MouseMove { event } = event else {
                        return;
                    };
                    set_hovering.set(event.is_local_xy_in());
                },
            ),
        );

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

        let locale = game_state.text().locale();

        let tooltip_scale = with_spring(
            ctx,
            if *hovering && health_cost > 0 {
                1.0
            } else {
                0.0
            },
            0.0,
            |v| v * v,
            || 0.0,
        );

        ctx.compose(|ctx| {
            if tooltip_scale > 0.01 {
                let tooltip = ctx.ghost_add(
                    "reroll-tooltip",
                    RerollHealthCostWarningTooltip {
                        health_cost,
                        locale,
                    },
                );

                if let Some(tooltip_wh) = tooltip.bounding_box().map(|rect| rect.wh()) {
                    let pivot = Xy::new(0.px(), tooltip_wh.height / 2.0);
                    let tooltip_gap = 10.px();
                    let base = Xy::new(
                        wh.width + tooltip_gap,
                        (wh.height - tooltip_wh.height) / 2.0,
                    );

                    ctx.translate(base + pivot)
                        .scale(Xy::new(tooltip_scale, tooltip_scale))
                        .translate(Xy::new(-pivot.x, -pivot.y))
                        .on_top()
                        .add(tooltip);
                }
            }
        });
    }
}
