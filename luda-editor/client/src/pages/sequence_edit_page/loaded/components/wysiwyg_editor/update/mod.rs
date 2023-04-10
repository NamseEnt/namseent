use super::{render::resizer, *};

impl WysiwygEditor {
    pub fn update(&mut self, event: &namui::Event) {
        event
            .is::<resizer::Event>(|event| match event {
                resizer::Event::UpdateDraggingContext(context) => {
                    if matches!(self.dragging, Some(Dragging::Resizer { .. })) {
                        self.dragging =
                            context.and_then(|context| Some(Dragging::Resizer { context }));
                    }
                }
                &resizer::Event::StartDraggingContext(context) => {
                    if self.dragging.is_none() {
                        self.dragging = Some(Dragging::Resizer { context });
                    }
                }
            })
            .is::<InternalEvent>(|event| match event {
                &InternalEvent::SelectImage { index } => {
                    self.editing_image_index = Some(index);
                }
                &InternalEvent::ImageMoveStart {
                    start_global_xy,
                    end_global_xy,
                    container_wh,
                } => {
                    if self.dragging.is_none() {
                        self.dragging = Some(Dragging::Mover {
                            context: render::mover::MoverDraggingContext {
                                start_global_xy,
                                end_global_xy,
                                container_wh,
                            },
                        });
                    }
                }
                &InternalEvent::MouseMoveContainer { global_xy } => {
                    if let Some(Dragging::Mover { context }) = self.dragging.as_mut() {
                        context.end_global_xy = global_xy;
                    }
                }
                InternalEvent::MouseDownContainer => {
                    self.editing_image_index = None;
                }
                &InternalEvent::MouseUp { global_xy, cut_id } => {
                    if let Some(Dragging::Mover { context }) = self.dragging.as_mut() {
                        context.end_global_xy = global_xy;
                        if let Some(index) = self.editing_image_index {
                            let context = context.clone();
                            namui::event::send(Event::UpdateCutImages {
                                cut_id,
                                callback: Box::new({
                                    move |images| {
                                        let circumscribed = images[index].circumscribed;
                                        let moved_circumscribed =
                                            context.move_circumscribed(circumscribed);
                                        images[index].circumscribed = moved_circumscribed;
                                    }
                                }),
                            });
                        }
                        self.dragging = None;
                    }
                }
                &InternalEvent::OpenContextMenu {
                    global_xy,
                    cut_id,
                    image_index,
                    image_wh,
                    image,
                } => {
                    let image_width_per_height_ratio = image_wh.width / image_wh.height;
                    let screen_width_per_height_ratio = 4.0 / 3.0;

                    let fit_items = [
                        context_menu::Item::new_button("Fit - contain", move || {
                            namui::event::send(Event::UpdateCutImages {
                                cut_id,
                                callback: Box::new({
                                    move |images| {
                                        let image = &mut images[image_index];

                                        image.circumscribed.center_xy = Xy::single(50.percent());

                                        let radius = if image_width_per_height_ratio
                                            > screen_width_per_height_ratio
                                        {
                                            let width = 4.0 / 5.0;
                                            let height = width / image_width_per_height_ratio;
                                            Xy::new(width, height).length()
                                        } else {
                                            let height = 3.0 / 5.0;
                                            let width = height * image_width_per_height_ratio;
                                            Xy::new(width, height).length()
                                        };

                                        image.circumscribed.radius = Percent::from(radius);
                                    }
                                }),
                            });
                        }),
                        context_menu::Item::new_button("Fit - cover", move || {
                            namui::event::send(Event::UpdateCutImages {
                                cut_id,
                                callback: Box::new({
                                    move |images| {
                                        let image = &mut images[image_index];

                                        image.circumscribed.center_xy = Xy::single(50.percent());

                                        let radius = if image_width_per_height_ratio
                                            > screen_width_per_height_ratio
                                        {
                                            let height = 3.0 / 5.0;
                                            let width = height * image_width_per_height_ratio;
                                            Xy::new(width, height).length()
                                        } else {
                                            let width = 4.0 / 5.0;
                                            let height = width / image_width_per_height_ratio;
                                            Xy::new(width, height).length()
                                        };

                                        image.circumscribed.radius = Percent::from(radius);
                                    }
                                }),
                            });
                        }),
                    ];

                    let spread_as_background = if image_index != 0 {
                        [].to_vec()
                    } else {
                        [context_menu::Item::new_button(
                            "Spread as background",
                            move || {
                                namui::event::send(Event::UpdateSequenceImages {
                                    callback: Box::new({
                                        move |images| {
                                            if images.len() == 0 {
                                                images.push(image);
                                                return;
                                            }

                                            let first = images.first_mut().unwrap();
                                            if first.id == image.id {
                                                first.circumscribed = image.circumscribed;
                                                return;
                                            }

                                            images.insert(0, image);
                                        }
                                    }),
                                });
                            },
                        )]
                        .to_vec()
                    };

                    self.context_menu = Some(context_menu::ContextMenu::new(
                        global_xy,
                        fit_items.into_iter().chain(spread_as_background),
                    ));
                }
            })
            .is::<context_menu::Event>(|event| match event {
                context_menu::Event::Close => {
                    self.context_menu = None;
                }
            });

        self.context_menu.as_mut().map(|context_menu| {
            context_menu.update(event);
        });
    }
}
