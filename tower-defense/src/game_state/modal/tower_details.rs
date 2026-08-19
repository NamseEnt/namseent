use super::card_grid::Cards;
use crate::card::Card;
use crate::game_state::set_modal;
use crate::game_state::use_game_state;
use crate::icon::IconKind;
use crate::l10n::ui::TowerDetailsModalText;
use crate::theme::fab::{FabPosition, FabSide, FabVerticalPosition, FloatingActionButton};
use crate::theme::typography::{FontSize, memoized_text};
use namui::*;
use namui_prebuilt::{scroll_view::AutoScrollViewWithCtx, simple_rect};

const CARD_VIEW_WIDTH: Px = px(540.0);
const VERTICAL_MARGIN: Px = px(128.0);
const SCROLL_BAR_WIDTH: Px = px(8.0);
const PADDING: Px = px(36.0);

#[derive(Debug, Clone, State)]
pub struct TowerDetailsModal {
    pub cards: Vec<Card>,
}

impl Component for TowerDetailsModal {
    fn render(self, ctx: &RenderCtx) {
        let Self { cards } = self;
        let game_state = use_game_state(ctx);
        let screen_wh = screen::size().into_type::<Px>();
        let title = game_state
            .text()
            .tower_details_modal(TowerDetailsModalText::Title)
            .to_string();

        ctx.translate((PADDING, PADDING))
            .add(memoized_text(&title, |mut builder| {
                builder
                    .headline()
                    .bold()
                    .color(Color::WHITE)
                    .stroke(2.px(), Color::BLACK)
                    .size(FontSize::Large)
                    .text(title.clone())
                    .render_left_top()
            }));

        let close = || set_modal(None);
        ctx.compose(|ctx| {
            ctx.add(FloatingActionButton {
                screen_wh,
                position: FabPosition::new(FabSide::Right, FabVerticalPosition::Top),
                visible: true,
                icon: IconKind::Reject,
                disabled: false,
                long_press_time: None,
                on_click: &close,
                tooltip_content: None,
            });
        });

        ctx.add(AutoScrollViewWithCtx {
            wh: screen_wh,
            scroll_bar_width: SCROLL_BAR_WIDTH,
            content: move |ctx| {
                let card_view_x = (screen_wh.width - CARD_VIEW_WIDTH) * 0.5;
                let card_view = ctx.translate((card_view_x, VERTICAL_MARGIN)).ghost_add(
                    "tower-used-cards".to_string(),
                    Cards {
                        width: CARD_VIEW_WIDTH,
                        cards: &cards,
                        selected_card_ids: &[],
                        on_card_click: None,
                    },
                );
                let bounding_box = card_view.bounding_box().unwrap_or(Rect::Xywh {
                    x: 0.px(),
                    y: 0.px(),
                    width: CARD_VIEW_WIDTH,
                    height: 0.px(),
                });

                ctx.translate((card_view_x, VERTICAL_MARGIN)).add(card_view);
                ctx.add(simple_rect(
                    Wh::new(
                        screen_wh.width,
                        bounding_box.height() + VERTICAL_MARGIN * 2.0,
                    ),
                    Color::TRANSPARENT,
                    0.px(),
                    Color::TRANSPARENT,
                ));
            },
        })
        .attach_event(|event| match event {
            Event::MouseDown { event } | Event::MouseMove { event } | Event::MouseUp { event }
                if event.is_local_xy_in() =>
            {
                event.stop_propagation();
            }
            Event::Wheel { event } if event.is_local_xy_in() => {
                event.stop_propagation();
            }
            _ => {}
        });

        ctx.mouse_cursor(MouseCursor::Standard(StandardCursor::Default))
            .add(
                simple_rect(
                    screen_wh,
                    Color::TRANSPARENT,
                    0.px(),
                    Color::BLACK.with_alpha(180),
                )
                .attach_event(|event| match event {
                    Event::MouseDown { event }
                    | Event::MouseMove { event }
                    | Event::MouseUp { event } => {
                        event.stop_propagation();
                    }
                    _ => {}
                }),
            );
    }
}
