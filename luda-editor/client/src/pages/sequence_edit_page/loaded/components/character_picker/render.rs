use super::*;
use crate::color;
use namui_prebuilt::*;

impl CharacterPicker {
    pub fn render(&self, props: Props) -> namui::RenderingTree {
        render([self.render_background(props)])
    }

    fn render_background(&self, props: Props) -> namui::RenderingTree {
        let background = simple_rect(props.wh, color::STROKE_NORMAL, 1.px(), color::BACKGROUND);
        background.attach_event(|builder| {
            builder
                .on_mouse_down_out(|_| namui::event::send(Event::MouseDownOutsideCharacterPicker));
        })
    }
}
