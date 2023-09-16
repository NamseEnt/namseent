use super::{wysiwyg_tool::WysiwygTool, *};
use crate::{app::notification, clipboard::LudaEditorClipboardItem};

#[namui::component]
pub struct GraphicClip<'a> {
    pub cut_id: Uuid,
    pub graphic_index: Uuid,
    pub graphic: &'a ScreenGraphic,
    pub is_editing_graphic: bool,
    pub project_id: Uuid,
    pub wh: Wh<Px>,
    pub dragging: &'a Option<Dragging>,
    pub cg_files: &'a Vec<CgFile>,
    pub on_event: &'a dyn Fn(Event),
}

pub enum Event<'a> {
    WysiwygTool(wysiwyg_tool::Event),
    SelectImage {
        graphic_index: Uuid,
    },
    GraphicRightClick {
        global_xy: Xy<Px>,
        cut_id: Uuid,
        graphic_index: Uuid,
        graphic_wh: Wh<Px>,
        graphic: &'a ScreenGraphic,
    },
    DeleteGraphic {
        graphic_index: Uuid,
    },
}

impl Component for GraphicClip<'_> {
    fn render(self, ctx: &RenderCtx) -> RenderDone {
        let Self {
            cut_id,
            graphic_index,
            graphic,
            is_editing_graphic,
            project_id,
            wh,
            dragging,
            cg_files,
            on_event,
        } = self;

        let url = match &graphic {
            ScreenGraphic::Image(image) => get_project_image_url(project_id, image.id).unwrap(),
            ScreenGraphic::Cg(cg) => get_project_cg_thumbnail_image_url(project_id, cg.id).unwrap(),
        };
        let image = ctx.image(&url);
        let image = match image.as_ref() {
            Some(Ok(image)) => image,
            Some(Err(error)) => {
                namui::log!("Failed to load image: {:?}", error);
                return ctx.done();
            }
            None => {
                return ctx.done();
            }
        };
        let graphic_wh = image.wh;
        let circumscribed = graphic.circumscribed();

        let screen_radius = wh.length() / 2;
        let graphic_radius_px = graphic_wh.length() / 2;
        let radius_px = screen_radius * circumscribed.radius;
        let graphic_wh_on_screen = graphic_wh * (radius_px / graphic_radius_px);

        let center_xy = wh.as_xy() * circumscribed.center_xy;

        let graphic_rendering_rect = {
            match (is_editing_graphic, dragging.as_ref()) {
                (true, Some(dragging)) => match dragging {
                    Dragging::Resizer { context } => {
                        let circumscribed = context.resize(center_xy, graphic_wh_on_screen, wh);
                        calculate_graphic_rect_on_screen(graphic_wh, wh, circumscribed)
                    }
                    Dragging::Mover { context } => {
                        let circumscribed = context.move_circumscribed(circumscribed);

                        calculate_graphic_rect_on_screen(graphic_wh, wh, circumscribed)
                    }
                },
                _ => {
                    let image_left_top_xy = center_xy - graphic_wh_on_screen.as_xy() / 2.0;

                    Rect::from_xy_wh(image_left_top_xy, graphic_wh_on_screen)
                }
            }
        };

        let graphic_rendering_tree = |ctx: &mut ComposeCtx| {
            match &graphic {
                ScreenGraphic::Image(_image) => ctx.add(namui::image(ImageParam {
                    rect: graphic_rendering_rect,
                    source: image.src.clone(),
                    style: ImageStyle {
                        fit: ImageFit::Fill,
                        paint: None,
                    },
                })),
                ScreenGraphic::Cg(cg) => {
                    let Some(cg_file) = cg_files.iter().find(|cg_file| cg_file.name == cg.name)
                    else {
                        return;
                    };
                    ctx.add(cg_render::CgRender {
                        project_id,
                        rect: graphic_rendering_rect,
                        screen_cg: cg,
                        cg_file,
                    })
                }
            }
            .attach_event(|event| match event {
                namui::Event::MouseDown { event } => {
                    if event.is_local_xy_in() {
                        event.stop_propagation();
                        on_event(Event::SelectImage { graphic_index });

                        if event.button == Some(MouseButton::Right) {
                            on_event(Event::GraphicRightClick {
                                global_xy: event.global_xy,
                                cut_id,
                                graphic_index,
                                graphic_wh,
                                graphic,
                            });
                        }
                    }
                }
                namui::Event::KeyDown { event } => {
                    let ctrl_press = namui::keyboard::ctrl_press();

                    if is_editing_graphic && ctrl_press && event.code == Code::KeyC {
                        match graphic {
                            ScreenGraphic::Image(_) => {
                                let image = image.clone();
                                spawn_local(async move {
                                    let result = namui::clipboard::write_image(&image).await;
                                    match result {
                                        Ok(_) => {
                                            namui::log!("Image copied to clipboard");
                                        }
                                        Err(_) => {
                                            namui::log!("Failed to copy image to clipboard");
                                        }
                                    }
                                })
                            }
                            ScreenGraphic::Cg(cg) => {
                                let cg = cg.clone();
                                spawn_local(async move {
                                    match cg.write_to_clipboard().await {
                                        Ok(()) => namui::log!("Cg copied to clipboard"),
                                        Err(error) => {
                                            notification::push_notification(
                                                notification::Notification::error(
                                                    error.to_string(),
                                                ),
                                            );
                                        }
                                    }
                                })
                            }
                        }
                    }

                    if is_editing_graphic && event.code == Code::Delete {
                        on_event(Event::DeleteGraphic { graphic_index })
                    }
                }
                _ => {}
            });
        };

        ctx.compose(|ctx| {
            if is_editing_graphic {
                ctx.add(WysiwygTool {
                    graphic_dest_rect: graphic_rendering_rect,
                    original_graphic_size: graphic_wh,
                    graphic_index,
                    graphic,
                    dragging,
                    wh,
                    on_event: &|event| on_event(Event::WysiwygTool(event)),
                });
            }
        });

        ctx.compose(graphic_rendering_tree);

        ctx.done()
    }
}
