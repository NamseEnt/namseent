use super::*;
use crate::{color, storage::get_project_cg_part_variant_image_url};
use namui_prebuilt::{
    table::TableCell,
    typography::{center_text, center_text_full_height},
    *,
};
use rpc::data::{CgFile, CgPart, CgPartVariant, ScreenCg};
use std::iter::once;
use tooltip::*;

const BUTTON_HEIGHT: Px = px(32.0);
const OUTER_PADDING: Px = px(8.0);
const INNER_PADDING: Px = px(4.0);
const THUMBNAIL_WH: Wh<Px> = Wh {
    width: px(96.0),
    height: px(96.0),
};

impl CharacterEditor {
    pub fn render_part_picker(
        &self,
        wh: Wh<Px>,
        cg_file: &CgFile,
        project_id: Uuid,
        cut_id: Uuid,
        graphic_index: usize,
        screen_cg: &ScreenCg,
    ) -> namui::RenderingTree {
        table::padding(OUTER_PADDING, |wh| {
            table::vertical([
                table::fixed(BUTTON_HEIGHT, |wh| render_cg_select_button(wh)),
                render_divider(BUTTON_HEIGHT),
                table::ratio(1, |wh| {
                    self.scroll_view.render(&scroll_view::Props {
                        xy: Xy::zero(),
                        height: wh.height,
                        scroll_bar_width: 4.px(),
                        content: render_cg_part_group_list(
                            wh,
                            cg_file,
                            project_id,
                            cut_id,
                            graphic_index,
                            screen_cg,
                        ),
                    })
                }),
            ])(wh)
        })(wh)
    }
}

fn render_cg_select_button(wh: Wh<Px>) -> RenderingTree {
    table::horizontal_padding(INNER_PADDING, |wh| {
        render([
            center_text_full_height(wh, "Change Cg", color::STROKE_NORMAL),
            simple_rect(wh, color::STROKE_NORMAL, 1.px(), Color::TRANSPARENT)
                .with_mouse_cursor(MouseCursor::Pointer)
                .attach_event(|builder| {
                    builder.on_mouse_down_in(|_| {
                        namui::event::send(InternalEvent::CgChangeButtonClicked)
                    });
                }),
        ])
    })(wh)
}

fn render_cg_part_group_list(
    wh: Wh<Px>,
    cg_file: &CgFile,
    project_id: Uuid,
    cut_id: Uuid,
    graphic_index: usize,
    screen_cg: &ScreenCg,
) -> RenderingTree {
    let cg_id = cg_file.id;
    table::vertical(cg_file.parts.iter().flat_map(|cg_part| {
        render_cg_part_group(
            wh.width,
            cg_part,
            project_id,
            cg_id,
            cut_id,
            graphic_index,
            screen_cg,
        )
    }))(wh)
}

fn render_cg_part_group<'a>(
    width: Px,
    cg_part: &'a CgPart,
    project_id: Uuid,
    cg_id: Uuid,
    cut_id: Uuid,
    graphic_index: usize,
    screen_cg: &'a ScreenCg,
) -> Vec<TableCell<'a>> {
    let no_selection = !screen_cg
        .part_variants
        .iter()
        .any(|(selected_variant_id, _)| {
            cg_part
                .variants
                .iter()
                .any(|variant| variant.id == *selected_variant_id)
        });

    let title_bar = render_title_bar(cg_part);

    let no_selection_button =
        render_no_selection_button(no_selection, cg_part, cut_id, graphic_index);

    let max_thumbnails_per_row = (width / (THUMBNAIL_WH.width)).floor() as usize;
    let chunks = cg_part.variants.chunks_exact(max_thumbnails_per_row);
    let chunk_remainder = chunks.remainder();
    let last_variant_row = table::fixed(THUMBNAIL_WH.height, move |wh| {
        table::horizontal(
            chunk_remainder
                .iter()
                .map(|variant| {
                    render_thumbnail(
                        cg_part,
                        variant,
                        project_id,
                        cg_id,
                        cut_id,
                        graphic_index,
                        screen_cg,
                    )
                })
                .chain(once(no_selection_button)),
        )(wh)
    });
    let variant_rows = chunks.map(|row| {
        table::fixed(THUMBNAIL_WH.height, move |wh| {
            table::horizontal(row.iter().map(|variant| {
                render_thumbnail(
                    cg_part,
                    variant,
                    project_id,
                    cg_id,
                    cut_id,
                    graphic_index,
                    screen_cg,
                )
            }))(wh)
        })
    });

    once(title_bar)
        .chain(variant_rows)
        .chain(once(last_variant_row))
        .chain(once(render_divider(BUTTON_HEIGHT)))
        .collect()
}

fn render_title_bar(cg_part: &CgPart) -> TableCell {
    table::fixed(BUTTON_HEIGHT, |wh| {
        table::horizontal_padding(INNER_PADDING, |wh| {
            render([
                simple_rect(wh, color::STROKE_NORMAL, 1.px(), color::BACKGROUND),
                center_text_full_height(wh, cg_part.name.clone(), color::STROKE_NORMAL),
            ])
        })(wh)
    })
}

