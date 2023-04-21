use super::*;
use crate::color;
use namui_prebuilt::{
    table::TableCell,
    typography::{center_text, center_text_full_height},
    *,
};
use std::iter::once;
use tooltip::*;

const BUTTON_HEIGHT: Px = px(32.0);
const OUTER_PADDING: Px = px(8.0);
const INNER_PADDING: Px = px(4.0);

impl CharacterEditor {
    pub fn render_part_picker(&self, wh: Wh<Px>, pose_file: &PoseFile) -> namui::RenderingTree {
        table::padding(OUTER_PADDING, |wh| {
            table::vertical([
                table::fixed(BUTTON_HEIGHT, |wh| render_pose_select_button(wh)),
                render_divider(BUTTON_HEIGHT),
                table::ratio(1, |wh| {
                    self.scroll_view.render(&scroll_view::Props {
                        xy: Xy::zero(),
                        height: wh.height,
                        scroll_bar_width: 4.px(),
                        content: render_pose_part_group_list(wh, pose_file),
                    })
                }),
            ])(wh)
        })(wh)
    }
}

fn render_pose_select_button(wh: Wh<Px>) -> RenderingTree {
    table::horizontal_padding(INNER_PADDING, |wh| {
        render([
            center_text_full_height(wh, "Change Pose", color::STROKE_NORMAL),
            simple_rect(wh, color::STROKE_NORMAL, 1.px(), Color::TRANSPARENT)
                .with_mouse_cursor(MouseCursor::Pointer)
                .attach_event(|builder| {
                    builder.on_mouse_down_in(|_| {
                        namui::event::send(InternalEvent::PoseChangeButtonClicked)
                    });
                }),
        ])
    })(wh)
}

fn render_pose_part_group_list(wh: Wh<Px>, pose_file: &PoseFile) -> RenderingTree {
    table::vertical(
        pose_file
            .parts
            .iter()
            .flat_map(|pose_part| render_pose_part_group(wh.width, pose_part)),
    )(wh)
}

fn render_pose_part_group(width: Px, pose_part: &PosePart) -> Vec<TableCell> {
    const THUMBNAIL_WH: Wh<Px> = Wh {
        width: px(96.0),
        height: px(96.0),
    };

    fn render_thumbnail(pose_variant: &PoseVariant) -> TableCell {
        table::fixed(THUMBNAIL_WH.width, |wh| {
            table::padding(INNER_PADDING, |wh| {
                render([
                    simple_rect(wh, Color::TRANSPARENT, 0.px(), color::BACKGROUND)
                        .with_mouse_cursor(MouseCursor::Pointer),
                    simple_rect(wh, color::STROKE_NORMAL, 1.px(), Color::TRANSPARENT),
                ])
                .with_tooltip(pose_variant.name.clone())
            })(wh)
        })
    }

    let title_bar = table::fixed(BUTTON_HEIGHT, |wh| {
        table::horizontal_padding(INNER_PADDING, |wh| {
            render([
                simple_rect(wh, color::STROKE_NORMAL, 1.px(), color::BACKGROUND),
                center_text_full_height(wh, pose_part.name.clone(), color::STROKE_NORMAL),
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
    let chunks = pose_part.variants.chunks_exact(max_thumbnails_per_row);
    let chunk_remainder = chunks.remainder();
    let last_variant_row = table::fixed(THUMBNAIL_WH.height, |wh| {
        table::horizontal(
            chunk_remainder
                .iter()
                .map(|variant| render_thumbnail(variant))
                .chain(once(no_selection_thumbnail)),
        )(wh)
    });
    let variant_rows = chunks.map(|row| {
        table::fixed(THUMBNAIL_WH.height, |wh| {
            table::horizontal(row.iter().map(|variant| render_thumbnail(variant)))(wh)
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
