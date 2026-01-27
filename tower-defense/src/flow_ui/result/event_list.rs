use crate::game_state::play_history::{HistoryEvent, HistoryEventType};
use crate::game_state::use_game_state;
use crate::icon::{Icon, IconKind};
use crate::l10n;
use crate::theme::palette;
use crate::theme::typography::{self};
use namui::*;
use namui_prebuilt::list_view::ListViewWithCtx;
use namui_prebuilt::{simple_rect, table};

// Layout constants
const PADDING: Px = px(8.0);
const ITEM_HEIGHT: Px = px(60.0);
const SCROLL_BAR_WIDTH: Px = px(2.0);

// Timeline constants
mod timeline {
    use namui::*;
    pub const WIDTH: Px = px(60.0);
    pub const LINE_WIDTH: Px = px(4.0);
    pub const CENTER_OFFSET: f32 = 0.5;
}

// Tooltip constants
mod tooltip {
    use namui::*;
    pub const MAX_WIDTH: Px = px(320.0);
    pub const PADDING: Px = px(12.0);
    pub const OFFSET_FROM_MOUSE: Px = px(8.0);
}

pub struct EventList<'a> {
    pub wh: Wh<Px>,
    pub events: &'a [HistoryEvent],
}

impl Component for EventList<'_> {
    fn render(self, ctx: &RenderCtx) {
        let Self { wh, events } = self;

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
                ctx.add(EventItem { wh: item_wh, event });
            },
        });
    }
}

struct EventItem<'a> {
    wh: Wh<Px>,
    event: &'a HistoryEvent,
}

impl Component for EventItem<'_> {
    fn render(self, ctx: &RenderCtx) {
        let Self { wh, event } = self;

        let game_state = use_game_state(ctx);
        let (mouse_hovering, set_mouse_hovering) = ctx.state::<bool>(|| false);
        let (mouse_xy, set_mouse_xy) = ctx.state(|| Xy::new(0.px(), 0.px()));

        let event_text = game_state.text().event(l10n::event::EventText::Description(
            &event.event_type,
            &game_state.text().locale(),
        ));

        // Render tooltip on hover
        Self::render_tooltip(ctx, *mouse_hovering, *mouse_xy, &event_text);

        // Render main content
        Self::render_content(ctx, wh, event, &event_text);

        // Attach event handlers
        Self::attach_event_handlers(ctx, wh, set_mouse_hovering, set_mouse_xy);
    }
}

impl EventItem<'_> {
    fn render_tooltip(ctx: &RenderCtx, hovering: bool, mouse_xy: Xy<Px>, content: &str) {
        ctx.compose(|ctx| {
            if !hovering {
                return;
            }
            let tooltip = ctx.ghost_add(
                "event-tooltip",
                EventTooltip {
                    content: content.to_string(),
                    max_width: tooltip::MAX_WIDTH,
                },
            );
            let Some(tooltip_wh) = tooltip.bounding_box().map(|rect| rect.wh()) else {
                return;
            };
            if tooltip_wh.height == 0.px() {
                return;
            }

            let tooltip_x = mouse_xy.x + tooltip::OFFSET_FROM_MOUSE;
            let tooltip_y = mouse_xy.y + tooltip::OFFSET_FROM_MOUSE;

            ctx.translate((tooltip_x, tooltip_y)).on_top().add(tooltip);
        });
    }

    fn render_content(ctx: &RenderCtx, wh: Wh<Px>, event: &HistoryEvent, event_text: &str) {
        ctx.compose(|ctx| {
            table::horizontal([
                table::fixed(timeline::WIDTH, |wh, ctx| {
                    ctx.add(TimelineComponent { wh, event });
                }),
                table::ratio(
                    1,
                    table::padding(PADDING, |wh, ctx| {
                        Self::render_event_description(&ctx, wh, event_text);
                    }),
                ),
            ])(wh, ctx);
        });
    }

    fn render_event_description(ctx: &ComposeCtx, wh: Wh<Px>, event_text: &str) {
        ctx.compose(|ctx| {
            table::padding(PADDING, |wh, ctx| {
                ctx.add(
                    typography::headline()
                        .text(event_text)
                        .size(typography::FontSize::Small)
                        .left_center(wh.height),
                );
            })(wh, ctx);
        });

        ctx.add(simple_rect(
            wh,
            Color::TRANSPARENT,
            0.px(),
            palette::SURFACE_CONTAINER_HIGH,
        ));
    }

    fn attach_event_handlers(
        ctx: &RenderCtx,
        wh: Wh<Px>,
        set_mouse_hovering: SetState<bool>,
        set_mouse_xy: SetState<Xy<Px>>,
    ) {
        ctx.add(
            simple_rect(wh, Color::TRANSPARENT, 0.px(), Color::TRANSPARENT).attach_event(
                move |event| {
                    let Event::MouseMove { event } = event else {
                        return;
                    };

                    if event.is_local_xy_in() {
                        set_mouse_hovering.set(true);
                        set_mouse_xy.set(event.local_xy());
                    } else {
                        set_mouse_hovering.set(false);
                    }
                },
            ),
        );
    }
}

struct EventTooltip {
    content: String,
    max_width: Px,
}

