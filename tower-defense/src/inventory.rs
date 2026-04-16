use crate::animation::with_spring;
use crate::theme::paper_container::{PaperContainerBackground, PaperTexture, PaperVariant};
use crate::{
    game_state::{item::use_item, mutate_game_state, use_game_state},
    palette,
    theme::typography::{FontSize, memoized_text},
};
use namui::*;
use namui_prebuilt::{scroll_view::AutoScrollViewWithCtx, simple_rect, table};

const ITEM_SIZE: Px = px(64.);
// original gap between items (vertical spacing)
const ITEM_GAP: Px = px(12.);
// half the gap becomes margin around each button
const ITEM_MARGIN: Px = px(6.);
const PADDING: Px = px(8.);
const INVENTORY_STICKER_THUMBNAIL_STROKE: Px = px(6.);

mod tooltip {
    use namui::*;
    pub const PADDING: Px = px(12.0);
    pub const MAX_WIDTH: Px = px(240.0);
    pub const ARROW_WIDTH: Px = px(8.0);
    pub const ARROW_HEIGHT: Px = px(16.0);
    pub const OFFSET_X: Px = px(4.0);
}

pub struct Inventory {
    pub wh: Wh<Px>,
}

impl Component for Inventory {
    fn render(self, render_ctx: &RenderCtx) {
        let wh = self.wh;
        let game_state = use_game_state(render_ctx);
        let locale = game_state.text().locale();

        let scroll_view = |wh: Wh<Px>, ctx: ComposeCtx| {
            ctx.add(AutoScrollViewWithCtx {
                wh,
                scroll_bar_width: PADDING,
                content: |mut ctx| {
                    for (item_index, item) in game_state.items.iter().enumerate() {
                        ctx.add(InventoryItem {
                            item,
                            index: item_index,
                            locale,
                        });
                        // advance by button height plus original gap
                        ctx = ctx.translate(Xy::new(0.px(), ITEM_SIZE + ITEM_GAP));
                    }
                },
            });
        };

        render_ctx.compose(|ctx| {
            table::horizontal([
                // since wh is already the fixed panel width, we render directly
                table::fixed_no_clip(wh.width, table::padding_no_clip(PADDING, scroll_view)),
            ])(wh, ctx);
        });
    }
}

struct InventoryItem<'a> {
    item: &'a crate::game_state::item::Item,
    index: usize,
    locale: crate::l10n::Locale,
}

impl Component for InventoryItem<'_> {
    fn render(self, ctx: &RenderCtx) {
        let Self {
            item,
            index,
            locale,
        } = self;

        let (hovering, set_hovering) = ctx.state(|| false);
        let (hover_start, set_hover_start) = ctx.state(|| None::<Instant>);

        let tooltip_scale = with_spring(
            ctx,
            if *hovering { 1.0 } else { 0.0 },
            0.0,
            |v| v * v,
            || 0.0,
        );

        ctx.compose(|ctx| {
            if tooltip_scale > 0.01 {
                let tooltip = ctx.ghost_add("inventory-tooltip", InventoryTooltip { item, locale });
                if let Some(tooltip_wh) = tooltip.bounding_box().map(|r| r.wh()) {
                    let total_width = tooltip_wh.width + tooltip::ARROW_WIDTH + tooltip::OFFSET_X;
                    let y = (ITEM_SIZE - tooltip_wh.height) / 2.0;

                    // pivot around right-center of tooltip so it scales from the edge near the item
                    let pivot = Xy::new(tooltip_wh.width, tooltip_wh.height / 2.0);
                    let base = Xy::new(-total_width, y);
                    ctx.translate(base + pivot)
                        .scale(Xy::new(tooltip_scale, tooltip_scale))
                        .translate(Xy::new(-pivot.x, -pivot.y))
                        .on_top()
                        .add(tooltip);
                }
            }
        });

        let item_wh = Wh::new(ITEM_SIZE, ITEM_SIZE);
        let inner_wh = Wh::new(
            item_wh.width - PADDING * 2.0,
            item_wh.height - PADDING * 2.0,
        );

        if *hovering && (*hover_start).is_none() {
            set_hover_start.set(Some(Instant::now()));
        }
        if !*hovering {
            set_hover_start.set(None);
        }

        let hover_rotation = if let Some(start) = *hover_start {
            ((Instant::now() - start).as_secs_f32() * 25.0).sin() * 3.0
        } else {
            0.0
        };

        ctx.translate(Xy::new(ITEM_MARGIN, ITEM_MARGIN))
            .compose(|ctx| {
                let pivot = Xy::new(ITEM_SIZE * 0.5, ITEM_SIZE * 0.5);
                ctx.translate(pivot)
                    .rotate(hover_rotation.deg())
                    .translate(Xy::new(-pivot.x, -pivot.y))
                    .translate(Xy::new(PADDING, PADDING))
                    .add(item.kind.thumbnail_with_shadow(
                        inner_wh,
                        INVENTORY_STICKER_THUMBNAIL_STROKE,
                        true,
                    ));
            });

        ctx.translate(Xy::new(ITEM_MARGIN, ITEM_MARGIN))
            .mouse_cursor(MouseCursor::Standard(StandardCursor::Pointer))
            .add(
                simple_rect(item_wh, Color::TRANSPARENT, 0.px(), Color::TRANSPARENT).attach_event(
                    move |event| match event {
                        Event::MouseMove { event } => {
                            if event.is_local_xy_in() {
                                set_hovering.set(true);
                            } else {
                                set_hovering.set(false);
                            }
                        }
                        Event::MouseDown { event } => {
                            if event.is_local_xy_in() {
                                mutate_game_state(move |game_state| {
                                    let item = game_state.items.remove(index);
                                    use_item(game_state, &item);
                                });
                            }
                        }
                        _ => {}
                    },
                ),
            );
    }
}

