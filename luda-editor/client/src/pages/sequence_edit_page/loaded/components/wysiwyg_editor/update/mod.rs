use super::{render::resizer, *};

impl WysiwygEditor {
    pub fn update(&mut self, event: &dyn std::any::Any) {
        if let Some(event) = event.downcast_ref::<Event>() {
            match event {
                Event::Resize {
                    circumscribed,
                    image_clip_address,
                    layer_index,
                } => {
                    self.editor_history_system
                        .mutate_image_clip(image_clip_address, |clip| {
                            clip.images.update(*layer_index, |layer| {
                                layer.circumscribed = *circumscribed;
                            })
                        });
                }
            }
        } else if let Some(event) = event.downcast_ref::<resizer::Event>() {
            match event {
                resizer::Event::UpdateDraggingContext(context) => {
                    self.dragging = context.and_then(|context| Some(Dragging::Resizer { context }));
                }
            }
        }
    }
}
