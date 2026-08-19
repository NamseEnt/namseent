use crate::animation::xy_with_spring;
use crate::game_state::modal::deck::{DeckKind, DeckModal};
use crate::game_state::{UserModal, set_modal};
use crate::icon::IconKind;
use crate::l10n::ui::FabTooltipText;
use crate::theme::button::{Button, ButtonVariant};
use crate::theme::typography::{FontSize, memoized_text};
use crate::thumbnail::{ThumbnailRenderOptions, ThumbnailSource, render_thumbnail};
use crate::tooltip::{TooltipContent, TooltipPlacement, WithHoverArea};
use namui::*;

const BUTTON_SIZE: Px = px(72.0);
const BUTTON_PADDING: Px = px(8.0);
const BUTTON_GAP: Px = px(12.0);
const SCREEN_MARGIN: Px = px(36.0);

pub(super) struct DeckPileButtons {
    pub screen_wh: Wh<Px>,
    pub visible: bool,
    pub draw_count: usize,
    pub discard_count: usize,
}

impl Component for DeckPileButtons {
    fn render(self, ctx: &RenderCtx) {
        let Self {
            screen_wh,
            visible,
            draw_count,
            discard_count,
        } = self;
        let stack_height = BUTTON_SIZE * 2.0 + BUTTON_GAP;
        let target_xy = Xy::new(
            SCREEN_MARGIN,
            screen_wh.height - SCREEN_MARGIN - stack_height,
        );
        let hidden_xy = Xy::new(-BUTTON_SIZE - SCREEN_MARGIN, target_xy.y);
        let animated_xy =
            xy_with_spring(ctx, if visible { target_xy } else { hidden_xy }, hidden_xy);

        ctx.absolute(animated_xy).compose(|ctx| {
            ctx.add(DeckPileButton {
                kind: DeckKind::Draw,
                visible,
                count: draw_count,
            });
            ctx.translate((0.px(), BUTTON_SIZE + BUTTON_GAP))
                .add(DeckPileButton {
                    kind: DeckKind::Discard,
                    visible,
                    count: discard_count,
                });
        });
    }
}

struct DeckPileButton {
    kind: DeckKind,
    visible: bool,
    count: usize,
}

impl Component for DeckPileButton {
    fn render(self, ctx: &RenderCtx) {
        let Self {
            kind,
            visible,
            count,
        } = self;
        let (hover_start, set_hover_start) = ctx.state(|| None::<Instant>);
        let hover_rotation = if let Some(start) = *hover_start {
            ((Instant::now() - start).as_secs_f32() * 25.0).sin() * 3.0
        } else {
            0.0
        };
        let enabled = visible && count > 0;

        let button_kind = kind.clone();
        let on_click = move || {
            if !enabled {
                return;
            }
            set_modal(Some(UserModal::Deck(DeckModal {
                deck_kind: button_kind.clone(),
                selection: None,
            })));
        };

        let thumbnail = move |wh: Wh<Px>, _color: Color, ctx: &RenderCtx| {
            let thumbnail_wh = Wh::new(
                wh.width - BUTTON_PADDING * 2.0,
                wh.height - BUTTON_PADDING * 2.0,
            );
            let pivot = Xy::new(wh.width * 0.5, wh.height * 0.5);
            ctx.translate(pivot)
                .rotate(hover_rotation.deg())
                .translate(Xy::new(-pivot.x, -pivot.y))
                .compose(|ctx| {
                    ctx.add(memoized_text(
                        (&count, &wh.width, &wh.height),
                        |mut builder| {
                            builder
                                .headline()
                                .bold()
                                .size(FontSize::Custom { size: px(20.0) })
                                .color(Color::WHITE)
                                .stroke(2.px(), Color::BLACK)
                                .text(count.to_string())
                                .render_right_bottom(Wh::new(
                                    wh.width - BUTTON_PADDING,
                                    wh.height - BUTTON_PADDING,
                                ))
                        },
                    ));
                    ctx.translate((BUTTON_PADDING, BUTTON_PADDING))
                        .add(render_thumbnail(
                            ThumbnailSource::Image(IconKind::Deck.image()),
                            thumbnail_wh,
                            ThumbnailRenderOptions::sticker(
                                crate::thumbnail::STICKER_THUMBNAIL_STROKE,
                                true,
                                1.0,
                            ),
                        ));
                });
        };

        let button = Button::new(Wh::single(BUTTON_SIZE), &on_click, &thumbnail)
            .variant(ButtonVariant::Text)
            .disabled(!enabled);
        let tooltip_text = match kind {
            DeckKind::Draw => FabTooltipText::DrawPile,
            DeckKind::Discard => FabTooltipText::DiscardPile,
            DeckKind::Deck => unreachable!("deck pile buttons only use draw or discard piles"),
        };

        ctx.add(WithHoverArea {
            component_key: format!("deck-pile-button:{kind:?}:{visible}"),
            component: button,
            placement: TooltipPlacement::RightOf,
            on_enter: move || {
                if enabled {
                    set_hover_start.set(Some(Instant::now()));
                }
                visible.then_some(TooltipContent::Fab {
                    text: tooltip_text,
                    health_cost: None,
                })
            },
            on_exit: move || set_hover_start.set(None),
        });
    }
}
