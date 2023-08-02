use super::*;
use crate::{color, storage::get_project_cg_thumbnail_image_url};
use namui_prebuilt::{table::hooks::TableCell, *};
use rpc::data::CgFile;

const OUTER_PADDING: Px = px(8.0);
const INNER_PADDING: Px = px(4.0);
const CHARACTER_THUMBNAIL_WH: Wh<Px> = Wh {
    width: px(96.0),
    height: px(144.0),
};

#[namui::component]
pub struct CgPicker<'a> {
    pub wh: Wh<Px>,
    pub project_id: Uuid,
    pub on_event: &'a dyn Fn(Event),
}

pub enum Event {
    MoveInCgFileThumbnail { global_xy: Xy<Px>, name: String },
    ClickCgFileThumbnail { cg_id: Uuid },
}

impl Component for CgPicker<'_> {
    fn render<'a>(&'a self, ctx: RenderCtx<'a>) -> RenderDone {
        let &Self {
            wh,
            project_id,
            on_event,
        } = self;
        let (cg_file_list, _) = ctx.use_atom(&CG_FILES_ATOM);

        ctx.add(table::hooks::padding(OUTER_PADDING, |wh| {
            let max_items_per_row = (wh.width / (CHARACTER_THUMBNAIL_WH.width)).floor() as usize;
            scroll_view::AutoScrollView {
                xy: Xy::zero(),
                scroll_bar_width: 4.px(),
                height: wh.height,
                content: table::hooks::vertical(cg_file_list.chunks(max_items_per_row).map(
                    |cg_files| {
                        table::hooks::fixed(CHARACTER_THUMBNAIL_WH.height, |wh| {
                            table::hooks::horizontal(
                                cg_files
                                    .iter()
                                    .map(|cg_file| render_thumbnail(cg_file, project_id, on_event)),
                            )(wh)
                        })
                    },
                ))(wh)
                .arc(),
            }
        })(wh));

        ctx.done()
    }
}

fn render_thumbnail<'a>(
    cg_file: &'a CgFile,
    project_id: Uuid,
    on_event: &'a dyn Fn(Event),
) -> TableCell<'a> {
    table::hooks::fixed(CHARACTER_THUMBNAIL_WH.width, move |wh| {
        table::hooks::padding(INNER_PADDING, |wh| {
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
                    // .with_tooltip(cg_file.name.clone())
                    .attach_event({
                        let cg_id = cg_file.id;
                        let cg_file_name = cg_file.name.clone();
                        move |builder| {
                            builder
                                .on_mouse_move_in(move |e: MouseEvent| {
                                    on_event(Event::MoveInCgFileThumbnail {
                                        global_xy: e.global_xy,
                                        name: cg_file_name.clone(),
                                    })
                                })
                                .on_mouse_down_in(move |_| {
                                    on_event(Event::ClickCgFileThumbnail { cg_id })
                                });
                        }
                    }),
            ])
        })(wh)
    })
}