struct InventoryTooltip<'a> {
    item: &'a crate::game_state::item::Item,
    locale: crate::l10n::Locale,
}

impl Component for InventoryTooltip<'_> {
    fn render(self, ctx: &RenderCtx) {
        let InventoryTooltip { item, locale } = self;
        let name_text = item.kind.name_text();
        let name_key = format!("{:?}:name", item.kind);
        let desc_key = format!("{:?}:{:?}:desc", item.kind, item.effect);

        let max_width = tooltip::MAX_WIDTH;
        let text_max = max_width - (tooltip::PADDING * 2.0);

        let content = ctx.ghost_compose("tooltip-content", |ctx| {
            table::vertical([
                table::fit(table::FitAlign::LeftTop, |compose_ctx| {
                    compose_ctx.add(memoized_text(
                        (&name_key, &text_max, &locale.language),
                        |mut builder| {
                            builder
                                .headline()
                                .size(FontSize::Medium)
                                .max_width(text_max)
                                .l10n(name_text.clone(), &locale)
                                .render_left_top()
                        },
                    ));
                }),
                table::fixed(tooltip::PADDING, |_, _| {}),
                table::fit(table::FitAlign::LeftTop, |compose_ctx| {
                    compose_ctx.add(memoized_text(
                        (&desc_key, &text_max, &locale.language),
                        |mut builder| {
                            builder
                                .paragraph()
                                .size(FontSize::Large)
                                .max_width(text_max)
                                .l10n(item.description_text(), &locale)
                                .render_left_top()
                        },
                    ));
                }),
            ])(Wh::new(text_max, f32::MAX.px()), ctx);
        });

        let Some(content_wh) = content.bounding_box().map(|rect| rect.wh()) else {
            return;
        };

        let container_wh = content_wh + Wh::single(tooltip::PADDING * 2.0);
        ctx.translate((tooltip::PADDING, tooltip::PADDING))
            .add(content);
        Self::render_background(ctx, container_wh);
    }
}

impl InventoryTooltip<'_> {
    fn render_background(ctx: &RenderCtx, wh: Wh<Px>) {
        ctx.add(PaperContainerBackground {
            width: wh.width,
            height: wh.height,
            texture: PaperTexture::Rough,
            variant: PaperVariant::Sticky,
            color: palette::SURFACE_CONTAINER,
            shadow: true,
            arrow: Some(crate::theme::paper_container::PaperArrow {
                side: crate::theme::paper_container::ArrowSide::Right,
                width: tooltip::ARROW_WIDTH,
                height: tooltip::ARROW_HEIGHT,
                offset: wh.height / 2.0,
            }),
        });
    }
}
