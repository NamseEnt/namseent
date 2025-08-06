use crate::{
    components::cg_render,
    pages::sequence_edit_page::atom::CG_FILES_ATOM,
    storage::{get_project_cg_thumbnail_image_url, get_project_image_url},
};
use namui::*;
use rpc::data::ScreenGraphic;

pub struct GraphicThumbnail<'a> {
    pub project_id: Uuid,
    pub wh: Wh<Px>,
    pub graphic: &'a ScreenGraphic,
}

impl Component for GraphicThumbnail<'_> {
    fn render(self, ctx: &RenderCtx) {
        let Self {
            project_id,
            wh,
            graphic,
        } = self;

        let graphic = ctx.track_eq(graphic);
        let url = ctx.memo(|| match graphic.as_ref() {
            ScreenGraphic::Image(screen_image) => {
                get_project_image_url(project_id, screen_image.id).unwrap()
            }
            ScreenGraphic::Cg(screen_cg) => {
                get_project_cg_thumbnail_image_url(project_id, screen_cg.id).unwrap()
            }
        });
        let image = ctx.image(&url);
        let Some(image) = image.as_ref() else {
            return;
        };

        let Ok(image) = image else {
            namui::log!("Failed to load image: {:?}", url);
            return;
        };

        ctx.compose(|ctx| match graphic.as_ref() {
            ScreenGraphic::Image(_screen_image) => {
                ctx.add(namui::image(ImageParam {
                    rect: wh.to_rect(),
                    source: ImageSource::Url {
                        url: url.clone_inner(),
                    },
                    style: ImageStyle {
                        fit: ImageFit::Contain,
                        paint: None,
                    },
                }));
            }
            ScreenGraphic::Cg(screen_cg) => {
                let cg_file = CG_FILES_ATOM
                    .get()
                    .iter()
                    .find(|cg_file| cg_file.name == screen_cg.name);

                // Assume that `wh.width` and `wh.height` are the same.
                let rect = {
                    let ratio = image.wh.width / image.wh.height;
                    match ratio > 1.0 {
                        true => {
                            let cg_height = wh.height / ratio;
                            Rect::Xywh {
                                x: 0.px(),
                                y: (wh.height - cg_height) / 2.0,
                                width: wh.width,
                                height: cg_height,
                            }
                        }
                        false => {
                            let cg_width = wh.width * ratio;
                            Rect::Xywh {
                                x: (wh.width - cg_width) / 2.0,
                                y: 0.px(),
                                width: cg_width,
                                height: wh.height,
                            }
                        }
                    }
                };

                match cg_file {
                    Some(cg_file) => ctx.add(cg_render::CgRender {
                        project_id,
                        rect,
                        screen_cg,
                        cg_file,
                    }),
                    None => ctx.add(RenderingTree::Empty),
                };
            }
        })
        .done()
    }
}
