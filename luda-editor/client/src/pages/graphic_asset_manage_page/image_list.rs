use super::{auto_column_list::AutoColumnList, SelectedAsset, IMAGES_ATOM, SELECTED_ASSET_ATOM};
use crate::{color, storage::get_project_image_url};
use namui::prelude::*;
use namui_prebuilt::simple_rect;
use rpc::data::ImageWithLabels;

const DOUBLE_CLICK_TIME: Time = Time::Sec(0.3);

#[component]
pub(super) struct ImageList {
    pub wh: Wh<Px>,
    pub project_id: Uuid,
}

impl Component for ImageList {
    fn render(self, ctx: &RenderCtx) -> RenderDone {
        let Self { wh, project_id } = self;

        let (images, _set_images) = ctx.atom(&IMAGES_ATOM);

        ctx.component(AutoColumnList {
            wh,
            items: images,
            name_specifier: &|image: &ImageWithLabels| image.id.to_string(),
            thumbnail_renderer: &|image, wh, ctx| {
                ctx.add(Thumbnail {
                    wh,
                    project_id,
                    image,
                });
            },
        });

        ctx.component(simple_rect(
            wh,
            color::STROKE_NORMAL,
            1.px(),
            color::BACKGROUND,
        ));

        ctx.done()
    }
}

#[component]
struct Thumbnail<'a> {
    wh: Wh<Px>,
    project_id: Uuid,
    image: &'a ImageWithLabels,
}
impl Component for Thumbnail<'_> {
    fn render(self, ctx: &RenderCtx) -> RenderDone {
        let Self {
            wh,
            image,
            project_id,
        } = self;

        let (_, set_selected_asset) = ctx.atom(&SELECTED_ASSET_ATOM);
        let (last_clicked_time, set_last_clicked_time) = ctx.state(|| Time::Day(-1.0));

        ctx.component(
            simple_rect(wh, color::STROKE_NORMAL, 1.px(), Color::TRANSPARENT)
                .with_mouse_cursor(MouseCursor::Pointer)
                .attach_event({
                    |event| {
                        if let namui::Event::MouseDown { event } = event {
                            if !event.is_local_xy_in() {
                                return;
                            }
                            let now = now();
                            if now - *last_clicked_time > DOUBLE_CLICK_TIME {
                                set_last_clicked_time.set(now);
                                return;
                            }
                            set_selected_asset.set(Some(SelectedAsset::Image(image.clone())));
                        }
                    }
                }),
        );
        ctx.component(get_project_image_url(project_id, image.id).map_or(
            RenderingTree::Empty,
            |cg_thumbnail_image_url| {
                namui::image(ImageParam {
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
        ));
        ctx.done()
    }
}
