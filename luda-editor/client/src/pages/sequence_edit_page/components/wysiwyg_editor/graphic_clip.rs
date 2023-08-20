use super::{wysiwyg_tool::WysiwygTool, *};

#[namui::component]
pub struct GraphicClip<'a> {
    pub cut_id: Uuid,
    pub graphic_index: Uuid,
    pub graphic: ScreenGraphic,
    pub is_editing_graphic: bool,
    pub project_id: Uuid,
    pub wh: Wh<Px>,
    pub dragging: Option<Dragging>,
    pub cg_files: Vec<CgFile>,
    pub on_event: Box<dyn 'a + Fn(Event)>,
}

pub enum Event {
    WysiwygTool(wysiwyg_tool::Event),
    SelectImage {
        graphic_index: Uuid,
    },
    GraphicRightClick {
        global_xy: Xy<Px>,
        cut_id: Uuid,
        graphic_index: Uuid,
        graphic_wh: Wh<Px>,
        graphic: ScreenGraphic,
    },
}

impl Component for GraphicClip<'_> {
    fn render<'a>(self, ctx: &'a RenderCtx) -> RenderDone {
        let Self {
            cut_id,
            graphic_index,
            ref graphic,
            is_editing_graphic,
            project_id,
            wh,
            ref dragging,
            ref cg_files,
            ref on_event,
        } = self;
        let on_event = on_event.clone();

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
                    // Dragging::Cropper => todo!(),
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

        let graphic_rendering_tree = match &graphic {
            ScreenGraphic::Image(_image) => namui::image(ImageParam {
                rect: graphic_rendering_rect,
                source: image.src.clone(),
                style: ImageStyle {
                    fit: ImageFit::Fill,
                    paint: None,
                },
            }),
            ScreenGraphic::Cg(cg) => try_render(|| {
                let cg_file = cg_files.iter().find(|cg_file| cg_file.name == cg.name)?;
                Some(cg_render::render_cg(
                    cg_render::CgRenderProps {
                        cg_id: cg.id,
                        project_id,
                        rect: graphic_rendering_rect,
                    },
                    cg,
                    cg_file,
                ))
            }),
        }
        .attach_event(|event| match event {
            namui::Event::MouseDown { event } => {
                if event.is_local_xy_in() {
                    let graphic = graphic.clone();
                    let on_event = on_event.clone();
                    let graphic = graphic.clone();
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
                if is_editing_graphic {
                    let graphic = graphic.clone();
                    namui::log!("key down: {:?}", event.code);
                    let graphic = graphic.clone();
                    if event.code != Code::KeyC || !namui::keyboard::ctrl_press() {
                        return;
                    }

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
                                match clipboard::write([(
                                    "web application/luda-editor-cg+json",
                                    serde_json::to_string(&cg).unwrap(),
                                )])
                                .await
                                {
                                    Ok(_) => {
                                        namui::log!("Cg copied to clipboard")
                                    }
                                    Err(_) => {
                                        namui::log!("Failed to copy cg to clipboard")
                                    }
                                };
                            })
                        }
                    }
                }
            }
            _ => {}
        });

        ctx.component(graphic_rendering_tree);

        ctx.compose(|ctx| {
            if is_editing_graphic {
                ctx.add(WysiwygTool {
                    graphic_dest_rect: graphic_rendering_rect,
                    original_graphic_size: graphic_wh,
                    graphic_index,
                    graphic: graphic.clone(),
                    dragging: dragging.clone(),
                    wh,
                    on_event: Box::new(|event| on_event(Event::WysiwygTool(event))),
                });
            }
        });

        ctx.done()
    }
}
