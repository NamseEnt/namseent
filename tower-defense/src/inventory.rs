use crate::{
    game_state::{item::use_item, mutate_game_state, use_game_state},
    sound,
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

pub struct Inventory {
    pub wh: Wh<Px>,
}

impl Component for Inventory {
    fn render(self, render_ctx: &RenderCtx) {
        let wh = self.wh;
        let game_state = use_game_state(render_ctx);

        let scroll_view = |wh: Wh<Px>, ctx: ComposeCtx| {
            ctx.add(AutoScrollViewWithCtx {
                wh,
                scroll_bar_width: PADDING,
                content: |mut ctx| {
                    for (item_index, item) in game_state.items.iter().enumerate() {
                        ctx.add(InventoryItem {
                            item,
                            index: item_index,
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
}

impl Component for InventoryItem<'_> {
    fn render(self, ctx: &RenderCtx) {
        let Self { item, index } = self;

        let (hovering, set_hovering) = ctx.state(|| false);
        let (hover_start, set_hover_start) = ctx.state(|| None::<Instant>);
        let (tooltip_id, _) = ctx.state(crate::tooltip::TooltipId::new);

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
                    .add(item.thumbnail_with_shadow(
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
                                if !*hovering {
                                    set_hovering.set(true);
                                    let origin = event.global_xy - event.local_xy();
                                    crate::tooltip::show_tooltip(
                                        *tooltip_id,
                                        Rect::from_xy_wh(origin, item_wh),
                                        crate::tooltip::TooltipPlacement::LeftOf,
                                        crate::tooltip::TooltipContent::Item(item.clone()),
                                    );
                                }
                            } else if *hovering {
                                set_hovering.set(false);
                                crate::tooltip::hide_tooltip(*tooltip_id);
                            }
                        }
                        Event::MouseDown { event } if event.is_local_xy_in() => {
                            sound::emit_sound(sound::EmitSoundParams::one_shot(
                                sound::random_small_button(),
                                sound::SoundGroup::Ui,
                                sound::VolumePreset::Medium,
                                sound::SpatialMode::NonSpatial,
                            ));
                            mutate_game_state(move |game_state| {
                                let item = game_state.items.remove(index);
                                use_item(game_state, &item);
                            });
                            event.stop_propagation();
                        }
                        _ => {}
                    },
                ),
            );
    }
}

