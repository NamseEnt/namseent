use crate::game_state::play_history::{HistoryEvent, HistoryEventType};
use crate::game_state::use_game_state;
use crate::icon::{Icon, IconKind};
use crate::l10n;
use crate::theme::palette;
use crate::theme::typography::HeadlineBuilder;
use namui::*;
use namui_prebuilt::list_view::ListViewWithCtx;
use namui_prebuilt::{simple_rect, table};

const PADDING: Px = px(8.0);
const ITEM_HEIGHT: Px = px(60.0);
const TIMELINE_WIDTH: Px = px(60.0);
const TIMELINE_LINE_WIDTH: Px = px(4.0);
const SCROLL_BAR_WIDTH: Px = px(2.0);

pub struct EventList<'a> {
    pub wh: Wh<Px>,
    pub events: &'a [HistoryEvent],
}

impl Component for EventList<'_> {
    fn render(self, ctx: &RenderCtx) {
        let Self { wh, events } = self;

        let game_state = use_game_state(ctx);
        let (scroll_y, set_scroll_y) = ctx.state(|| 0.px());
        let item_wh = Wh {
            width: wh.width,
            height: ITEM_HEIGHT,
        };
        ctx.add(ListViewWithCtx {
            height: wh.height,
            scroll_bar_width: SCROLL_BAR_WIDTH,
            item_wh,
            items: events.iter().rev().enumerate(),
            scroll_y: *scroll_y,
            set_scroll_y,
            item_render: |event, ctx| {
                ctx.compose(|ctx| {
                    table::horizontal([
                        table::fixed(TIMELINE_WIDTH, |wh, ctx| {
                            ctx.add(TimeLineComponent { wh, event });
                        }),
                        table::ratio(
                            1,
                            table::padding(PADDING, |wh, ctx| {
                                ctx.compose(|ctx| {
                                    table::padding(PADDING, |wh, ctx| {
                                        ctx.add(
                                            HeadlineBuilder::new(game_state.text().event(
                                                l10n::event::EventText::Description(
                                                    &event.event_type,
                                                    &game_state.text().locale(),
                                                ),
                                            ))
                                            .size(crate::theme::typography::FontSize::Small)
                                            .align(
                                                crate::theme::typography::TextAlign::LeftCenter {
                                                    height: wh.height,
                                                },
                                            )
                                            .build(),
                                        );
                                    })(wh, ctx);
                                });

                                ctx.add(simple_rect(
                                    wh,
                                    Color::TRANSPARENT,
                                    0.px(),
                                    palette::SURFACE_CONTAINER_HIGH,
                                ));
                            }),
                        ),
                    ])(item_wh, ctx);
                });
            },
        });
    }
}

struct TimeLineComponent<'a> {
    wh: Wh<Px>,
    event: &'a HistoryEvent,
}
impl Component for TimeLineComponent<'_> {
    fn render(self, ctx: &RenderCtx) {
        let Self { wh, event } = self;

        let center_x = TIMELINE_WIDTH * 0.5;

        // TODO: 이벤트 타입에 따라 선 위치 변경
        let line_start_xy = match event.event_type {
            HistoryEventType::GameOver => Xy::new(center_x, wh.height * 0.5),
            _ => Xy::new(center_x, 0.px()),
        };
        let line_end_xy = match event.event_type {
            HistoryEventType::GameStart => Xy::new(center_x, wh.height * 0.5),
            _ => Xy::new(center_x, wh.height),
        };

        let icon_center_xy = Xy::new(center_x, wh.height * 0.5);
        ctx.translate(icon_center_xy).add(TimeLineIconComponent {
            wh: Wh::single(wh.width.min(wh.height)),
            event,
        });

        let line_path = Path::new()
            .move_to(line_start_xy.x, line_start_xy.y)
            .line_to(line_end_xy.x, line_end_xy.y)
            .close();
        let line_paint = Paint::new(palette::PRIMARY)
            .set_stroke_width(TIMELINE_LINE_WIDTH)
            .set_style(PaintStyle::Stroke);
        ctx.add(path(line_path, line_paint));
    }
}

struct TimeLineIconComponent<'a> {
    wh: Wh<Px>,
    event: &'a HistoryEvent,
}
impl Component for TimeLineIconComponent<'_> {
    fn render(self, ctx: &RenderCtx) {
        let Self { wh, event } = self;

        let circle_wh = match event.event_type {
            HistoryEventType::GameStart
            | HistoryEventType::GameOver
            | HistoryEventType::StageStart { .. } => wh * 0.6,
            _ => wh * 0.5,
        };
        let icon_wh = circle_wh * 0.8;
        let icon_kind = event.event_type.icon_kind();
        let draw_circle = matches!(
            event.event_type,
            HistoryEventType::GameStart
                | HistoryEventType::GameOver
                | HistoryEventType::StageStart { .. }
        );

        ctx.compose(|ctx| {
            if let Some(icon_kind) = icon_kind {
                ctx.translate(icon_wh.to_xy() * -0.5)
                    .add(
                        Icon::new(icon_kind)
                            .wh(icon_wh)
                            .size(crate::icon::IconSize::Custom {
                                size: icon_wh.width,
                            }),
                    );
            }
        });

        ctx.compose(|_ctx| {
            let HistoryEventType::StageStart { stage } = event.event_type else {
                return;
            };
            ctx.translate(wh.to_xy() * -0.5).add(
                HeadlineBuilder::new(stage.to_string())
                    .size(crate::theme::typography::FontSize::Medium)
                    .align(crate::theme::typography::TextAlign::Center { wh })
                    .color(palette::ON_PRIMARY)
                    .build(),
            );
        });

        ctx.compose(|ctx| {
            if !draw_circle {
                return;
            }
            let circle_path = Path::new().add_oval(circle_wh.to_rect());
            let circle_paint = Paint::new(palette::PRIMARY).set_style(PaintStyle::Fill);
            ctx.translate(circle_wh.to_xy() * -0.5)
                .add(path(circle_path, circle_paint));
        });
    }
}

impl HistoryEventType {
    pub fn icon_kind(&self) -> Option<IconKind> {
        match self {
            HistoryEventType::GameStart | HistoryEventType::StageStart { .. } => None,
            HistoryEventType::TowerPlaced { .. } => Some(IconKind::Card),
            HistoryEventType::DamageTaken { .. } => Some(IconKind::Health),
            HistoryEventType::ItemPurchased { .. } => Some(IconKind::Item),
            HistoryEventType::ItemUsed { .. } => Some(IconKind::Item),
            HistoryEventType::UpgradeSelected { .. } => Some(IconKind::Up),
            HistoryEventType::UpgradePurchased { .. } => Some(IconKind::Shop),
            HistoryEventType::ContractPurchased { .. } => Some(IconKind::Contract),
            HistoryEventType::GameOver => Some(IconKind::Reject),
        }
    }
}
