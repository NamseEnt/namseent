use super::{auto_column_list::AutoColumnList, CG_FILES_ATOM, SELECTED_ASSET_ATOM};
use crate::{color, storage::get_project_cg_thumbnail_image_url};
use namui::*;
use namui_prebuilt::simple_rect;
use rpc::data::{CgFile, ScreenCg};

const DOUBLE_CLICK_TIME: Time = Time::Sec(0.3);

pub(super) struct CgList {
    pub wh: Wh<Px>,
    pub project_id: Uuid,
}

impl Component for CgList {
    fn render(self, ctx: &RenderCtx)  {
        let Self { wh, project_id } = self;

        let (cg_files, _set_cg_files) = ctx.atom(&CG_FILES_ATOM);

        ctx.component(AutoColumnList {
            wh,
            items: cg_files,
            name_specifier: &|cg_file| cg_file.name.to_string(),
            thumbnail_renderer: &|image, wh, ctx| {
                ctx.add(Thumbnail {
                    wh,
                    project_id,
                    cg_file: image,
                });
            },
        });

        ctx.component(simple_rect(
            wh,
            color::STROKE_NORMAL,
            1.px(),
            color::BACKGROUND,
        ));

        
    }
}

struct Thumbnail<'a> {
    wh: Wh<Px>,
    project_id: Uuid,
    cg_file: &'a CgFile,
}
impl Component for Thumbnail<'_> {
    fn render(self, ctx: &RenderCtx)  {
        let Self {
            wh,
            cg_file,
            project_id,
        } = self;

        let (_, set_selected_asset) = ctx.atom(&SELECTED_ASSET_ATOM);
        let (last_clicked_time, set_last_clicked_time) = ctx.state(|| Time::Day(-1.0));

        ctx.component(
            simple_rect(wh, color::STROKE_NORMAL, 1.px(), Color::TRANSPARENT)
                .with_mouse_cursor(MouseCursor::Pointer)
                .attach_event(|event| {
                    if let namui::Event::MouseDown { event } = event {
                        if !event.is_local_xy_in() {
                            return;
                        }
                        let now = now();
                        if now - *last_clicked_time > DOUBLE_CLICK_TIME {
                            set_last_clicked_time.set(now);
                            return;
                        }
                        set_selected_asset
                            .set(Some(super::SelectedAsset::Cg(ScreenCg::new(cg_file))))
                    }
                }),
        );

        ctx.component(
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
        );

        
    }
}
