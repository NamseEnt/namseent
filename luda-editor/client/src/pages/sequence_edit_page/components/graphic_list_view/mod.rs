use crate::{
    color,
    components::cg_render,
    pages::sequence_edit_page::atom::{CG_FILES_ATOM, EDITING_GRAPHIC_INDEX_ATOM},
    storage::{get_project_cg_thumbnail_image_url, get_project_image_url},
};
use namui::prelude::*;
use namui_prebuilt::{scroll_view, simple_rect, table};
use rpc::data::ScreenGraphic;

#[component]
pub struct GraphicListView<'a> {
    pub project_id: Uuid,
    pub wh: Wh<Px>,
    pub graphics: Option<&'a Vec<(Uuid, ScreenGraphic)>>,
}

pub enum Event {}

impl Component for GraphicListView<'_> {
    fn render(self, ctx: &RenderCtx) -> RenderDone {
        const HEADER_HEIGHT: Px = px(32.0);
        const GRAPHIC_LIST_ITEM_HEIGHT: Px = px(48.0);
        const PADDING: Px = px(4.0);

        let Self {
            project_id,
            wh,
            graphics,
        } = self;

        ctx.compose(|ctx| {
            table::hooks::vertical([
                table::hooks::fixed(HEADER_HEIGHT, |wh, ctx| {
                    ctx.add(Header { wh });
                }),
                table::hooks::ratio(1, |wh, ctx| {
                    table::hooks::padding(PADDING, |wh, ctx| {
                        ctx.add(scroll_view::AutoScrollViewWithCtx {
                            xy: Xy::zero(),
                            scroll_bar_width: 4.px(),
                            height: wh.height,
                            content: |ctx| {
                                let Some(graphics) = graphics else {
                                    return;
                                };
                                table::hooks::vertical(graphics.iter().map(
                                    |(graphic_index, graphic)| {
                                        table::hooks::fixed(GRAPHIC_LIST_ITEM_HEIGHT, |wh, ctx| {
                                            table::hooks::padding(PADDING, |wh, ctx| {
                                                let graphic_index = *graphic_index;
                                                ctx.add_with_key(
                                                    graphic_index,
                                                    GraphicListItem {
                                                        project_id,
                                                        wh,
                                                        graphic,
                                                        graphic_index,
                                                    },
                                                );
                                            })(wh, ctx);
                                        })
                                    },
                                ))(wh, ctx);
                            },
                        });
                    })(wh, ctx);
                }),
            ])(wh, ctx);
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
struct Header {
    wh: Wh<Px>,
}
impl Component for Header {
    fn render(self, ctx: &RenderCtx) -> RenderDone {
        const PADDING: Wh<Px> = Wh {
            width: px(8.0),
            height: px(4.0),
        };

        let Self { wh } = self;

        ctx.compose(|ctx| {
            table::hooks::vertical_padding(PADDING.height, |wh, ctx| {
                table::hooks::horizontal_padding(PADDING.width, |wh, ctx| {
                    ctx.add(text(TextParam {
                        text: "Graphic List".to_string(),
                        x: 0.px(),
                        y: wh.height / 2.0,
                        align: TextAlign::Left,
                        baseline: TextBaseline::Middle,
                        font: Font {
                            size: 12.int_px(),
                            name: "NotoSansKR-Regular".to_string(),
                        },
                        style: TextStyle {
                            color: color::STROKE_NORMAL,
                            ..Default::default()
                        },
                        max_width: Some(wh.width),
                    }));
                })(wh, ctx);
            })(wh, ctx);
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
struct GraphicListItem<'a> {
    project_id: Uuid,
    wh: Wh<Px>,
    graphic: &'a ScreenGraphic,
    graphic_index: Uuid,
}
impl Component for GraphicListItem<'_> {
    fn render(self, ctx: &RenderCtx) -> RenderDone {
        const PADDING: Px = px(4.0);

        let Self {
            project_id,
            wh,
            graphic,
            graphic_index,
        } = self;
        let graphic_name = match graphic {
            ScreenGraphic::Image(image) => image.id.to_string(),
            ScreenGraphic::Cg(cg) => cg.name.clone(),
        };
        let (editing_graphic_index, set_editing_graphic_index) =
            ctx.atom_init(&EDITING_GRAPHIC_INDEX_ATOM, || None);

        let is_selected = editing_graphic_index.as_ref() == &Some(graphic_index);
        let stroke_color = color::stroke_color(is_selected, false);
        let stroke_width = match is_selected {
            true => 2.px(),
            false => 1.px(),
        };

        ctx.compose(|ctx| {
            table::hooks::horizontal([
                table::hooks::fixed(wh.height, |wh, ctx| {
                    ctx.add(RenderGraphic {
                        project_id,
                        wh,
                        graphic,
                    });
                }),
                table::hooks::ratio(1, |wh, ctx| {
                    table::hooks::padding(PADDING, |wh, ctx| {
                        ctx.add(text(TextParam {
                            text: graphic_name,
                            x: 0.px(),
                            y: wh.height / 2.0,
                            align: TextAlign::Left,
                            baseline: TextBaseline::Middle,
                            font: Font {
                                size: 12.int_px(),
                                name: "NotoSansKR-Regular".to_string(),
                            },
                            style: TextStyle {
                                color: stroke_color,
                                ..Default::default()
                            },
                            max_width: None,
                        }));
                    })(wh, ctx);
                }),
            ])(wh, ctx);
        });

        ctx.component(
            simple_rect(wh, stroke_color, stroke_width, color::BACKGROUND)
                .attach_event(|event| {
                    if let namui::Event::MouseDown { event } = event {
                        if event.is_local_xy_in() && (event.button == Some(MouseButton::Left)) {
                            event.stop_propagation();
                            set_editing_graphic_index.set(Some(graphic_index));
                        }
                    }
                })
                .with_mouse_cursor(MouseCursor::Pointer),
        );

        ctx.done()
    }
}

#[component]
struct RenderGraphic<'a> {
    project_id: Uuid,
    wh: Wh<Px>,
    graphic: &'a ScreenGraphic,
}

impl Component for RenderGraphic<'_> {
    fn render(self, ctx: &RenderCtx) -> RenderDone {
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
            return ctx.done();
        };

        let Ok(image) = image else {
            namui::log!("Failed to load image: {:?}", url);
            return ctx.done();
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
