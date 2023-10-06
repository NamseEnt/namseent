use crate::{color, components::tool_tip::ToolTip, storage::get_project_cg_part_variant_image_url};
use namui::prelude::*;
use namui_prebuilt::{
    scroll_view, simple_rect,
    table::hooks::*,
    typography::{center_text, center_text_full_height},
};
use rpc::data::{CgFile, CgPart, CgPartVariant, PartSelectionType, ScreenCg};
use std::{iter::once, ops::Deref};

const THUMBNAIL_WH: Wh<Px> = Wh {
    width: px(96.0),
    height: px(96.0),
};
const MIN_THUMBNAIL_CONTAINER_SIDE_PADDING: Px = px(8.0);
const ROW_VERTICAL_PADDING: Px = px(8.0);
const DIVIDER_HEIGHT: Px = px(32.0);

#[namui::component]
pub struct PartPicker<'a> {
    pub wh: Wh<Px>,
    pub cg_file: &'a CgFile,
    pub project_id: Uuid,
    pub screen_cg: &'a ScreenCg,
    pub on_event: &'a dyn Fn(Event),
}

pub enum Event {
    UnselectCgPart {
        cg_part_name: String,
    },
    TurnOnCgPartVariant {
        cg_part_name: String,
        cg_part_variant_name: String,
    },
    TurnOffCgPartVariant {
        cg_part_name: String,
        cg_part_variant_name: String,
    },
}

pub enum InternalEvent {
    MouseHoverInThumbnail { global_xy: Xy<Px>, name: String },
    MouseHoverOutThumbnail,
}

impl Component for PartPicker<'_> {
    fn render(self, ctx: &RenderCtx) -> RenderDone {
        const PADDING: Px = px(8.0);

        let Self {
            wh,
            cg_file,
            project_id,
            screen_cg,
            on_event,
        } = self;

        let (mouse_hovering_part_variant, set_mouse_hovering_part_variant) =
            ctx.state::<Option<MouseHoveringPartVariant>>(|| None);

        let on_internal_event = |internal_event| match internal_event {
            InternalEvent::MouseHoverInThumbnail { global_xy, name } => {
                set_mouse_hovering_part_variant
                    .set(Some(MouseHoveringPartVariant { name, global_xy }))
            }
            InternalEvent::MouseHoverOutThumbnail => set_mouse_hovering_part_variant.set(None),
        };

        ctx.compose(|ctx| {
            let Some(mouse_hovering_part_variant) = mouse_hovering_part_variant.deref() else {
                return;
            };
            ctx.add(mouse_hovering_part_variant.tooltip());
        });

        ctx.compose(|ctx| {
            padding(PADDING, |wh, ctx| {
                ctx.add(CgPartList {
                    wh,
                    cg_file,
                    project_id,
                    screen_cg,
                    on_event,
                    on_internal_event: &on_internal_event,
                });
            })(wh, ctx);
        });

        ctx.component(
            simple_rect(wh, Color::TRANSPARENT, 0.px(), Color::TRANSPARENT).attach_event(|event| {
                if let namui::Event::MouseMove { event: _event } = event {
                    if mouse_hovering_part_variant.is_none() {
                        return;
                    }
                    on_internal_event(InternalEvent::MouseHoverOutThumbnail);
                }
            }),
        );

        ctx.done()
    }
}

#[component]
struct CgPartList<'a> {
    wh: Wh<Px>,
    cg_file: &'a CgFile,
    project_id: Uuid,
    screen_cg: &'a ScreenCg,
    on_event: &'a dyn Fn(Event),
    on_internal_event: &'a dyn Fn(InternalEvent),
}
impl Component for CgPartList<'_> {
    fn render(self, ctx: &RenderCtx) -> RenderDone {
        let Self {
            wh,
            cg_file,
            project_id,
            screen_cg,
            on_event,
            on_internal_event,
        } = self;

        let wh = ctx.track_eq(&wh);
        let max_thumbnails_per_row = ctx.memo(|| {
            (wh.width / (THUMBNAIL_WH.width + MIN_THUMBNAIL_CONTAINER_SIDE_PADDING * 2.0)).floor()
                as usize
        });
        let thumbnail_container_side_padding = ctx.memo(|| {
            (wh.width - THUMBNAIL_WH.width * *max_thumbnails_per_row)
                / (*max_thumbnails_per_row as f32 * 2.0)
        });
        let thumbnail_container_wh = ctx.memo(|| Wh {
            width: (THUMBNAIL_WH.width + *thumbnail_container_side_padding * 2.0),
            height: THUMBNAIL_WH.height,
        });
        let row_height = ctx.memo(|| thumbnail_container_wh.height + ROW_VERTICAL_PADDING * 2.0);

        ctx.component(scroll_view::AutoScrollViewWithCtx {
            wh: *wh,
            scroll_bar_width: 4.px(),
            content: |ctx| {
                vertical(
                    cg_file
                        .parts
                        .iter()
                        .filter(|part| part.selection_type != PartSelectionType::AlwaysOn)
                        .map(|cg_part| {
                            fit(
                                FitAlign::CenterMiddle,
                                CgPartGroup {
                                    width: wh.width,
                                    cg_part,
                                    project_id,
                                    cg_id: cg_file.id,
                                    screen_cg,
                                    on_event,
                                    on_internal_event: &on_internal_event,
                                    row_height: *row_height,
                                    thumbnail_container_wh: *thumbnail_container_wh,
                                    max_thumbnails_per_row: *max_thumbnails_per_row,
                                    thumbnail_container_side_padding:
                                        *thumbnail_container_side_padding,
                                },
                            )
                        }),
                )(Wh::new(wh.width, 100.px()), ctx);
            },
        });

        ctx.done()
    }
}

