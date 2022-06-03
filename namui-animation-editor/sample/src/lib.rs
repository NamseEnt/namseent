use namui::prelude::*;
use namui_animation_editor::v1_5::{self, *};
use std::sync::{Arc, RwLock};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub async fn start() {
    let namui_context = namui::init();
    let wh = {
        let managers = namui::managers();
        let screen_size = managers.screen_manager.screen_size();
        Wh {
            width: screen_size.0 as f32,
            height: screen_size.1 as f32,
        }
    };

    namui::start(
        namui_context,
        &mut AnimationEditorExample::new(),
        &Props { wh },
    )
    .await
}

struct AnimationEditorExample {
    animation: Arc<RwLock<animation::Animation>>,
    animation_editor: AnimationEditor,
}

impl AnimationEditorExample {
    fn new() -> Self {
        let animation = Arc::new(RwLock::new(animation::Animation {
            id: namui::nanoid(),
            layers: vec![animation::Layer {
                id: namui::nanoid(),
                name: "New Layer".to_string(),
                image: namui::animation::AnimatableImage::new(),
            }],
        }));
        Self {
            animation_editor: AnimationEditor::new(animation.clone()),
            animation,
        }
    }
}

struct Props {
    wh: Wh<f32>,
}
impl Entity for AnimationEditorExample {
    type Props = Props;

    fn render(&self, props: &Self::Props) -> RenderingTree {
        self.animation_editor.render(&v1_5::Props {
            wh: Wh {
                width: props.wh.width.into(),
                height: props.wh.height.into(),
            },
        })
    }

    fn update(&mut self, event: &dyn std::any::Any) {
        if let Some(event) = event.downcast_ref::<v1_5::Event>() {
            match event {
                Event::AddLayerButtonClicked => {
                    let mut animation = self.animation.write().unwrap();
                    animation.layers.push(animation::Layer {
                        id: namui::nanoid(),
                        name: "New Layer".to_string(),
                        image: namui::animation::AnimatableImage::new(),
                    });
                }
                Event::UpdateLayer(layer) => {
                    let mut animation = self.animation.write().unwrap();
                    let layer = animation
                        .layers
                        .iter_mut()
                        .find(|l| l.id == layer.id)
                        .unwrap();
                    layer.name = layer.name.clone();
                    layer.image = layer.image.clone();
                }
                Event::Error(error) => {
                    namui::log!("error: {}", error);
                }
            }
        }
        self.animation_editor.update(event);
    }
}
