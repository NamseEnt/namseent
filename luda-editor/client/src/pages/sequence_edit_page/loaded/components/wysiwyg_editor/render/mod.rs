// pub mod cropper;
pub mod mover;
pub mod resizer;

use super::*;
use crate::{
    components::{
        cg_render,
        sequence_player::{calculate_graphic_rect_on_screen, calculate_graphic_wh_on_screen},
    },
    pages::sequence_edit_page::sequence_atom::SEQUENCE_ATOM,
    storage::{get_project_cg_thumbnail_image_url, get_project_image_url},
};
use namui_prebuilt::*;

impl WysiwygEditor {
    pub fn render(&self, props: Props) -> namui::RenderingTree {
        let cut_id = props.cut_id;

        render([
            simple_rect(props.wh, Color::WHITE, 1.px(), Color::TRANSPARENT).attach_event(
                |builder| {
                    let dragging = self.dragging.clone();
                    let screen_graphics = props.screen_graphics.clone();
                    let editing_image_index = self.editing_image_index;
                    builder
                        .on_mouse_move_in(|event: MouseEvent| {
                            namui::event::send(InternalEvent::MouseMoveContainer {
                                global_xy: event.global_xy,
                            });
                        })
                        .on_mouse_down_in(|event: MouseEvent| {
                            if event.button == Some(MouseButton::Left) {
                                namui::event::send(InternalEvent::MouseDownContainer);
                            }
                        })
                        .on_mouse(move |event: MouseEvent| {
                            if event.event_type == MouseEventType::Up {
                                if let Some(Dragging::Mover { mut context }) = dragging {
                                    if let Some(graphic_index) = editing_image_index {
                                        context.end_global_xy = event.global_xy;

                                        let (_, graphic) = screen_graphics
                                            .iter()
                                            .find(|(index, _)| index == &graphic_index)
                                            .unwrap();
                                        let circumscribed =
                                            context.move_circumscribed(graphic.circumscribed());

                                        SEQUENCE_ATOM.update(|sequence| {
                                            sequence.update_cut(
                                                cut_id,
                                                CutUpdateAction::ChangeGraphicCircumscribed {
                                                    graphic_index,
                                                    circumscribed,
                                                },
                                            )
                                        });
                                    }
                                }

                                namui::event::send(InternalEvent::MouseUp);
                            }
                        });
                },
            ),
            clip(
                PathBuilder::new().add_rect(Rect::from_xy_wh(Xy::zero(), props.wh)),
                ClipOp::Intersect,
                self.render_graphic_clip(&props),
            ),
            render_grid_guide(props.wh),
            self.context_menu
                .as_ref()
                .map_or(RenderingTree::Empty, |context_menu| context_menu.render()),
        ])
    }
    fn render_graphic_clip(&self, props: &Props) -> RenderingTree {
        let cut_id = props.cut_id;
        render(
            props
                .screen_graphics
                .clone()
                .into_iter()
                .map(|(graphic_index, graphic)| {
                    let is_editing_graphic = self.editing_image_index == Some(graphic_index);

                    namui::try_render(|| {
                        let url = match &graphic {
                            ScreenGraphic::Image(image) => {
                                get_project_image_url(props.project_id, image.id).unwrap()
                            }
                            ScreenGraphic::Cg(cg) => {
                                get_project_cg_thumbnail_image_url(props.project_id, cg.id).unwrap()
                            }
                        };
                        let namui_image = namui::image::try_load_url(&url)?;
                        let graphic_size = namui_image.size();
                        let circumscribed = graphic.circumscribed();

                        let screen_radius = props.wh.length() / 2;
                        let graphic_radius_px = graphic_size.length() / 2;
                        let radius_px = screen_radius * circumscribed.radius;
                        let graphic_size_on_screen = graphic_size * (radius_px / graphic_radius_px);

                        let center_xy = props.wh.as_xy() * circumscribed.center_xy;

                        let graphic_rendering_rect = {
                            match (is_editing_graphic, self.dragging.as_ref()) {
                                (true, Some(dragging)) => match dragging {
                                    Dragging::Resizer { context } => {
                                        let circumscribed = context.resize(
                                            center_xy,
                                            graphic_size_on_screen,
                                            props.wh,
                                        );
                                        calculate_graphic_rect_on_screen(
                                            graphic_size,
                                            props.wh,
                                            circumscribed,
                                        )
                                    }
                                    // Dragging::Cropper => todo!(),
                                    Dragging::Mover { context } => {
                                        let circumscribed =
                                            context.move_circumscribed(circumscribed);

                                        calculate_graphic_rect_on_screen(
                                            graphic_size,
                                            props.wh,
                                            circumscribed,
                                        )
                                    }
                                },
                                _ => {
                                    let image_left_top_xy =
                                        center_xy - graphic_size_on_screen.as_xy() / 2.0;

                                    Rect::from_xy_wh(image_left_top_xy, graphic_size_on_screen)
                                }
                            }
                        };

                        let wysiwyg_tool = if is_editing_graphic {
                            self.render_wysiwyg_tool(
                                props,
                                graphic_rendering_rect,
                                graphic_size,
                                graphic_index,
                                &graphic,
                            )
                        } else {
                            RenderingTree::Empty
                        };

                        let graphic_rendering_tree = match &graphic {
                            ScreenGraphic::Image(_image) => namui::image(ImageParam {
                                rect: graphic_rendering_rect,
                                source: namui::ImageSource::Image(namui_image.clone()),
                                style: ImageStyle {
                                    fit: ImageFit::Fill,
                                    paint_builder: None,
                                },
                            }),
                            ScreenGraphic::Cg(cg) => try_render(|| {
                                let cg_file = props
                                    .cg_files
                                    .iter()
                                    .find(|cg_file| cg_file.name == cg.name)?;
                                Some(cg_render::render_cg(
                                    cg_render::CgRenderProps {
                                        cg_id: cg.id,
                                        project_id: props.project_id,
                                        rect: graphic_rendering_rect,
                                    },
                                    cg,
                                    cg_file,
                                ))
                            }),
                        };

                        Some(render([
                            graphic_rendering_tree.attach_event(move |builder| {
                                builder.on_mouse_down_in({
                                    let graphic = graphic.clone();
                                    move |event: MouseEvent| {
                                        let graphic = graphic.clone();
                                        event.stop_propagation();
                                        namui::event::send(InternalEvent::SelectImage {
                                            index: graphic_index,
                                        });

                                        if event.button == Some(MouseButton::Right) {
                                            namui::event::send(InternalEvent::OpenContextMenu {
                                                global_xy: event.global_xy,
                                                cut_id,
                                                graphic_index,
                                                graphic_wh: graphic_size,
                                                graphic,
                                            })
                                        }
                                    }
                                });

                                if is_editing_graphic {
                                    let namui_image = namui_image.clone();
                                    let graphic = graphic.clone();
                                    builder.on_key_down(move |event: KeyboardEvent| {
                                        namui::log!("key down: {:?}", event.code);
                                        let graphic = graphic.clone();
                                        if event.code != Code::KeyC
                                            || !namui::keyboard::ctrl_press()
                                        {
                                            return;
                                        }

                                        match graphic {
                                            ScreenGraphic::Image(_) => {
                                                let namui_image = namui_image.clone();
                                                spawn_local(async move {
                                                    let result =
                                                        namui::clipboard::write_image(namui_image)
                                                            .await;
                                                    match result {
                                                        Ok(_) => {
                                                            namui::log!(
                                                                "Image copied to clipboard"
                                                            );
                                                        }
                                                        Err(_) => {
                                                            namui::log!(
                                                                "Failed to copy image to clipboard"
                                                            );
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
                                                            namui::log!(
                                                                "Failed to copy cg to clipboard"
                                                            )
                                                        }
                                                    };
                                                })
                                            }
                                        }
                                    });
                                }
                            }),
                            wysiwyg_tool,
                        ]))
                    })
                }),
        )
    }
    fn render_wysiwyg_tool(
        &self,
        props: &Props,
        graphic_dest_rect: Rect<Px>,
        original_graphic_size: Wh<Px>,
        graphic_index: Uuid,
        graphic: &ScreenGraphic,
    ) -> RenderingTree {
        let cut_id = props.cut_id;
        render([
            self.render_border_with_move_handling(graphic_dest_rect, props.wh),
            resizer::render_resizer(resizer::Props {
                rect: graphic_dest_rect,
                dragging_context: if let Some(Dragging::Resizer { context }) =
                    self.dragging.as_ref()
                {
                    Some(*context)
                } else {
                    None
                },
                on_resize: {
                    Box::new(move |circumscribed| {
                        SEQUENCE_ATOM.update(|sequence| {
                            sequence.update_cut(
                                cut_id,
                                CutUpdateAction::UpdateCircumscribed {
                                    graphic_index,
                                    circumscribed,
                                },
                            )
                        });
                    })
                },
                container_size: props.wh,
                image_size: calculate_graphic_wh_on_screen(
                    original_graphic_size,
                    props.wh,
                    graphic.circumscribed(),
                ),
            }),
            // self.render_cropper(props),
        ])
    }
}