#[component]
struct CgPartGroup<'a> {
    width: Px,
    cg_part: &'a CgPart,
    project_id: Uuid,
    cg_id: Uuid,
    screen_cg: &'a ScreenCg,
    on_event: &'a dyn Fn(Event),
    on_internal_event: &'a dyn Fn(InternalEvent),
    row_height: Px,
    thumbnail_container_wh: Wh<Px>,
    max_thumbnails_per_row: usize,
    thumbnail_container_side_padding: Px,
}
impl Component for CgPartGroup<'_> {
    fn render(self, ctx: &RenderCtx) -> RenderDone {
        let Self {
            width,
            cg_part,
            project_id,
            cg_id,
            screen_cg,
            on_event,
            on_internal_event,

            row_height,
            thumbnail_container_wh,
            max_thumbnails_per_row,
            thumbnail_container_side_padding,
        } = self;

        let no_selection = screen_cg.part(&cg_part.name).unwrap().is_not_selected();

        let title_bar = render_title_bar(cg_part);

        let no_selection_button = fixed(thumbnail_container_wh.width, |wh, ctx| {
            horizontal_padding(thumbnail_container_side_padding, |wh, ctx| {
                ctx.add(NoSelectionButton {
                    wh,
                    no_selection,
                    cg_part,
                    on_event,
                });
            })(wh, ctx)
        });

        let chunks = cg_part.variants.chunks_exact(max_thumbnails_per_row);
        let chunk_remainder = chunks.remainder();
        let last_variant_row = fixed(
            row_height,
            vertical_padding(
                ROW_VERTICAL_PADDING,
                horizontal(
                    chunk_remainder
                        .iter()
                        .map(|variant| {
                            fixed(thumbnail_container_wh.width, |wh, ctx| {
                                horizontal_padding(thumbnail_container_side_padding, |wh, ctx| {
                                    ctx.add(Thumbnail {
                                        wh,
                                        cg_part,
                                        cg_part_variant: variant,
                                        project_id,
                                        cg_id,
                                        screen_cg,
                                        on_event,
                                        on_internal_event,
                                    });
                                })(wh, ctx);
                            })
                        })
                        .chain(once(no_selection_button)),
                ),
            ),
        );
        let variant_rows = chunks.map(|row| {
            fixed(
                row_height,
                vertical_padding(
                    ROW_VERTICAL_PADDING,
                    horizontal(row.iter().map(|variant| {
                        fixed(thumbnail_container_wh.width, |wh, ctx| {
                            horizontal_padding(thumbnail_container_side_padding, |wh, ctx| {
                                ctx.add(Thumbnail {
                                    wh,
                                    cg_part,
                                    cg_part_variant: variant,
                                    project_id,
                                    cg_id,
                                    screen_cg,
                                    on_event,
                                    on_internal_event,
                                });
                            })(wh, ctx);
                        })
                    })),
                ),
            )
        });

        let items = once(title_bar)
            .chain(variant_rows)
            .chain(once(last_variant_row))
            .chain(once(render_divider(DIVIDER_HEIGHT)));

        ctx.compose(|ctx| vertical(items)(Wh::new(width, 0.px()), ctx));

        ctx.done()
    }
}

fn render_title_bar(cg_part: &CgPart) -> TableCell {
    const HEIGHT: Px = px(32.0);
    const PADDING: Px = px(8.0);

    fixed(HEIGHT, {
        horizontal_padding(PADDING, |wh, ctx| {
            ctx.add(render([
                center_text_full_height(wh, cg_part.name.clone(), color::STROKE_NORMAL),
                simple_rect(wh, color::STROKE_NORMAL, 1.px(), color::BACKGROUND),
            ]));
        })
    })
}

