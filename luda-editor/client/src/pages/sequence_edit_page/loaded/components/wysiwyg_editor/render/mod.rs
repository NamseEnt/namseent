// pub mod cropper;
pub mod mover;
pub mod resizer;

use super::*;
use crate::{
    components::sequence_player::{calculate_image_rect_on_screen, calculate_image_wh_on_screen},
    storage::get_project_image_url,
};
use namui_prebuilt::*;

impl WysiwygEditor {
    pub fn render(&self, props: Props) -> namui::RenderingTree {
        let cut_id = props.cut_id;
        render([
            simple_rect(props.wh, Color::WHITE, 1.px(), Color::TRANSPARENT).attach_event(
                |builder| {
                    builder
                        .on_mouse_move_in(|event| {
                            namui::event::send(InternalEvent::MouseMoveContainer {
                                global_xy: event.global_xy,
                            });
                        })
                        .on_mouse_down_in(|event| {
                            if event.button == Some(MouseButton::Left) {
                                namui::event::send(InternalEvent::MouseDownContainer);
                            }
                        })
                        .on_mouse(move |event| {
                            namui::log!("event: {:?}", event.event_type);
                            if event.event_type == MouseEventType::Up {
                                namui::event::send(InternalEvent::MouseUp {
                                    global_xy: event.global_xy,
                                    cut_id,
                                });
                            }
                        });
                },
            ),
            clip(
                PathBuilder::new().add_rect(Rect::from_xy_wh(Xy::zero(), props.wh)),
                ClipOp::Intersect,
                self.render_image_clip(&props),
            ),
            render_grid_guide(props.wh),
            self.context_menu
                .as_ref()
                .map_or(RenderingTree::Empty, |context_menu| context_menu.render()),
        ])
    }
    fn render_image_clip(&self, props: &Props) -> RenderingTree {
        let cut_id = props.cut_id;
        render(
            props
                .screen_images
                .iter()
                .enumerate()
                .map(|(image_index, image)| {
                    let is_editing_image = self.editing_image_index == Some(image_index);

                    namui::try_render(|| {
                        let url = get_project_image_url(props.project_id, image.id).unwrap();
                        let namui_image = namui::image::try_load_url(&url)?;
                        let image_size = namui_image.size();

                        let screen_radius = props.wh.length() / 2;
                        let image_radius_px = image_size.length() / 2;
                        let radius_px = screen_radius * image.circumscribed.radius;
                        let image_size_on_screen = image_size * (radius_px / image_radius_px);

                        let center_xy = props.wh.as_xy() * image.circumscribed.center_xy;

                        let image_rendering_rect = {
                            match (is_editing_image, self.dragging.as_ref()) {
                                (true, Some(dragging)) => match dragging {
                                    Dragging::Resizer { context } => {
                                        let circumscribed = context.resize(
                                            center_xy,
                                            image_size_on_screen,
                                            props.wh,
                                        );
                                        calculate_image_rect_on_screen(
                                            image_size,
                                            props.wh,
                                            circumscribed,
                                        )
                                    }
                                    // Dragging::Cropper => todo!(),
                                    Dragging::Mover { context } => {
                                        let circumscribed =
                                            context.move_circumscribed(image.circumscribed);

                                        calculate_image_rect_on_screen(
                                            image_size,
                                            props.wh,
                                            circumscribed,
                                        )
                                    }
                                },
                                _ => {
                                    let image_left_top_xy =
                                        center_xy - image_size_on_screen.as_xy() / 2.0;

                                    Rect::from_xy_wh(image_left_top_xy, image_size_on_screen)
                                }
                            }
                        };

                        let wysiwyg_tool = if is_editing_image {
                            self.render_wysiwyg_tool(
                                props,
                                image_rendering_rect,
                                image_size,
                                image_index,
                                image,
                            )
                        } else {
                            RenderingTree::Empty
                        };

                        Some(render([
                            namui::image(ImageParam {
                                rect: image_rendering_rect,
                                source: namui::ImageSource::Image(namui_image),
                                style: ImageStyle {
                                    fit: ImageFit::Fill,
                                    paint_builder: None,
                                },
                            })
                            .attach_event(move |builder| {
                                builder.on_mouse_down_in(move |event| {
                                    event.stop_propagation();
                                    namui::event::send(InternalEvent::SelectImage {
                                        index: image_index,
                                    });

                                    if event.button == Some(MouseButton::Right) {
                                        namui::event::send(InternalEvent::OpenContextMenu {
                                            global_xy: event.global_xy,
                                            cut_id,
                                            image_index,
                                            image_wh: image_size,
                                        })
                                    }
                                });
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
        image_dest_rect: Rect<Px>,
        original_image_size: Wh<Px>,
        image_index: usize,
        image: &ScreenImage,
    ) -> RenderingTree {
        let cut_id = props.cut_id;
        render([
            self.render_border_with_move_handling(image_dest_rect, props.wh),
            resizer::render_resizer(resizer::Props {
                rect: image_dest_rect,
                dragging_context: if let Some(Dragging::Resizer { context }) =
                    self.dragging.as_ref()
                {
                    Some(*context)
                } else {
                    None
                },
                on_resize: {
                    Box::new(move |circumscribed| {
                        namui::event::send(Event::UpdateImages {
                            cut_id,
                            callback: Box::new(move |images| {
                                images[image_index].circumscribed = circumscribed;
                            }),
                        });
                    })
                },
                container_size: props.wh,
                image_size: calculate_image_wh_on_screen(
                    original_image_size,
                    props.wh,
                    image.circumscribed,
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
