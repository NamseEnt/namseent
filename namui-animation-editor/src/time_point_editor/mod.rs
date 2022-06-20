use namui::{prelude::*, types::Time};
use namui_prebuilt::{table::*, *};
mod wysiwyg_window;

pub struct TimePointEditor {
    animation: crate::ReadOnlyLock<animation::Animation>,
    wysiwyg_window: wysiwyg_window::WysiwygWindow,
}

pub struct Props {
    pub wh: Wh<f32>,
}

pub(crate) enum Event {}

impl TimePointEditor {
    pub fn new(animation: crate::ReadOnlyLock<animation::Animation>) -> Self {
        Self {
            wysiwyg_window: wysiwyg_window::WysiwygWindow::new(animation.clone()),
            animation,
        }
    }
    pub fn update(&mut self, event: &dyn std::any::Any) {
        self.wysiwyg_window.update(event);
    }
    pub fn render(&self, props: Props) -> namui::RenderingTree {
        vertical([
            ratio(
                8.0,
                horizontal([
                    ratio(2.0, |wh| {
                        simple_rect(wh, Color::BLACK, 1.0, Color::grayscale_f01(0.5))
                    }),
                    ratio(8.0, |wh| {
                        self.wysiwyg_window.render(wysiwyg_window::Props { wh })
                    }),
                ]),
            ),
            ratio(2.0, |wh| {
                simple_rect(wh, Color::BLACK, 1.0, Color::grayscale_f01(0.5))
            }),
        ])(Wh {
            width: props.wh.width.into(),
            height: props.wh.height.into(),
        })
    }
}
