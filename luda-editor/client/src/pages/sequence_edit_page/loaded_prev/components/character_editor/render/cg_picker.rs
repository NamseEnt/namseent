use super::*;
use crate::{color, storage::get_project_cg_thumbnail_image_url};
use namui_prebuilt::{table::TableCell, *};
use rpc::data::CgFile;

const OUTER_PADDING: Px = px(8.0);
const INNER_PADDING: Px = px(4.0);
const CHARACTER_THUMBNAIL_WH: Wh<Px> = Wh {
    width: px(96.0),
    height: px(144.0),
};

impl CharacterEditor {
    pub fn render_cg_picker(
        &self,
        wh: Wh<Px>,
        cg_file_list: &Vec<CgFile>,
        project_id: Uuid,
    ) -> namui::RenderingTree {
        table::padding(OUTER_PADDING, |wh| {
            let max_items_per_row = (wh.width / (CHARACTER_THUMBNAIL_WH.width)).floor() as usize;
            self.scroll_view.render(&scroll_view::Props {
                xy: Xy::zero(),
                height: wh.height,
                scroll_bar_width: 4.px(),
                content: table::vertical(cg_file_list.chunks(max_items_per_row).map(|cg_files| {
                    table::fixed(CHARACTER_THUMBNAIL_WH.height, |wh| {
                        table::horizontal(
                            cg_files
                                .iter()
                                .map(|cg_file| render_thumbnail(cg_file, project_id)),
                        )(wh)
                    })
                }))(wh),
            })
        })(wh)
    }
}

fn render_thumbnail(cg_file: &CgFile, project_id: Uuid) -> TableCell {
    table::fixed(CHARACTER_THUMBNAIL_WH.width, move |wh| {
        table::padding(INNER_PADDING, |wh| {
            render([
                get_project_cg_thumbnail_image_url(project_id, cg_file.id).map_or(
                    RenderingTree::Empty,
                    |cg_thumbnail_image_url| {
                        image(ImageParam {
                            rect: Rect::from_xy_wh(Xy::zero(), wh),
                            source: ImageSource::Url(cg_thumbnail_image_url),
                            style: ImageStyle {
                                fit: ImageFit::Contain,
                                paint_builder: None,
                            },
                        })
                    },
                ),
                simple_rect(wh, color::STROKE_NORMAL, 1.px(), Color::TRANSPARENT)
                    .with_mouse_cursor(MouseCursor::Pointer)
                    .with_tooltip(cg_file.name.clone())
                    .attach_event({
                        let cg_id = cg_file.id;
                        move |builder| {
                            builder.on_mouse_down_in(move |_| {
                                namui::event::send(InternalEvent::CgThumbnailClicked { cg_id })
                            });
                        }
                    }),
            ])
        })(wh)
    })
}
