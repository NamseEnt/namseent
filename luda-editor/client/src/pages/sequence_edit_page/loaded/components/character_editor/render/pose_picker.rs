use super::*;
use crate::color;
use namui_prebuilt::{table::TableCell, *};

impl CharacterEditor {
    pub fn render_pose_picker(
        &self,
        wh: Wh<Px>,
        pose_file_list: &Vec<PoseFile>,
    ) -> namui::RenderingTree {
        const CHARACTER_THUMBNAIL_WH: Wh<Px> = Wh {
            width: px(96.0),
            height: px(144.0),
        };
        const PADDING: Px = px(8.0);

        fn render_thumbnail(pose_file: &PoseFile) -> TableCell {
            table::fixed(CHARACTER_THUMBNAIL_WH.width, |wh| {
                table::padding(PADDING, |wh| {
                    simple_rect(wh, color::STROKE_NORMAL, 1.px(), Color::TRANSPARENT)
                        .with_mouse_cursor(MouseCursor::Pointer)
                        .with_tooltip(pose_file.name.clone())
                })(wh)
            })
        }

        table::padding(PADDING, |wh| {
            let max_items_per_row = (wh.width / (CHARACTER_THUMBNAIL_WH.width)).floor() as usize;
            self.scroll_view.render(&scroll_view::Props {
                xy: Xy::zero(),
                height: wh.height,
                scroll_bar_width: 4.px(),
                content: table::vertical(pose_file_list.chunks(max_items_per_row).map(
                    |pose_files| {
                        table::fixed(CHARACTER_THUMBNAIL_WH.height, |wh| {
                            table::horizontal(
                                pose_files
                                    .iter()
                                    .map(|pose_file| render_thumbnail(pose_file)),
                            )(wh)
                        })
                    },
                ))(wh),
            })
        })(wh)
    }
}
