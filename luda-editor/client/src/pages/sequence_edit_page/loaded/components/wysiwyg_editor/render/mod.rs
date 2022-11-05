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
        render([
            simple_rect(props.wh, Color::WHITE, 1.px(), Color::TRANSPARENT).attach_event(
                |builder| {
                    builder
                        .on_mouse_move_in(|event| {
                            namui::event::send(InternalEvent::MouseMoveContainer {
                                global_xy: event.global_xy,
                            });
                        })
                        .on_mouse_down_in(|_event| {
                            namui::event::send(InternalEvent::MouseDownContainer);
                        });
                },
            ),
            clip(
                PathBuilder::new().add_rect(Rect::from_xy_wh(Xy::zero(), props.wh)),
                ClipOp::Intersect,
                self.render_image_clip(&props),
            ),
        ])
    }
    fn render_image_clip(&self, props: &Props) -> RenderingTree {
        render(
            self.screen_images
                .iter()
                .enumerate()
                .map(|(image_index, image)| {
                    let is_editing_image = self.editing_image_index == Some(image_index);

                    namui::try_render(|| {
                        let image = image.as_ref()?;

                        let url = get_project_image_url(self.project_id, image.id).unwrap();
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
                        namui::event::send(InternalEvent::ResizeImage {
                            index: image_index,
                            circumscribed,
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