fn render_grid_guide(wh: Wh<Px>) -> RenderingTree {
    let paint = PaintBuilder::new()
        .set_style(PaintStyle::Stroke)
        .set_color(Color::from_f01(0.5, 0.5, 0.5, 0.5))
        .set_stroke_width(5.px());

    let horizontal_third = (0..2).map(|index| {
        let x = wh.width * (index + 1) as f32 / 3.0;
        PathBuilder::new().move_to(x, 0.px()).line_to(x, wh.height)
    });
    let vertical_third = (0..2).map(|index| {
        let y = wh.height * (index + 1) as f32 / 3.0;
        PathBuilder::new().move_to(0.px(), y).line_to(wh.width, y)
    });

    let top = PathBuilder::new()
        .move_to(wh.width / 2.0, 0.px())
        .line_to(wh.width / 2.0, wh.height / 20.0);
    let bottom = PathBuilder::new()
        .move_to(wh.width / 2.0, wh.height)
        .line_to(wh.width / 2.0, wh.height - wh.height * 1.0 / 20.0);
    let left = PathBuilder::new()
        .move_to(0.px(), wh.height / 2.0)
        .line_to(wh.width / 20.0, wh.height / 2.0);
    let right = PathBuilder::new()
        .move_to(wh.width, wh.height / 2.0)
        .line_to(wh.width - wh.width * 1.0 / 20.0, wh.height / 2.0);

    let center_vertical = PathBuilder::new()
        .move_to(wh.width / 2.0 - wh.width / 20.0, wh.height / 2.0)
        .line_to(wh.width / 2.0 + wh.width / 20.0, wh.height / 2.0);
    let center_horizontal = PathBuilder::new()
        .move_to(wh.width / 2.0, wh.height / 2.0 - wh.height / 20.0)
        .line_to(wh.width / 2.0, wh.height / 2.0 + wh.height / 20.0);

    let paths = [top, bottom, left, right, center_vertical, center_horizontal]
        .into_iter()
        .chain(horizontal_third)
        .chain(vertical_third);

    render(paths.map(|path| namui::path(path, paint.clone())))
}
