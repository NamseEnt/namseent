use super::{render::resizer, *};

impl WysiwygEditor {
    pub fn update(&mut self, event: &dyn std::any::Any) {
        if let Some(event) = event.downcast_ref::<resizer::Event>() {
            match event {
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
            }
        } else if let Some(event) = event.downcast_ref::<InternalEvent>() {
            match event {
                &InternalEvent::SelectImage { index } => {
                    self.editing_image_index = Some(index);
                }
                &InternalEvent::ResizeImage {
                    index,
                    circumscribed,
                } => {
                    self.screen_images[index].circumscribed = circumscribed;
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
            }
        } else if let Some(NamuiEvent::MouseUp(event)) = event.downcast_ref::<NamuiEvent>() {
            if let Some(Dragging::Mover { context }) = self.dragging.as_mut() {
                context.end_global_xy = event.xy;
                if let Some(index) = self.editing_image_index {
                    let circumscribed = self.screen_images[index].circumscribed;
                    let moved_circumscribed = context.move_circumscribed(circumscribed);
                    self.screen_images[index].circumscribed = moved_circumscribed;
                }
                self.dragging = None;
            }
        }
    }
}
