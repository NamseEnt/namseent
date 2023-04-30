use super::*;
use crate::{color, storage::get_project_cg_part_variant_image_url};
use namui_prebuilt::{
    table::TableCell,
    typography::{center_text, center_text_full_height},
    *,
};
use rpc::data::{CgFile, CgPart, CgPartVariant};
use std::iter::once;
use tooltip::*;

const BUTTON_HEIGHT: Px = px(32.0);
const OUTER_PADDING: Px = px(8.0);
const INNER_PADDING: Px = px(4.0);

impl CharacterEditor {
    pub fn render_part_picker(
        &self,
        wh: Wh<Px>,
        cg_file: &CgFile,
        project_id: Uuid,
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
                        content: render_cg_part_group_list(wh, cg_file, project_id),
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

fn render_cg_part_group_list(wh: Wh<Px>, cg_file: &CgFile, project_id: Uuid) -> RenderingTree {
    let cg_id = cg_file.id;
    table::vertical(
        cg_file
            .parts
            .iter()
            .flat_map(|cg_part| render_cg_part_group(wh.width, cg_part, project_id, cg_id)),
    )(wh)
}

fn render_cg_part_group(
    width: Px,
    cg_part: &CgPart,
    project_id: Uuid,
    cg_id: Uuid,
) -> Vec<TableCell> {
    const THUMBNAIL_WH: Wh<Px> = Wh {
        width: px(96.0),
        height: px(96.0),
    };

    fn render_thumbnail(
        cg_part_variant: &CgPartVariant,
        project_id: Uuid,
        cg_id: Uuid,
    ) -> TableCell {
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
                    simple_rect(wh, color::STROKE_NORMAL, 1.px(), Color::TRANSPARENT),
                ])
                .with_tooltip(cg_part_variant.name.clone())
            })(wh)
        })
    }

    let title_bar = table::fixed(BUTTON_HEIGHT, |wh| {
        table::horizontal_padding(INNER_PADDING, |wh| {
            render([
                simple_rect(wh, color::STROKE_NORMAL, 1.px(), color::BACKGROUND),
                center_text_full_height(wh, cg_part.name.clone(), color::STROKE_NORMAL),
            ])
        })(wh)
    });

    let no_selection_thumbnail = table::fixed(THUMBNAIL_WH.width, |wh| {
        table::padding(INNER_PADDING, |wh| {
            render([
                center_text(wh, "No Selection", color::STROKE_NORMAL, 12.int_px()),
                simple_rect(wh, color::STROKE_NORMAL, 1.px(), Color::TRANSPARENT)
                    .with_mouse_cursor(MouseCursor::Pointer),
            ])
        })(wh)
    });

    let max_thumbnails_per_row = (width / (THUMBNAIL_WH.width)).floor() as usize;
    let chunks = cg_part.variants.chunks_exact(max_thumbnails_per_row);
    let chunk_remainder = chunks.remainder();
    let last_variant_row = table::fixed(THUMBNAIL_WH.height, move |wh| {
        table::horizontal(
            chunk_remainder
                .iter()
                .map(|variant| render_thumbnail(variant, project_id, cg_id))
                .chain(once(no_selection_thumbnail)),
        )(wh)
    });
    let variant_rows = chunks.map(|row| {
        table::fixed(THUMBNAIL_WH.height, move |wh| {
            table::horizontal(
                row.iter()
                    .map(|variant| render_thumbnail(variant, project_id, cg_id)),
            )(wh)
        })
    });

    once(title_bar)
        .chain(variant_rows)
        .chain(once(last_variant_row))
        .chain(once(render_divider(BUTTON_HEIGHT)))
        .collect()
}

fn render_divider<'a>(height: Px) -> TableCell<'a> {
    table::fixed(height, |_wh| RenderingTree::Empty)
}