fn render_no_selection_button(
    no_selection: bool,
    cg_part: &CgPart,
    cut_id: Uuid,
    graphic_index: usize,
) -> TableCell {
    table::fixed(THUMBNAIL_WH.width, move |wh| {
        table::padding(INNER_PADDING, |wh| {
            render([
                center_text(
                    wh,
                    "No Selection",
                    match no_selection {
                        true => color::STROKE_SELECTED,
                        false => color::STROKE_NORMAL,
                    },
                    12.int_px(),
                ),
                simple_rect(wh, color::STROKE_NORMAL, 1.px(), Color::TRANSPARENT)
                    .with_mouse_cursor(MouseCursor::Pointer)
                    .attach_event(move |builder| {
                        let cg_part_variant_ids = cg_part
                            .variants
                            .iter()
                            .map(|variant| variant.id)
                            .collect::<Vec<_>>();
                        builder.on_mouse_down_in(move |_| {
                            namui::event::send(Event::UpdateCutGraphics {
                                cut_id,
                                callback: {
                                    let cg_part_variant_ids = cg_part_variant_ids.clone();
                                    Box::new(move |graphics| {
                                        if let ScreenGraphic::Cg(cg) = &mut graphics[graphic_index]
                                        {
                                            cg.part_variants.retain(|(variant_id, _)| {
                                                !cg_part_variant_ids.contains(variant_id)
                                            });
                                        };
                                    })
                                },
                            });
                        });
                    }),
            ])
        })(wh)
    })
}

fn render_divider<'a>(height: Px) -> TableCell<'a> {
    table::fixed(height, |_wh| RenderingTree::Empty)
}

fn render_thumbnail<'a>(
    cg_part: &'a CgPart,
    cg_part_variant: &'a CgPartVariant,
    project_id: Uuid,
    cg_id: Uuid,
    cut_id: Uuid,
    graphic_index: usize,
    screen_cg: &'a ScreenCg,
) -> TableCell<'a> {
    let selected = screen_cg
        .part_variants
        .iter()
        .any(|(cg_part_variant_id, _)| *cg_part_variant_id == cg_part_variant.id);

    table::fixed(THUMBNAIL_WH.width, move |wh| {
        table::padding(INNER_PADDING, |wh| {
            render([
                simple_rect(wh, Color::TRANSPARENT, 0.px(), color::BACKGROUND)
                    .with_mouse_cursor(MouseCursor::Pointer),
                get_project_cg_part_variant_image_url(project_id, cg_id, cg_part_variant.id)
                    .map_or(RenderingTree::Empty, |cg_part_image_url| {
                        image(ImageParam {
                            rect: Rect::from_xy_wh(Xy::zero(), wh),
                            source: ImageSource::Url(cg_part_image_url),
                            style: ImageStyle {
                                fit: ImageFit::Contain,
                                paint_builder: None,
                            },
                        })
                    }),
                simple_rect(
                    wh,
                    match selected {
                        true => color::STROKE_SELECTED,
                        false => color::STROKE_NORMAL,
                    },
                    1.px(),
                    Color::TRANSPARENT,
                )
                .attach_event(|builder| {
                    let selection_type = cg_part.selection_type;
                    let cg_part_variant = cg_part_variant.clone();
                    let cg_part_variant_ids = cg_part
                        .variants
                        .iter()
                        .map(|variant| variant.id)
                        .collect::<Vec<_>>();
                    builder.on_mouse_down_in(move |_| {
                        namui::event::send(Event::UpdateCutGraphics {
                            cut_id,
                            callback: {
                                let cg_part_variant_ids = cg_part_variant_ids.clone();
                                Box::new(move |graphics| {
                                    if let ScreenGraphic::Cg(cg) = &mut graphics[graphic_index] {
                                        match (selected, selection_type) {
                                            (true, rpc::data::PartSelectionType::AlwaysOn) => {}
                                            (true, _) => {
                                                cg.part_variants.retain(|(variant_id, _)| {
                                                    variant_id != &cg_part_variant.id
                                                })
                                            }
                                            (false, rpc::data::PartSelectionType::Single) => {
                                                cg.part_variants.retain(|(variant_id, _)| {
                                                    !cg_part_variant_ids.contains(variant_id)
                                                });
                                                cg.part_variants.push((
                                                    cg_part_variant.id,
                                                    cg_part_variant.rect,
                                                ));
                                            }
                                            (false, rpc::data::PartSelectionType::Multi) => {
                                                cg.part_variants.retain(|(variant_id, _)| {
                                                    variant_id != &cg_part_variant.id
                                                });
                                                cg.part_variants.push((
                                                    cg_part_variant.id,
                                                    cg_part_variant.rect,
                                                ));
                                            }
                                            (false, rpc::data::PartSelectionType::AlwaysOn) => {
                                                cg.part_variants.retain(|(variant_id, _)| {
                                                    !cg_part_variant_ids.contains(variant_id)
                                                });
                                                cg.part_variants.push((
                                                    cg_part_variant.id,
                                                    cg_part_variant.rect,
                                                ));
                                            }
                                        }
                                    };
                                })
                            },
                        })
                    });
                }),
            ])
            .with_tooltip(cg_part_variant.name.clone())
        })(wh)
    })
}
