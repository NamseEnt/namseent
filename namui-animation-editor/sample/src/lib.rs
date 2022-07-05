use namui::{animation::*, prelude::*};
use namui_animation_editor::{self, *};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub async fn start() {
    let namui_context = namui::init().await;

    let wh = namui::screen::size();

    namui::start(
        namui_context,
        &mut AnimationEditorExample::new(),
        &Props { wh },
    )
    .await
}

struct AnimationEditorExample {
    animation: animation::Animation,
    animation_editor: AnimationEditor,
}

impl AnimationEditorExample {
    fn new() -> Self {
        let mut image = namui::animation::AnimatableImage::new();

        image.image_source_url =
            Some(Url::parse("bundle:img/%EB%86%80%EB%9E%8C%EB%8C%80.png").unwrap());

        image.image_keyframe_graph.put(
            KeyframePoint::new(
                0.0.ms(),
                ImageKeyframe {
                    x: 500.0.px(),
                    y: 000.0.px(),
                    width_percent: 50.0.percent(),
                    height_percent: 50.0.percent(),
                    rotation_angle: 0.0.deg(),
                    opacity: 1.0.into(),
                },
            ),
            animation::ImageInterpolation::AllLinear,
        );

        let animation = animation::Animation {
            id: namui::nanoid(),
            layers: vec![animation::Layer {
                id: namui::nanoid(),
                name: "New Layer".to_string(),
                image,
            }],
        };
        Self {
            animation_editor: AnimationEditor::new(&animation),
            animation,
        }
    }
}

struct Props {
    wh: Wh<Px>,
}
impl Entity for AnimationEditorExample {
    type Props = Props;

    fn render(&self, props: &Self::Props) -> RenderingTree {
        self.animation_editor.render(namui_animation_editor::Props {
            wh: Wh {
                width: props.wh.width.into(),
                height: props.wh.height.into(),
            },
        })
    }

    fn update(&mut self, event: &dyn std::any::Any) {
        if let Some(event) = event.downcast_ref::<namui_animation_editor::Event>() {
            match event {
                Event::Error(error) => {
                    namui::log!("error: {}", error);
                }
                Event::AnimationUpdated(animation) => {
                    self.animation = (**animation).clone();
                }
            }
        }
        self.animation_editor.update(event);
    }
}
