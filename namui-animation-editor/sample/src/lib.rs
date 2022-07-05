use namui::{animation::KeyframePoint, prelude::*, types::*};
use namui_animation_editor::{self, *};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub async fn start() {
    let namui_context = namui::init().await;

    let wh = {
        let screen_size = namui::screen::size();
        Wh {
            width: screen_size.width as f32,
            height: screen_size.height as f32,
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
    animation: animation::Animation,
    animation_editor: AnimationEditor,
}

impl AnimationEditorExample {
    fn new() -> Self {
        let mut image = namui::animation::AnimatableImage::new();

        image.image_source_url =
            Some(Url::parse("bundle:img/%EB%86%80%EB%9E%8C%EB%8C%80.png").unwrap());
        image.x.put(
            // KeyframePoint::<Px>::new(Time::Ms(0.0), Px::from(0.0)),
            KeyframePoint::<Px>::new(Time::Ms(0.0), Px::from(500.0)),
            animation::KeyframeLine::Linear,
        );
        image.y.put(
            // KeyframePoint::<Px>::new(Time::Ms(0.0), Px::from(0.0)),
            KeyframePoint::<Px>::new(Time::Ms(0.0), Px::from(0.0)),
            animation::KeyframeLine::Linear,
        );
        image.width_percent.put(
            KeyframePoint::<Percent>::new(Time::Ms(0.0), Percent::from_percent(50.0_f32)),
            animation::KeyframeLine::Linear,
        );
        image.height_percent.put(
            KeyframePoint::<Percent>::new(Time::Ms(0.0), Percent::from_percent(50.0_f32)),
            animation::KeyframeLine::Linear,
        );
        image.rotation_angle.put(
            // KeyframePoint::<Angle>::new(Time::Ms(0.0), Angle::from(30.0)),
            KeyframePoint::<Angle>::new(Time::Ms(0.0), Angle::Degree(0.0)),
            animation::KeyframeLine::Linear,
        );
        image.opacity.put(
            KeyframePoint::<OneZero>::new(Time::Ms(0.0), 1.0.into()),
            animation::KeyframeLine::Linear,
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
