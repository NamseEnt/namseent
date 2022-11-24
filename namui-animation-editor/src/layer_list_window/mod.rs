use crate::types::{Act, AnimationHistory};
use namui::{
    animation::{Animation, Layer},
    prelude::*,
};
use namui_prebuilt::{table::*, *};
mod body;
mod header;

pub struct LayerListWindow {
    header: header::Header,
    body: body::Body,
    animation_history: AnimationHistory,
    pub selected_layer_id: Option<Uuid>,
}

pub struct Props<'a> {
    pub wh: Wh<Px>,
    pub layers: &'a [Layer],
}

pub enum Event {
    LayerSelected(Uuid),
    AddLayerButtonClicked,
}

impl LayerListWindow {
    pub fn new(animation_history: AnimationHistory) -> Self {
        Self {
            header: header::Header::new(),
            body: body::Body::new(),
            animation_history,
            selected_layer_id: None,
        }
    }
    pub fn update(&mut self, event: &namui::Event) {
        event.is::<Event>(|event| match event {
            Event::AddLayerButtonClicked => {
                struct AddLayerAction;
                impl Act<Animation> for AddLayerAction {
                    fn act(
                        &self,
                        state: &Animation,
                    ) -> Result<Animation, Box<dyn std::error::Error>> {
                        let mut animation = state.clone();
                        animation.layers.push(animation::Layer {
                            id: namui::uuid(),
                            name: "New Layer".to_string(),
                            image: namui::animation::AnimatableImage::new(),
                        });
                        Ok(animation)
                    }
                }
                if let Some(action_ticket) = self.animation_history.try_set_action(AddLayerAction) {
                    self.animation_history.act(action_ticket).unwrap();
                }
            }
            Event::LayerSelected(layer_id) => {
                self.selected_layer_id = Some(layer_id.clone());
            }
        });
        self.header.update(event);
        self.body.update(event);
    }
    pub fn render(&self, props: Props) -> RenderingTree {
        render![
            simple_rect(props.wh, Color::BLACK, px(1.0), Color::WHITE),
            vertical([
                fixed(px(20.0), |wh| { self.header.render(header::Props { wh }) }),
                ratio(1.0, |wh| {
                    self.body.render(body::Props {
                        wh: wh,
                        layers: props.layers,
                        selected_layer_id: self.selected_layer_id.clone(),
                    })
                }),
            ])(props.wh)
        ]
    }
}
