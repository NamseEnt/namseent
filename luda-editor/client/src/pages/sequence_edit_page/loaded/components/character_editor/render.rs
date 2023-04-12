use std::iter::once;

use super::*;
use crate::color;
use namui_prebuilt::{table::TableCell, typography::center_text_full_height, *};

impl CharacterEditor {
    pub fn render(&self, props: Props) -> namui::RenderingTree {
        render([
            self.render_background(props),
            self.render_pose_part_list(props.wh, &mock()),
        ])
    }

    fn render_background(&self, props: Props) -> namui::RenderingTree {
        let background = simple_rect(props.wh, color::STROKE_NORMAL, 1.px(), color::BACKGROUND);
        background.attach_event(|builder| {
            builder
                .on_mouse_down_out(|_| namui::event::send(Event::MouseDownOutsideCharacterEditor));
        })
    }

    fn render_pose_part_list(&self, wh: Wh<Px>, pose_file: &PoseFile) -> namui::RenderingTree {
        const PADDING: Px = px(8.0);
        table::padding(PADDING, |wh| {
            table::vertical(
                pose_file
                    .parts
                    .iter()
                    .flat_map(|pose_part| render_pose_part_group(wh.width, pose_part)),
            )(wh)
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

    let title_bar = table::fixed(TITLE_BAR_HEIGHT, |wh| {
        table::horizontal_padding(PADDING, |wh| {
            render([
                simple_rect(wh, color::STROKE_NORMAL, 1.px(), color::BACKGROUND),
                center_text_full_height(wh, pose_part.name.clone(), color::STROKE_NORMAL),
            ])
        })(wh)
    });
    let divider = table::fixed(TITLE_BAR_HEIGHT, |_wh| RenderingTree::Empty);
    let max_thumbnails_per_row = (width / (THUMBNAIL_WH.width)).floor() as usize;
    let variant_rows = pose_part
        .variants
        .chunks(max_thumbnails_per_row)
        .map(|row| {
            table::fixed(THUMBNAIL_WH.height, |wh| {
                table::horizontal(row.iter().map(|_variant| {
                    table::fixed(THUMBNAIL_WH.width, |wh| {
                        table::padding(PADDING, |wh| {
                            render([
                                simple_rect(wh, Color::TRANSPARENT, 0.px(), color::BACKGROUND)
                                    .with_mouse_cursor(MouseCursor::Pointer),
                                simple_rect(wh, color::STROKE_NORMAL, 1.px(), Color::TRANSPARENT),
                            ])
                        })(wh)
                    })
                }))(wh)
            })
        });

    once(title_bar)
        .chain(variant_rows)
        .chain(once(divider))
        .collect()
}

fn mock() -> PoseFile {
    PoseFile {
        parts: vec![
            PosePart {
                name: "PosePart name 0".to_string(),
                variants: vec![
                    PoseVariant {},
                    PoseVariant {},
                    PoseVariant {},
                    PoseVariant {},
                    PoseVariant {},
                    PoseVariant {},
                    PoseVariant {},
                    PoseVariant {},
                    PoseVariant {},
                    PoseVariant {},
                ],
            },
            PosePart {
                name: "PosePart name 1".to_string(),
                variants: vec![
                    PoseVariant {},
                    PoseVariant {},
                    PoseVariant {},
                    PoseVariant {},
                    PoseVariant {},
                    PoseVariant {},
                    PoseVariant {},
                    PoseVariant {},
                    PoseVariant {},
                    PoseVariant {},
                ],
            },
        ],
    }
}

struct PoseFile {
    parts: Vec<PosePart>,
}
struct PosePart {
    name: String,
    variants: Vec<PoseVariant>,
}
struct PoseVariant {}
