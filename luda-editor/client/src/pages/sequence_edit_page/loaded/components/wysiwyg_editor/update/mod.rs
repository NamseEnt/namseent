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
                    namui::log!("MouseUp");
                    if let Some(Dragging::Mover { context }) = self.dragging.as_mut() {
                        context.end_global_xy = global_xy;
                        if let Some(index) = self.editing_image_index {
                            let context = context.clone();
                            namui::event::send(Event::UpdateImages {
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
            });
    }
}