#[component]
struct NoSelectionButton<'a> {
    wh: Wh<Px>,
    no_selection: bool,
    cg_part: &'a CgPart,
    on_event: &'a dyn Fn(Event),
}
impl Component for NoSelectionButton<'_> {
    fn render(self, ctx: &RenderCtx) -> RenderDone {
        let Self {
            wh,
            no_selection,
            cg_part,
            on_event,
        } = self;

        ctx.component(
            simple_rect(
                wh,
                match no_selection {
                    true => color::STROKE_SELECTED,
                    false => color::STROKE_NORMAL,
                },
                match no_selection {
                    true => 3.px(),
                    false => 1.px(),
                },
                Color::TRANSPARENT,
            )
            .with_mouse_cursor(MouseCursor::Pointer)
            .attach_event(move |event| {
                if let namui::Event::MouseDown { event } = event {
                    if event.is_local_xy_in() {
                        let cg_part_name = cg_part.name.clone();
                        on_event(Event::UnselectCgPart { cg_part_name })
                    }
                }
            }),
        )
        .component(center_text(
            wh,
            "No Selection",
            match no_selection {
                true => color::STROKE_SELECTED,
                false => color::STROKE_NORMAL,
            },
            12.int_px(),
        ))
        .done()
    }
}

fn render_divider<'a>(height: Px) -> TableCell<'a> {
    fixed(height, |wh, ctx| {
        ctx.add(simple_rect(
            wh,
            Color::TRANSPARENT,
            0.px(),
            Color::TRANSPARENT,
        ));
    })
}

#[component]
struct Thumbnail<'a> {
    wh: Wh<Px>,
    cg_part: &'a CgPart,
    cg_part_variant: &'a CgPartVariant,
    project_id: Uuid,
    cg_id: Uuid,
    screen_cg: &'a ScreenCg,
    on_event: &'a dyn Fn(Event),
    on_internal_event: &'a dyn Fn(InternalEvent),
}
impl Component for Thumbnail<'_> {
    fn render(self, ctx: &RenderCtx) -> RenderDone {
        let Self {
            wh,
            cg_part,
            cg_part_variant,
            project_id,
            cg_id,
            screen_cg,
            on_event,
            on_internal_event,
        } = self;

        let variant_selected = screen_cg
            .part(&cg_part.name)
            .unwrap()
            .is_variant_selected(&cg_part_variant.name);

        ctx.component(
            simple_rect(
                wh,
                match variant_selected {
                    true => color::STROKE_SELECTED,
                    false => color::STROKE_NORMAL,
                },
                match variant_selected {
                    true => 3.px(),
                    false => 1.px(),
                },
                Color::TRANSPARENT,
            )
            .attach_event(|event| match event {
                namui::Event::MouseDown { event } => {
                    if !event.is_local_xy_in() {
                        return;
                    }

                    let cg_part_name = cg_part.name.clone();
                    let cg_part_variant_name = cg_part_variant.name.clone();

                    match variant_selected {
                        true => on_event(Event::TurnOffCgPartVariant {
                            cg_part_name,
                            cg_part_variant_name,
                        }),
                        false => on_event(Event::TurnOnCgPartVariant {
                            cg_part_name,
                            cg_part_variant_name,
                        }),
                    };
                }
                namui::Event::MouseMove { event } => {
                    if event.is_local_xy_in() {
                        event.stop_propagation();
                        on_internal_event(InternalEvent::MouseHoverInThumbnail {
                            global_xy: event.global_xy,
                            name: cg_part_variant.name.clone(),
                        });
                    }
                }
                _ => {}
            })
            .with_mouse_cursor(MouseCursor::Pointer),
        );

        ctx.component(
            get_project_cg_part_variant_image_url(project_id, cg_id, cg_part_variant.id).map_or(
                RenderingTree::Empty,
                |cg_part_image_url| {
                    image(ImageParam {
                        rect: Rect::from_xy_wh(Xy::zero(), wh),
                        source: ImageSource::Url {
                            url: cg_part_image_url,
                        },
                        style: ImageStyle {
                            fit: ImageFit::Contain,
                            paint: None,
                        },
                    })
                },
            ),
        );

        ctx.done()
    }
}

#[derive(Debug)]
struct MouseHoveringPartVariant {
    name: String,
    global_xy: Xy<Px>,
}
impl MouseHoveringPartVariant {
    fn tooltip(&self) -> ToolTip {
        ToolTip {
            global_xy: self.global_xy,
            text: self.name.clone(),
        }
    }
}
