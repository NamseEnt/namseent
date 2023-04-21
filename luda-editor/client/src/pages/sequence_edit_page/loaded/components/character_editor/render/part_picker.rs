use super::*;
use crate::color;
use namui_prebuilt::{
    table::TableCell,
    typography::{center_text, center_text_full_height},
    *,
};
use std::iter::once;
use tooltip::*;

impl CharacterEditor {
    pub fn render_part_picker(&self, wh: Wh<Px>, pose_file: &PoseFile) -> namui::RenderingTree {
        const PADDING: Px = px(8.0);
        table::padding(PADDING, |wh| {
            self.scroll_view.render(&scroll_view::Props {
                xy: Xy::zero(),
                height: wh.height,
                scroll_bar_width: 4.px(),
                content: table::vertical(
                    pose_file
                        .parts
                        .iter()
                        .flat_map(|pose_part| render_pose_part_group(wh.width, pose_part)),
                )(wh),
            })
        })(wh)
    }
}

fn render_pose_part_group(width: Px, pose_part: &PosePart) -> Vec<TableCell> {
    const TITLE_BAR_HEIGHT: Px = px(32.0);
    const THUMBNAIL_WH: Wh<Px> = Wh {
        width: px(96.0),
        height: px(96.0),
    };
    const PADDING: Px = px(4.0);

    fn render_thumbnail(pose_variant: &PoseVariant) -> TableCell {
        table::fixed(THUMBNAIL_WH.width, |wh| {
            table::padding(PADDING, |wh| {
                render([
                    simple_rect(wh, Color::TRANSPARENT, 0.px(), color::BACKGROUND)
                        .with_mouse_cursor(MouseCursor::Pointer),
                    simple_rect(wh, color::STROKE_NORMAL, 1.px(), Color::TRANSPARENT),
                ])
                .with_tooltip(pose_variant.name.clone())
            })(wh)
        })
    }

    let title_bar = table::fixed(TITLE_BAR_HEIGHT, |wh| {
        table::horizontal_padding(PADDING, |wh| {
            render([
                simple_rect(wh, color::STROKE_NORMAL, 1.px(), color::BACKGROUND),
                center_text_full_height(wh, pose_part.name.clone(), color::STROKE_NORMAL),
            ])
        })(wh)
    });
    let divider = table::fixed(TITLE_BAR_HEIGHT, |_wh| RenderingTree::Empty);
    let no_selection_thumbnail = table::fixed(THUMBNAIL_WH.width, |wh| {
        table::padding(PADDING, |wh| {
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
        .chain(once(divider))
        .collect()
}
