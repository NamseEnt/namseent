use crate::types::Act;

use super::*;

impl LineEditWindow {
    pub fn update(&mut self, event: &dyn std::any::Any) {
        if let Some(event) = event.downcast_ref::<Event>() {
            match event {
                Event::SelectItem {
                    line,
                    layer_id,
                    point_id,
                } => {
                    struct SetLineAction {
                        line: ImageInterpolation,
                        layer_id: String,
                        point_id: String,
                    }
                    impl Act<Animation> for SetLineAction {
                        fn act(
                            &self,
                            state: &Animation,
                        ) -> Result<Animation, Box<dyn std::error::Error>> {
                            let mut animation = state.clone();
                            let layer = animation
                                .layers
                                .iter_mut()
                                .find(|layer| layer.id == self.layer_id)
                                .ok_or("layer not found")?;

                            let (_, line) = layer
                                .image
                                .image_keyframe_graph
                                .get_point_and_line_mut(&self.point_id)
                                .ok_or("point not found")?;

                            *line = self.line;
                            Ok(animation)
                        }
                    }
                    if let Some(ticket) = self.animation_history.try_set_action(SetLineAction {
                        layer_id: layer_id.clone(),
                        point_id: point_id.clone(),
                        line: *line,
                    }) {
                        self.animation_history.act(ticket).unwrap();
                    }
                }
            }
        }
        self.dropdown.update(event);
    }
}
