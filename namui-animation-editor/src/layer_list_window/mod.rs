use crate::types::{Act, AnimationHistory};
use namui::{
    animation::{Animation, Layer},
    prelude::*,
};
use namui_prebuilt::*;
mod body;
mod header;

pub(crate) struct LayerListWindow {
    header: header::Header,
    body: body::Body,
    animation_history: AnimationHistory,
}

pub(crate) struct Props<'a> {
    pub layers: &'a [Layer],
    pub selected_layer_id: Option<String>,
}

pub(crate) enum Event {
    LayerSelected(String),
    AddLayerButtonClicked,
}

impl LayerListWindow {
    pub(crate) fn new(animation_history: AnimationHistory) -> Self {
        Self {
            header: header::Header::new(),
            body: body::Body::new(),
            animation_history,
        }
    }
    pub(crate) fn update(&mut self, event: &dyn std::any::Any) {
        if let Some(event) = event.downcast_ref::<Event>() {
            match event {
                Event::AddLayerButtonClicked => {
                    struct AddLayerAction;
                    impl Act<Animation> for AddLayerAction {
                        fn act(
                            &self,
                            state: &Animation,
                        ) -> Result<Animation, Box<dyn std::error::Error>> {
                            let mut animation = state.clone();
                            animation.layers.push(animation::Layer {
                                id: namui::nanoid(),
                                name: "New Layer".to_string(),
                                image: namui::animation::AnimatableImage::new(),
                            });
                            Ok(animation)
                        }
                    }
                    if let Some(action_ticket) =
                        self.animation_history.try_set_action(AddLayerAction)
                    {
                        self.animation_history.act(action_ticket).unwrap();
                    }
                }
                _ => {}
            }
        }
        self.header.update(event);
        self.body.update(event);
    }
}

impl table::CellRender<Props<'_>> for LayerListWindow {
    fn render(&self, wh: Wh<f32>, props: Props<'_>) -> RenderingTree {
        render![
            simple_rect(wh, Color::BLACK, 1.0, Color::WHITE),
            vertical![
                fixed!(20.0, &self.header, header::Props()),
                ratio!(
                    1.0,
                    &self.body,
                    body::Props {
                        layers: props.layers,
                        selected_layer_id: props.selected_layer_id,
                    }
                ),
            ](Wh {
                width: wh.width.into(),
                height: wh.height.into(),
            })
        ]
    }
}