impl Component for EventTooltip {
    fn render(self, ctx: &RenderCtx) {
        let EventTooltip { content, max_width } = self;
        let text_max_width = max_width - (tooltip::PADDING * 2.0);

        let text = ctx.ghost_add(
            "tooltip-text",
            typography::paragraph()
                .text(&content)
                .size(typography::FontSize::Small)
                .max_width(text_max_width)
                .left_top(),
        );

        let Some(text_wh) = text.bounding_box().map(|rect| rect.wh()) else {
            return;
        };

        if text_wh.height == 0.px() {
            return;
        }

        let container_wh = Wh::new(
            text_wh.width + (tooltip::PADDING * 2.0),
            text_wh.height + (tooltip::PADDING * 2.0),
        );

        ctx.translate((tooltip::PADDING, tooltip::PADDING))
            .add(text);
        Self::render_background(ctx, container_wh);
    }
}

impl EventTooltip {
    fn render_background(ctx: &RenderCtx, wh: Wh<Px>) {
        ctx.add(rect(RectParam {
            rect: wh.to_rect(),
            style: RectStyle {
                stroke: Some(RectStroke {
                    color: palette::OUTLINE,
                    width: 1.px(),
                    border_position: BorderPosition::Inside,
                }),
                fill: Some(RectFill {
                    color: palette::SURFACE,
                }),
                round: Some(RectRound {
                    radius: palette::ROUND,
                }),
            },
        }));
    }
}

struct TimelineComponent<'a> {
    wh: Wh<Px>,
    event: &'a HistoryEvent,
}

impl Component for TimelineComponent<'_> {
    fn render(self, ctx: &RenderCtx) {
        let Self { wh, event } = self;
        let center_x = timeline::WIDTH * timeline::CENTER_OFFSET;

        let (line_start_xy, line_end_xy) =
            Self::calculate_line_positions(center_x, wh.height, &event.event_type);

        let icon_center_xy = Xy::new(center_x, wh.height * 0.5);
        ctx.translate(icon_center_xy).add(TimelineIconComponent {
            wh: Wh::single(wh.width.min(wh.height)),
            event,
        });

        Self::render_line(ctx, line_start_xy, line_end_xy);
    }
}

impl TimelineComponent<'_> {
    fn calculate_line_positions(
        center_x: Px,
        height: Px,
        event_type: &HistoryEventType,
    ) -> (Xy<Px>, Xy<Px>) {
        let line_start = match *event_type {
            HistoryEventType::GameOver => Xy::new(center_x, height * 0.5),
            _ => Xy::new(center_x, 0.px()),
        };
        let line_end = match *event_type {
            HistoryEventType::GameStart => Xy::new(center_x, height * 0.5),
            _ => Xy::new(center_x, height),
        };
        (line_start, line_end)
    }

    fn render_line(ctx: &RenderCtx, start: Xy<Px>, end: Xy<Px>) {
        let line_path = Path::new()
            .move_to(start.x, start.y)
            .line_to(end.x, end.y)
            .close();
        let line_paint = Paint::new(palette::PRIMARY)
            .set_stroke_width(timeline::LINE_WIDTH)
            .set_style(PaintStyle::Stroke);
        ctx.add(path(line_path, line_paint));
    }
}

struct TimelineIconComponent<'a> {
    wh: Wh<Px>,
    event: &'a HistoryEvent,
}

impl Component for TimelineIconComponent<'_> {
    fn render(self, ctx: &RenderCtx) {
        let Self { wh, event } = self;
        let event_type = &event.event_type;

        let circle_wh = Self::calculate_circle_size(wh, event_type);
        let icon_wh = circle_wh * 0.8;

        // Render icon
        if let Some(icon_kind) = Self::get_icon_kind(event_type) {
            ctx.translate(icon_wh.to_xy() * -0.5)
                .add(
                    Icon::new(icon_kind)
                        .wh(icon_wh)
                        .size(crate::icon::IconSize::Custom {
                            size: icon_wh.width,
                        }),
                );
        }

        // Render stage label if applicable
        if let HistoryEventType::StageStart { stage } = event_type {
            ctx.translate(wh.to_xy() * -0.5).add(
                typography::headline()
                    .text(stage.to_string())
                    .size(typography::FontSize::Medium)
                    .color(palette::ON_PRIMARY)
                    .center(wh),
            );
        }

        // Render circle background if needed
        if Self::should_draw_circle(event_type) {
            Self::render_circle(ctx, circle_wh);
        }
    }
}

impl TimelineIconComponent<'_> {
    fn calculate_circle_size(wh: Wh<Px>, event_type: &HistoryEventType) -> Wh<Px> {
        let size_factor = if Self::should_draw_circle(event_type) {
            0.6
        } else {
            0.5
        };
        wh * size_factor
    }

    fn should_draw_circle(event_type: &HistoryEventType) -> bool {
        matches!(
            *event_type,
            HistoryEventType::GameStart
                | HistoryEventType::GameOver
                | HistoryEventType::StageStart { .. }
        )
    }

    fn get_icon_kind(event_type: &HistoryEventType) -> Option<IconKind> {
        event_type.icon_kind()
    }

    fn render_circle(ctx: &RenderCtx, wh: Wh<Px>) {
        let circle_path = Path::new().add_oval(wh.to_rect());
        let circle_paint = Paint::new(palette::PRIMARY).set_style(PaintStyle::Fill);
        ctx.translate(wh.to_xy() * -0.5)
            .add(path(circle_path, circle_paint));
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
