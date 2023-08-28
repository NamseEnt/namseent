use super::*;
use crate::{
    pages::sequence_edit_page::atom::SEQUENCE_ATOM, storage::get_project_cg_part_variant_image_url,
};
use namui_prebuilt::{
    table::hooks::TableCell,
    typography::{center_text, center_text_full_height},
    *,
};
use rpc::data::{CgFile, CgPart, CgPartVariant, PartSelectionType, ScreenCg};
use std::iter::once;

const BUTTON_HEIGHT: Px = px(32.0);
const OUTER_PADDING: Px = px(8.0);
const INNER_PADDING: Px = px(4.0);
const THUMBNAIL_WH: Wh<Px> = Wh {
    width: px(96.0),
    height: px(96.0),
};

#[namui::component]
pub struct PartPicker<'a> {
    pub wh: Wh<Px>,
    pub cg_file: &'a CgFile,
    pub project_id: Uuid,
    pub cut_id: Uuid,
    pub graphic_index: Uuid,
    pub screen_cg: &'a ScreenCg,
    pub on_event: Box<dyn 'a + Fn(Event)>,
}

pub enum Event {
    MoveInCgFileThumbnail { global_xy: Xy<Px>, name: String },
    CgChangeButtonClicked,
}

impl Component for PartPicker<'_> {
    fn render<'a>(self, ctx: &'a RenderCtx) -> RenderDone {
        let Self {
            wh,
            cg_file,
            project_id,
            cut_id,
            graphic_index,
            screen_cg,
            on_event,
        } = self;

        let cg_id = cg_file.id;

        let cg_select_button = table::hooks::horizontal_padding(INNER_PADDING, |wh, ctx| {
            ctx.add(
                simple_rect(wh, color::STROKE_NORMAL, 1.px(), Color::TRANSPARENT)
                    .with_mouse_cursor(MouseCursor::Pointer)
                    .attach_event(|event| match event {
                        namui::Event::MouseDown { event } => {
                            if event.is_local_xy_in() {
                                on_event(Event::CgChangeButtonClicked);
                            }
                        }
                        _ => {}
                    }),
            )
            .add(center_text_full_height(
                wh,
                "Change Cg",
                color::STROKE_NORMAL,
            ));
        });

        let cg_part_group_list = table::hooks::vertical(
            cg_file
                .parts
                .iter()
                .filter(|part| part.selection_type != PartSelectionType::AlwaysOn)
                .flat_map(|cg_part| {
                    render_cg_part_group(
                        wh.width,
                        cg_part,
                        project_id,
                        cg_id,
                        cut_id,
                        graphic_index,
                        screen_cg,
                        &on_event,
                    )
                }),
        );

        ctx.compose(|ctx| {
            table::hooks::padding(
                OUTER_PADDING,
                table::hooks::vertical([
                    table::hooks::fixed(BUTTON_HEIGHT, cg_select_button),
                    render_divider(BUTTON_HEIGHT),
                    table::hooks::ratio(1, |wh, ctx| {
                        ctx.add(scroll_view::AutoScrollViewWithCtx {
                            xy: Xy::zero(),
                            height: wh.height,
                            scroll_bar_width: 4.px(),
                            content: |ctx| cg_part_group_list(wh, ctx),
                        });
                    }),
                ]),
            )(wh, ctx)
        });

        ctx.done()
    }
}

fn render_cg_part_group<'a>(
    width: Px,
    cg_part: &'a CgPart,
    project_id: Uuid,
    cg_id: Uuid,
    cut_id: Uuid,
    graphic_index: Uuid,
    screen_cg: &'a ScreenCg,
    on_event: &'a (dyn 'a + Fn(Event)),
) -> Vec<TableCell<'a>> {
    let no_selection = screen_cg.part(&cg_part.name).unwrap().is_not_selected();

    let title_bar = render_title_bar(cg_part);

    let no_selection_button =
        render_no_selection_button(no_selection, cg_part, cut_id, graphic_index);

    let max_thumbnails_per_row = (width / (THUMBNAIL_WH.width)).floor() as usize;
    let chunks = cg_part.variants.chunks_exact(max_thumbnails_per_row);
    let chunk_remainder = chunks.remainder();
    let last_variant_row = table::hooks::fixed(THUMBNAIL_WH.height, {
        table::hooks::horizontal(
            chunk_remainder
                .iter()
                .map(move |variant| {
                    render_thumbnail(
                        cg_part,
                        variant,
                        project_id,
                        cg_id,
                        cut_id,
                        graphic_index,
                        screen_cg,
                        on_event,
                    )
                })
                .chain(once(no_selection_button)),
        )
    });
    let variant_rows = chunks.map(move |row| {
        table::hooks::fixed(
            THUMBNAIL_WH.height,
            table::hooks::horizontal(row.iter().map(move |variant| {
                render_thumbnail(
                    cg_part,
                    variant,
                    project_id,
                    cg_id,
                    cut_id,
                    graphic_index,
                    screen_cg,
                    on_event,
                )
            })),
        )
    });

    once(title_bar)
        .chain(variant_rows)
        .chain(once(last_variant_row))
        .chain(once(render_divider(BUTTON_HEIGHT)))
        .collect()
}

