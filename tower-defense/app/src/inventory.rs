use crate::{
    PresentationInstant,
    game_state::use_game_state,
    hand::xy_with_spring,
    palette, sound,
    theme::paper_container::{PaperContainerBackground, PaperTexture, PaperVariant},
    thumbnail::{ThumbnailRenderOptions, render_thumbnail},
    tooltip::WithHoverArea,
};
use namui::*;
use namui_prebuilt::{scroll_view::AutoScrollViewWithCtx, simple_rect, table};

const ITEM_SIZE: Px = px(64.);
// original gap between items (vertical spacing)
const ITEM_GAP: Px = px(12.);
// half the gap becomes margin around each button
const ITEM_MARGIN: Px = px(6.);
const PADDING: Px = px(8.);

pub struct Inventory {
    pub wh: Wh<Px>,
}

impl Component for Inventory {
    fn render(self, render_ctx: &RenderCtx) {
        let wh = self.wh;
        let game_state = use_game_state(render_ctx);
        let entries = game_state.state().presentation_inventory_snapshot();
        let capacity = game_state.raw_core_state().item_capacity();
        let active_count = entries.iter().filter(|entry| !entry.is_exiting()).count();

        let scroll_view = |wh: Wh<Px>, ctx: ComposeCtx| {
            ctx.add(AutoScrollViewWithCtx {
                wh,
                scroll_bar_width: PADDING,
                content: |ctx| {
                    for entry in entries.iter() {
                        ctx.add_with_key(
                            entry.item.id.0 as u128,
                            InventoryItem {
                                item: &entry.item,
                                interactive: !entry.is_exiting(),
                                exiting: entry.is_exiting(),
                                target_xy: Xy::new(
                                    0.px(),
                                    (ITEM_SIZE + ITEM_GAP) * entry.order as f32,
                                ),
                            },
                        );
                    }
                    for slot in active_count..capacity {
                        ctx.add_with_key(
                            (1_u128 << 127) + slot as u128,
                            InventoryEmptySlot {
                                wh: Wh::new(ITEM_SIZE, ITEM_SIZE),
                                target_xy: Xy::new(0.px(), (ITEM_SIZE + ITEM_GAP) * slot as f32),
                            },
                        );
                    }
                    let content_height =
                        (ITEM_SIZE + ITEM_GAP) * capacity.max(entries.len()) as f32;
                    ctx.add(simple_rect(
                        Wh::new(wh.width, content_height),
                        Color::TRANSPARENT,
                        0.px(),
                        Color::TRANSPARENT,
                    ));
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

struct InventoryEmptySlot {
    wh: Wh<Px>,
    target_xy: Xy<Px>,
}

impl Component for InventoryEmptySlot {
    fn render(self, ctx: &RenderCtx) {
        ctx.translate(self.target_xy)
            .translate(Xy::new(ITEM_MARGIN, ITEM_MARGIN))
            .add(PaperContainerBackground {
                width: self.wh.width,
                height: self.wh.height,
                texture: PaperTexture::Rough,
                variant: PaperVariant::PaperSingleLayer,
                color: palette::SURFACE_CONTAINER_LOW,
                outline_color: Some(palette::OUTLINE),
                shadow: true,
                arrow: None,
            });
    }
}

struct InventoryItem<'a> {
    item: &'a crate::game_state::item::ItemWithId,
    interactive: bool,
    exiting: bool,
    target_xy: Xy<Px>,
}

impl Component for InventoryItem<'_> {
    fn render(self, ctx: &RenderCtx) {
        let Self {
            item,
            interactive,
            exiting,
            target_xy,
        } = self;
        let (hover_start, set_hover_start) = ctx.state(|| None::<PresentationInstant>);

        let animation_target = if exiting {
            Xy::new(target_xy.x, target_xy.y + ITEM_SIZE)
        } else {
            target_xy
        };
        let animated_xy = xy_with_spring(
            ctx,
            animation_target,
            Xy::new(target_xy.x, target_xy.y + ITEM_SIZE),
        );
        let animated_scale = xy_with_spring(
            ctx,
            if exiting {
                Xy::single(0.0)
            } else {
                Xy::single(1.0)
            },
            Xy::single(0.0),
        );
        let half_item = Wh::new(ITEM_SIZE, ITEM_SIZE).to_xy() * 0.5;
        let ctx = ctx
            .translate(animated_xy)
            .translate(half_item)
            .scale(animated_scale)
            .translate(-half_item);

        let item_wh = Wh::new(ITEM_SIZE, ITEM_SIZE);
        let inner_wh = Wh::new(
            item_wh.width - PADDING * 2.0,
            item_wh.height - PADDING * 2.0,
        );

        let hover_rotation = if let Some(start) = *hover_start {
            ((PresentationInstant::capture() - start).as_secs_f32() * 25.0).sin() * 3.0
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
                    .add(render_thumbnail(
                        item.thumbnail_source(),
                        inner_wh,
                        ThumbnailRenderOptions::sticker(
                            crate::thumbnail::STICKER_THUMBNAIL_STROKE,
                            true,
                            1.0,
                        ),
                    ));
            });

        ctx.translate(Xy::new(ITEM_MARGIN, ITEM_MARGIN))
            .mouse_cursor(MouseCursor::Standard(match interactive {
                true => StandardCursor::Pointer,
                false => StandardCursor::Default,
            }))
            .compose(|ctx| {
                let inventory_item = item.item.clone();
                let item_id = item.id;
                if interactive {
                    ctx.add(
                        WithHoverArea {
                            component_key: "item tooltip",
                            component: simple_rect(
                                item_wh,
                                Color::TRANSPARENT,
                                0.px(),
                                Color::TRANSPARENT,
                            ),
                            placement: crate::tooltip::TooltipPlacement::LeftOf,
                            on_enter: || {
                                set_hover_start.set(Some(PresentationInstant::capture()));
                                Some(crate::tooltip::TooltipContent::Item(inventory_item.clone()))
                            },
                            on_exit: move || {
                                set_hover_start.set(None);
                            },
                        }
                        .attach_event(move |event| {
                            let Event::MouseDown { event } = event else {
                                return;
                            };
                            if !event.is_local_xy_in() {
                                return;
                            }
                            sound::emit_sound(sound::EmitSoundParams::one_shot(
                                sound::random_small_button(),
                                sound::SoundGroup::Ui,
                                sound::VolumePreset::Medium,
                                sound::SpatialMode::NonSpatial,
                            ));
                            crate::game_state::dispatch_use_inventory_item(item_id);
                            event.stop_propagation();
                        }),
                    );
                } else {
                    ctx.add(simple_rect(
                        item_wh,
                        Color::TRANSPARENT,
                        0.px(),
                        Color::TRANSPARENT,
                    ));
                }
            });
    }
}
