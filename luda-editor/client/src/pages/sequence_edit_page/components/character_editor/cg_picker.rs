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
    pub on_event: &'a (dyn 'a + Fn(Event)),
}

pub enum Event {
    MoveInCgFileThumbnail { global_xy: Xy<Px>, name: String },
    ClickCgFileThumbnail { cg_id: Uuid },
}

impl Component for CgPicker<'_> {
    fn render<'a>(self, ctx: &'a RenderCtx) -> RenderDone {
        let Self {
            wh,
            project_id,
            ref on_event,
        } = self;
        let (cg_file_list, _) = ctx.atom(&CG_FILES_ATOM);

        ctx.compose(|ctx| {
            table::hooks::padding(OUTER_PADDING, |wh, ctx| {
                let max_items_per_row =
                    (wh.width / (CHARACTER_THUMBNAIL_WH.width)).floor() as usize;
                ctx.add(scroll_view::AutoScrollViewWithCtx {
                    xy: Xy::zero(),
                    scroll_bar_width: 4.px(),
                    height: wh.height,
                    content: |ctx| {
                        table::hooks::vertical(cg_file_list.chunks(max_items_per_row).map(
                            |cg_files| {
                                table::hooks::fixed(CHARACTER_THUMBNAIL_WH.height, {
                                    table::hooks::horizontal(cg_files.iter().map(|cg_file| {
                                        render_thumbnail(cg_file, project_id, on_event.clone())
                                    }))
                                })
                            },
                        ))(wh, ctx)
                    },
                });
            })(wh, ctx)
        })
        .done()
    }
}

fn render_thumbnail<'a>(
    cg_file: &'a CgFile,
    project_id: Uuid,
    on_event: &'a (dyn 'a + Fn(Event)),
) -> TableCell<'a> {
    table::hooks::fixed(CHARACTER_THUMBNAIL_WH.width, {
        table::hooks::padding(INNER_PADDING, move |wh, ctx| {
            ctx.add(
                get_project_cg_thumbnail_image_url(project_id, cg_file.id).map_or(
                    RenderingTree::Empty,
                    |cg_thumbnail_image_url| {
                        image(ImageParam {
                            rect: Rect::from_xy_wh(Xy::zero(), wh),
                            source: ImageSource::Url {
                                url: cg_thumbnail_image_url,
                            },
                            style: ImageStyle {
                                fit: ImageFit::Contain,
                                paint: None,
                            },
                        })
                    },
                ),
            )
            .add(
                simple_rect(wh, color::STROKE_NORMAL, 1.px(), Color::TRANSPARENT)
                    .with_mouse_cursor(MouseCursor::Pointer)
                    // .with_tooltip(cg_file.name.clone())
                    .attach_event({
                        let cg_id = cg_file.id;
                        let cg_file_name = cg_file.name.clone();
                        move |event| match event {
                            namui::Event::MouseDown { event } => {
                                if event.is_local_xy_in() {
                                    on_event(Event::ClickCgFileThumbnail { cg_id })
                                }
                            }
                            namui::Event::MouseMove { event } => {
                                if event.is_local_xy_in() {
                                    on_event(Event::MoveInCgFileThumbnail {
                                        global_xy: event.global_xy,
                                        name: cg_file_name.clone(),
                                    })
                                }
                            }
                            _ => {}
                        }
                    }),
            );
        })
    })
}