fn render_title_bar(cg_part: &CgPart) -> TableCell {
    table::hooks::fixed(BUTTON_HEIGHT, {
        table::hooks::horizontal_padding(INNER_PADDING, |wh, ctx| {
            ctx.add(render([
                center_text_full_height(wh, cg_part.name.clone(), color::STROKE_NORMAL),
                simple_rect(wh, color::STROKE_NORMAL, 1.px(), color::BACKGROUND),
            ]));
        })
    })
}

fn render_no_selection_button(
    no_selection: bool,
    cg_part: &CgPart,
    cut_id: Uuid,
    graphic_index: Uuid,
) -> TableCell {
    table::hooks::fixed(
        THUMBNAIL_WH.width,
        table::hooks::padding(INNER_PADDING, move |wh, ctx| {
            ctx.add(
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
                .attach_event(move |event| match event {
                    namui::Event::MouseDown { event } => {
                        if event.is_local_xy_in() {
                            let cg_part_name = cg_part.name.clone();
                            SEQUENCE_ATOM.mutate(move |sequence| {
                                sequence.update_cut(
                                    cut_id,
                                    CutUpdateAction::UnselectCgPart {
                                        graphic_index,
                                        cg_part_name: cg_part_name.clone(),
                                    },
                                )
                            });
                        }
                    }
                    _ => {}
                }),
            )
            .add(center_text(
                wh,
                "No Selection",
                match no_selection {
                    true => color::STROKE_SELECTED,
                    false => color::STROKE_NORMAL,
                },
                12.int_px(),
            ));
        }),
    )
}

fn render_divider<'a>(height: Px) -> TableCell<'a> {
    table::hooks::fixed(height, |_wh, _ctx| {})
}

fn render_thumbnail<'a>(
    cg_part: &'a CgPart,
    cg_part_variant: &'a CgPartVariant,
    project_id: Uuid,
    cg_id: Uuid,
    cut_id: Uuid,
    graphic_index: Uuid,
    screen_cg: &'a ScreenCg,
    on_event: &'a (dyn 'a + Fn(Event)),
) -> TableCell<'a> {
    let variant_selected = screen_cg
        .part(&cg_part.name)
        .unwrap()
        .is_variant_selected(&cg_part_variant.name);

    table::hooks::fixed(
        THUMBNAIL_WH.width,
        table::hooks::padding(INNER_PADDING, move |wh, ctx| {
            ctx.add(
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
                        let selection_type = cg_part.selection_type;
                        let cg_part_variant = cg_part_variant.clone();

                        match (variant_selected, selection_type) {
                            (_, rpc::data::PartSelectionType::AlwaysOn) => {}
                            (true, _) => {
                                if event.is_local_xy_in() {
                                    let cg_part_name = cg_part.name.clone();
                                    let cg_part_variant_name = cg_part_variant.name.clone();
                                    SEQUENCE_ATOM.mutate(move |sequence| {
                                        sequence.update_cut(
                                            cut_id,
                                            CutUpdateAction::TurnOffCgPartVariant {
                                                graphic_index,
                                                cg_part_name: cg_part_name.clone(),
                                                cg_part_variant_name: cg_part_variant_name.clone(),
                                            },
                                        )
                                    });
                                }
                            }
                            (false, _) => {
                                if event.is_local_xy_in() {
                                    let cg_part_name = cg_part.name.clone();
                                    let cg_part_variant_name = cg_part_variant.name.clone();
                                    SEQUENCE_ATOM.mutate(move |sequence| {
                                        sequence.update_cut(
                                            cut_id,
                                            CutUpdateAction::TurnOnCgPartVariant {
                                                graphic_index,
                                                cg_part_name: cg_part_name.clone(),
                                                cg_part_variant_name: cg_part_variant_name.clone(),
                                            },
                                        )
                                    });
                                }
                            }
                        };
                    }
                    namui::Event::MouseMove { event } => {
                        if event.is_local_xy_in() {
                            on_event(Event::MoveInCgFileThumbnail {
                                global_xy: event.global_xy,
                                name: cg_part_variant.name.clone(),
                            });
                        }
                    }
                    _ => {}
                }),
            )
            .add(
                simple_rect(wh, Color::TRANSPARENT, 0.px(), color::BACKGROUND)
                    .with_mouse_cursor(MouseCursor::Pointer),
            )
            .add(
                get_project_cg_part_variant_image_url(project_id, cg_id, cg_part_variant.id)
                    .map_or(RenderingTree::Empty, |cg_part_image_url| {
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
                    }),
            );
        }),
    )
}
