use super::{render::resizer, *};

impl WysiwygEditor {
    pub fn update(&mut self, event: &dyn std::any::Any) {
        if let Some(event) = event.downcast_ref::<resizer::Event>() {
            match event {
                resizer::Event::UpdateDraggingContext(context) => {
                    self.dragging = context.and_then(|context| Some(Dragging::Resizer { context }));
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
            }
        }
    }
}
