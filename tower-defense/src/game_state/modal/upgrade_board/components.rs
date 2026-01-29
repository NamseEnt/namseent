use super::data_conversion::{UpgradeInfo, get_upgrade_infos};
use crate::{
    game_state::use_game_state,
    l10n::upgrade_board::UpgradeBoardText,
    palette,
    theme::typography::{self, FontSize},
};
use namui::*;
use namui_prebuilt::{
    list_view, simple_rect,
    table::{self, horizontal},
};

const SCROLL_BAR_WIDTH: Px = px(4.0);
const TITLE_HEIGHT: Px = px(36.0);
const PADDING: Px = px(4.0);
const UPGRADE_BOARD_WH: Wh<Px> = Wh {
    width: px(640.0),
    height: px(480.0),
};
const ITEM_HEIGHT: Px = px(48.0);

pub struct UpgradeBoardModal;

impl Component for UpgradeBoardModal {
    fn render(self, ctx: &namui::RenderCtx) {
        let screen_wh = screen::size().into_type::<Px>();

        ctx.compose(|ctx| {
            let offset = ((screen_wh - UPGRADE_BOARD_WH) * 0.5).to_xy();

            ctx.translate(offset).add(UpgradeBoard {});

            ctx.add(
                simple_rect(
                    screen_wh,
                    Color::TRANSPARENT,
                    0.px(),
                    Color::from_u8(0, 0, 0, 128),
                )
                .attach_event(|event| match event {
                    Event::MouseDown { event }
                    | Event::MouseMove { event }
                    | Event::MouseUp { event } => {
                        event.stop_propagation();
                    }
                    Event::Wheel { event } => {
                        event.stop_propagation();
                    }
                    _ => {}
                }),
            );
        });
    }
}

pub struct UpgradeBoard {}

impl Component for UpgradeBoard {
    fn render(self, ctx: &namui::RenderCtx) {
        let game_state = use_game_state(ctx);
        let upgrade_infos = get_upgrade_infos(&game_state.upgrade_state, &game_state.text());

        ctx.compose(|ctx| {
            table::padding(
                PADDING,
                table::vertical([
                    table::fixed(TITLE_HEIGHT, |wh, ctx| {
                        let title = game_state
                            .text()
                            .upgrade_board(UpgradeBoardText::Title)
                            .to_string();
                        ctx.add(
                            typography::headline()
                                .size(FontSize::Large)
                                .max_width(wh.width)
                                .text(&title)
                                .render_center(wh),
                        );
                    }),
                    table::ratio(1, |wh, ctx| {
                        let item_wh = Wh {
                            width: wh.width - SCROLL_BAR_WIDTH,
                            height: ITEM_HEIGHT,
                        };
                        ctx.add(list_view::AutoListView {
                            height: wh.height,
                            scroll_bar_width: SCROLL_BAR_WIDTH,
                            item_wh,
                            items: upgrade_infos.into_iter().enumerate().map(
                                |(index, upgrade_info)| {
                                    (
                                        index,
                                        UpgradeItem {
                                            wh: item_wh,
                                            upgrade_info,
                                        },
                                    )
                                },
                            ),
                        });
                    }),
                ]),
            )(UPGRADE_BOARD_WH, ctx);
        });

        ctx.add(rect(RectParam {
            rect: UPGRADE_BOARD_WH.to_rect(),
            style: RectStyle {
                stroke: Some(RectStroke {
                    color: palette::OUTLINE,
                    width: 1.px(),
                    border_position: BorderPosition::Inside,
                }),
                fill: Some(RectFill {
                    color: palette::SURFACE_CONTAINER,
                }),
                round: Some(RectRound {
                    radius: palette::ROUND,
                }),
            },
        }));
    }
}

struct UpgradeItem {
    wh: Wh<Px>,
    upgrade_info: UpgradeInfo,
}

impl Component for UpgradeItem {
    fn render(self, ctx: &RenderCtx) {
        let Self { wh, upgrade_info } = self;

        ctx.compose(|ctx| {
            table::padding(PADDING, |wh, ctx| {
                ctx.compose(|ctx| {
                    horizontal([
                        table::fixed(
                            wh.height,
                            table::padding(PADDING, |wh, ctx| {
                                ctx.add(upgrade_info.upgrade_kind.thumbnail(wh));
                            }),
                        ),
                        table::fixed(PADDING, |_, _| {}),
                        table::ratio(
                            1,
                            table::padding(PADDING, |wh, ctx| {
                                ctx.add(
                                    typography::paragraph()
                                        .size(FontSize::Medium)
                                        .max_width(wh.width)
                                        .text(&upgrade_info.description)
                                        .render_left_top(),
                                );
                            }),
                        ),
                    ])(wh, ctx);
                });
                ctx.add(simple_rect(
                    wh,
                    palette::OUTLINE,
                    1.px(),
                    palette::SURFACE_CONTAINER_HIGH,
                ));
            })(wh, ctx);
        });
    }
}
