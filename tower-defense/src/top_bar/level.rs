use crate::icon::IconAttribute;
use crate::theme::button::Button;
use crate::{
    game_state::{mutate_game_state, use_game_state},
    icon::{Icon, IconKind, IconSize},
    palette,
    theme::typography::headline,
};
use namui::*;
use namui_prebuilt::{simple_rect, table};

const PADDING: Px = px(8.);

pub struct LevelIndicator {
    pub wh: Wh<Px>,
    pub level: usize,
    pub level_up_cost: usize,
    pub gold: usize,
}
impl Component for LevelIndicator {
    fn render(self, ctx: &RenderCtx) {
        let Self {
            wh,
            level,
            level_up_cost,
            gold,
        } = self;
        let _game_state = use_game_state(ctx);
        let (mouse_hovering, set_mouse_hovering) = ctx.state(|| false);
        let can_upgrade = level < 10 && gold >= level_up_cost;
        let level_up = || {
            mutate_game_state(move |game_state| {
                game_state.level = game_state.level.checked_add(1).expect("Level overflow");
                game_state.spend_gold(level_up_cost);
            });
        };
        ctx.compose(|ctx| {
            table::horizontal([
                table::fixed(48.px(), |wh, ctx| {
                    ctx.add(Icon::new(IconKind::Level).size(IconSize::Large).wh(wh));
                }),
                table::fixed(32.px(), |wh, ctx| {
                    ctx.add(
                        headline(format!("{level}",))
                            .size(crate::theme::typography::FontSize::Medium)
                            .align(crate::theme::typography::TextAlign::Center { wh })
                            .build(),
                    );
                }),
                table::ratio(1, |_, _| {}),
                table::fixed(
                    128.px(),
                    table::padding(PADDING, |wh, ctx| {
                        ctx.add(
                            Button::new(
                                wh,
                                &|| {
                                    if !can_upgrade {
                                        return;
                                    }
                                    level_up();
                                },
                                &|wh, _text_color, ctx| {
                                    let text_color = match can_upgrade {
                                        true => palette::ON_PRIMARY,
                                        false => palette::ON_SURFACE,
                                    };
                                    ctx.add(
                                        headline(format!(
                                            "{} {}{level_up_cost}",
                                            Icon::new(IconKind::Level)
                                                .attributes(vec![IconAttribute {
                                                    icon_kind: IconKind::Up,
                                                    position: crate::icon::IconAttributePosition::BottomRight
                                                }])
                                                .wh(Wh::single(wh.height))
                                                .as_tag(),
                                            Icon::new(IconKind::Gold)
                                                .wh(Wh::single(wh.height))
                                                .as_tag(),
                                        ))
                                        .align(crate::theme::typography::TextAlign::Center { wh })
                                        .color(text_color)
                                        .build_rich(),
                                    );
                                },
                            )
                            .variant(crate::theme::button::ButtonVariant::Contained)
                            .color(crate::theme::button::ButtonColor::Primary)
                            .disabled(!can_upgrade),
                        );
                    }),
                ),
            ])(wh, ctx);
        })
        .attach_event(|event| {
            let Event::MouseMove { event } = event else {
                return;
            };
            let mouse_move_is_local_xy_in = event.is_local_xy_in();
            if *mouse_hovering != mouse_move_is_local_xy_in {
                set_mouse_hovering.set(mouse_move_is_local_xy_in);
            }
        });
        ctx.compose(|ctx| {
            if !*mouse_hovering {
                return;
            }
            ctx.translate((0.px(), wh.height)).on_top().add(
                crate::top_bar::level_up_details::LevelUpDetails {
                    width: wh.width,
                    current_level: level,
                },
            );
        });
        ctx.add(simple_rect(
            wh,
            Color::TRANSPARENT,
            0.px(),
            palette::SURFACE_CONTAINER,
        ));
    }
}
