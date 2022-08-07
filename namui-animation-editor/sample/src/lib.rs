use namui::{animation::*, prelude::*};
use namui_animation_editor::{self, *};

pub async fn main() {
    let namui_context = namui::init().await;

    namui::start(namui_context, &mut AnimationEditorExample::new(), &Props {}).await
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
                    matrix: namui::Matrix3x3::identity(),
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

struct Props {}
impl Entity for AnimationEditorExample {
    type Props = Props;

    fn render(&self, _props: &Self::Props) -> RenderingTree {
        let wh = namui::screen::size();
        self.animation_editor
            .render(namui_animation_editor::Props { wh })
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
